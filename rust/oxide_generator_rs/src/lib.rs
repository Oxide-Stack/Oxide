#![doc = include_str!("../README.md")]

// Proc-macro entrypoint and stable macro surface.
//
// Why: keep macro names and signatures stable for downstream crates, while
// allowing internal parsing/codegen modules to evolve safely.
use proc_macro::TokenStream;
use syn::Item;
use syn::parse_macro_input;
#[cfg(test)]
use std::sync::{Mutex, OnceLock};

mod derive;
mod meta;
mod reducer;
mod routes;
#[cfg(feature = "isolated-channels")]
mod isolated_channels;
#[cfg(test)]
pub(crate) static TEST_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

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

#[proc_macro_attribute]
/// Generates navigation artifacts for an application routes module.
///
/// Apply this macro to an inline `routes` module. The macro discovers `Route`
/// implementations, emits route metadata, and generates navigation bridge glue.
///
/// # Errors
/// Emits a compile error if route files cannot be scanned or metadata cannot be emitted.
pub fn routes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Mod(item_mod) => match routes::expand_routes_module(item_mod) {
            Ok(ts) => ts.into(),
            Err(e) => e.to_compile_error().into(),
        },
        other => syn::Error::new_spanned(other, "#[routes] can only be applied to a module")
            .to_compile_error()
            .into(),
    }
}

#[cfg(feature = "isolated-channels")]
#[proc_macro_attribute]
/// Generates glue for an Oxide isolated event channel or duplex channel.
///
/// This macro is applied to an `impl OxideEventChannel for <Type>` block (events)
/// or an `impl OxideEventDuplexChannel for <Type>` block (duplex).
///
/// The generated code follows the locked `OxideIsolatedChannels` specification:
/// predictable send helpers, explicit initialization boundaries, and FRB-friendly
/// stream endpoints without string-based routing.
pub fn oxide_event_channel(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as isolated_channels::OxideEventChannelArgs);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Impl(item_impl) => match isolated_channels::expand_oxide_event_channel(args, item_impl) {
            Ok(ts) => ts.into(),
            Err(e) => e.to_compile_error().into(),
        },
        other => syn::Error::new_spanned(
            other,
            "#[oxide_event_channel] can only be applied to an impl block",
        )
        .to_compile_error()
        .into(),
    }
}

#[cfg(feature = "isolated-channels")]
#[proc_macro_attribute]
/// Generates glue for an Oxide callback interface (Rust → Dart → Rust).
///
/// The macro enforces deterministic request/response binding by variant name:
/// every `Request::<Variant>` must have a matching `Response::<Variant>`.
pub fn oxide_callback(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as isolated_channels::OxideCallbackArgs);
    let input = parse_macro_input!(item as Item);
    match input {
        Item::Impl(item_impl) => match isolated_channels::expand_oxide_callback(args, item_impl) {
            Ok(ts) => ts.into(),
            Err(e) => e.to_compile_error().into(),
        },
        other => syn::Error::new_spanned(
            other,
            "#[oxide_callback] can only be applied to an impl block",
        )
        .to_compile_error()
        .into(),
    }
}
