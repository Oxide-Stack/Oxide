#![doc = include_str!("../README.md")]

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
/// # Errors
/// Emits a compile error if applied to any other item.
pub fn state(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Struct(item_struct) => derive::expand_state_struct(item_struct).into(),
        Item::Enum(item_enum) => derive::expand_state_enum(item_enum).into(),
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
