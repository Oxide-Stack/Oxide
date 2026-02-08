#![doc = include_str!("../README.md")]

// Proc-macro entrypoint and stable macro surface.
//
// Why: keep macro names and signatures stable for downstream crates, while
// allowing internal parsing/codegen modules to evolve safely.
use proc_macro::TokenStream;
use syn::Item;
use syn::parse_macro_input;

mod derive;
mod meta;
mod reducer;

#[proc_macro_attribute]
/// Marks a struct or enum as an Oxide state type.
///
/// The annotated item must be a `struct` or `enum`.
///
/// This macro primarily does two things:
///
/// 1. Ensures a baseline set of derives exist (`Debug`, `Clone`, `PartialEq`, `Eq`).
/// 2. Injects metadata into doc strings (`oxide:meta:<json>`) so downstream tools
///    can discover the state shape without type-checking.
///
/// With the `state-persistence` feature enabled, the macro also injects serde
/// derives using `oxide_core::serde` so consumer crates do not need to depend on
/// `serde` directly.
///
/// # Errors
/// Emits a compile error if applied to any other item.
pub fn state(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as derive::StateArgs);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Struct(item_struct) => derive::expand_state_struct(args, item_struct).into(),
        Item::Enum(item_enum) => derive::expand_state_enum(args, item_enum).into(),
        other => syn::Error::new_spanned(other, "#[state] can only be applied to a struct or enum")
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_attribute]
/// Marks an enum as an Oxide actions type.
///
/// The annotated item must be an `enum`.
///
/// This performs the same derive and metadata injection as [`state`], but is
/// restricted to enums (actions are modeled as closed sets).
///
/// # Errors
/// Emits a compile error if applied to any other item.
pub fn actions(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Enum(item_enum) => derive::expand_actions_enum(item_enum).into(),
        other => syn::Error::new_spanned(other, "#[actions] can only be applied to an enum")
            .to_compile_error()
            .into(),
    }
}

#[proc_macro_attribute]
/// Generates an Oxide engine and bindings for a reducer implementation.
///
/// The annotated item must be an `impl oxide_core::Reducer for <Type>` block.
///
/// See the crate README for the full argument mini-language, including the
/// required `engine`, `snapshot`, and `initial` keys.
///
/// # Errors
/// Emits a compile error if arguments are missing or the reducer impl is invalid.
pub fn reducer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as reducer::ReducerArgs);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Impl(item_impl) => reducer::expand_reducer_impl(args, item_impl).into(),
        other => syn::Error::new_spanned(
            other,
            "#[reducer(...)] can only be applied to an `impl oxide_core::Reducer for <Type>` block",
        )
        .to_compile_error()
        .into(),
    }
}
