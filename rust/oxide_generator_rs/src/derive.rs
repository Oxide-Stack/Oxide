//! Expansion helpers for `#[state]` and `#[actions]`.
//!
//! These macros are intentionally lightweight:
//!
//! - They ensure common derives exist for ergonomic use in Rust and across FFI boundaries.
//! - They inject structured metadata as doc strings so other code generators can inspect
//!   the state/action shapes without needing a Rust type-checker.
//!
//! The metadata format is implemented in [`crate::meta`].

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Fields, Ident, ItemEnum, ItemStruct, LitBool, Token};

use crate::meta::{
    ActionsMeta, StateMeta, collect_doc_lines, enum_variants, push_meta_doc, struct_fields,
};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct StateArgs {
    pub sliced: bool,
}

impl Parse for StateArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let mut sliced: Option<bool> = None;
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let value: LitBool = input.parse()?;
            match key.to_string().as_str() {
                "sliced" => sliced = Some(value.value),
                other => {
                    return Err(syn::Error::new_spanned(
                        key,
                        format!("unknown #[state] argument `{other}`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            sliced: sliced.unwrap_or(false),
        })
    }
}

fn ensure_required_derives(attrs: &mut Vec<Attribute>) {
    // These are the baseline derives Oxide expects for state and actions.
    let mut required: Vec<syn::Path> = vec![
        syn::parse_quote!(Debug),
        syn::parse_quote!(Clone),
        syn::parse_quote!(PartialEq),
        syn::parse_quote!(Eq),
    ];

    if cfg!(feature = "state-persistence") {
        // When persistence is enabled, we inject serde derives using oxide_core's re-export
        // so downstream crates do not need to depend on serde directly.
        required.push(syn::parse_quote!(::oxide_core::serde::Serialize));
        required.push(syn::parse_quote!(::oxide_core::serde::Deserialize));
    }

    ensure_derive(attrs, &required);
}

fn ensure_derive(attrs: &mut Vec<Attribute>, required: &[syn::Path]) {
    // We either update the first existing `#[derive(...)]` attribute, or insert a new one.
    let mut existing: Vec<syn::Path> = Vec::new();
    let mut derive_attr_idx: Option<usize> = None;

    for (idx, attr) in attrs.iter().enumerate() {
        if !attr.path().is_ident("derive") {
            continue;
        }
        derive_attr_idx = Some(idx);
        let parsed: syn::punctuated::Punctuated<syn::Path, Token![,]> =
            match attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                Ok(v) => v,
                Err(_) => continue,
            };
        existing.extend(parsed.into_iter());
        break;
    }

    for req in required {
        // Compare by last path segment so `Serialize` and `oxide_core::serde::Serialize`
        // are treated as the same derive.
        let req_last = req.segments.last().map(|s| s.ident.to_string());
        let already = existing.iter().any(|p| {
            p.segments
                .last()
                .map(|s| s.ident.to_string())
                .is_some_and(|x| Some(x) == req_last)
        });
        if !already {
            existing.push(req.clone());
        }
    }

    let new_attr: Attribute = syn::parse_quote!(#[derive(#(#existing),*)]);
    match derive_attr_idx {
        Some(idx) => attrs[idx] = new_attr,
        None => attrs.insert(0, new_attr),
    }
}

fn has_serde_crate_attr(attrs: &[Attribute]) -> bool {
    // We only inject `#[serde(crate = "oxide_core::serde")]` if the user hasn't already
    // configured serde's crate path.
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let syn::Meta::List(list) = &attr.meta else {
            continue;
        };
        let parsed: syn::punctuated::Punctuated<syn::Meta, Token![,]> =
            match list.parse_args_with(syn::punctuated::Punctuated::parse_terminated) {
                Ok(v) => v,
                Err(_) => continue,
            };
        for meta in parsed {
            if meta.path().is_ident("crate") {
                return true;
            }
        }
    }
    false
}

fn ensure_serde_crate_attr(attrs: &mut Vec<Attribute>) {
    if has_serde_crate_attr(attrs) {
        return;
    }
    attrs.push(syn::parse_quote!(#[serde(crate = "oxide_core::serde")]));
}

fn slice_variant_ident(field: &Ident) -> Ident {
    let raw = field.to_string();
    let raw = raw.trim_start_matches('_');
    let mut out = String::new();
    for part in raw.split('_') {
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        let Some(first) = chars.next() else {
            continue;
        };
        out.push_str(&first.to_uppercase().to_string());
        out.push_str(chars.as_str());
    }
    if out.is_empty() {
        out = raw.to_string();
    }
    quote::format_ident!("{out}")
}

pub(crate) fn expand_state_struct(
    args: StateArgs,
    mut item: ItemStruct,
) -> proc_macro2::TokenStream {
    if args.sliced {
        let Fields::Named(named) = &item.fields else {
            return syn::Error::new_spanned(
                &item,
                "#[state(sliced = true)] is only supported on structs with named fields",
            )
            .to_compile_error();
        };

        // Why: The previously-generated enum name was always `StateSlice`, which
        // collides when multiple sliced state structs exist in the same module.
        //
        // How: Make the generated enum name state-specific, e.g. `MyStateSlice`.
        let slice_enum_ident = quote::format_ident!("{}Slice", item.ident);

        let slice_variants: Vec<Ident> = named
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .map(slice_variant_ident)
            .collect();

        let slice_checks: Vec<proc_macro2::TokenStream> = named
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .map(|field_ident| {
                let variant = slice_variant_ident(field_ident);
                quote! {
                    if before.#field_ident != after.#field_ident {
                        slices.push(#slice_enum_ident::#variant);
                    }
                }
            })
            .collect();

        let name = item.ident.to_string();
        let ident = item.ident.clone();
        ensure_required_derives(&mut item.attrs);
        if cfg!(feature = "state-persistence") {
            ensure_serde_crate_attr(&mut item.attrs);
        }
        let docs = collect_doc_lines(&item.attrs);
        let fields = struct_fields(&item);
        item.attrs.push(syn::parse_quote!(#[doc = "oxide:state"]));
        push_meta_doc(
            &mut item.attrs,
            &StateMeta {
                kind: "state",
                name,
                docs,
                fields: Some(fields),
                variants: None,
            },
        );
        let persistence_bounds = if cfg!(feature = "state-persistence") {
            quote!(+ ::oxide_core::serde::Serialize + for<'de> ::oxide_core::serde::Deserialize<'de>)
        } else {
            quote!()
        };

        return quote!(
            #item

            /// Slice identifiers for top-level segments of this state.
            ///
            /// This enum is generated when `#[state(sliced = true)]` is enabled.
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum #slice_enum_ident {
                #( #slice_variants, )*
            }

            impl #ident {
                pub fn infer_slices_impl(before: &Self, after: &Self) -> Vec<#slice_enum_ident> {
                    let mut slices: Vec<#slice_enum_ident> = Vec::new();
                    #(#slice_checks)*
                    slices
                }
            }

            impl ::oxide_core::SlicedState for #ident {
                type StateSlice = #slice_enum_ident;

                fn infer_slices(before: &Self, after: &Self) -> Vec<Self::StateSlice> {
                    let mut slices: Vec<Self::StateSlice> = Vec::new();
                    #(#slice_checks)*
                    slices
                }
            }

            const _: () = {
                fn _oxide_require_state_traits<T>()
                where
                    T: ::core::fmt::Debug + Clone + PartialEq + Eq #persistence_bounds,
                {
                }
                let _ = _oxide_require_state_traits::<#ident>;
            };
        );
    }

    let name = item.ident.to_string();
    let ident = item.ident.clone();
    ensure_required_derives(&mut item.attrs);
    if cfg!(feature = "state-persistence") {
        ensure_serde_crate_attr(&mut item.attrs);
    }
    let docs = collect_doc_lines(&item.attrs);
    let fields = struct_fields(&item);
    item.attrs.push(syn::parse_quote!(#[doc = "oxide:state"]));
    push_meta_doc(
        &mut item.attrs,
        &StateMeta {
            kind: "state",
            name,
            docs,
            fields: Some(fields),
            variants: None,
        },
    );
    let persistence_bounds = if cfg!(feature = "state-persistence") {
        quote!(+ ::oxide_core::serde::Serialize + for<'de> ::oxide_core::serde::Deserialize<'de>)
    } else {
        quote!()
    };

    quote!(
        #item
        const _: () = {
            fn _oxide_require_state_traits<T>()
            where
                T: ::core::fmt::Debug + Clone + PartialEq + Eq #persistence_bounds,
            {
            }
            let _ = _oxide_require_state_traits::<#ident>;
        };
    )
}

pub(crate) fn expand_state_enum(args: StateArgs, mut item: ItemEnum) -> proc_macro2::TokenStream {
    if args.sliced {
        return syn::Error::new_spanned(
            &item,
            "enum slicing is not supported; model sliceable state as a struct with top-level fields and use #[state(sliced = true)]",
        )
        .to_compile_error();
    }

    let name = item.ident.to_string();
    let ident = item.ident.clone();
    ensure_required_derives(&mut item.attrs);
    if cfg!(feature = "state-persistence") {
        ensure_serde_crate_attr(&mut item.attrs);
    }
    let docs = collect_doc_lines(&item.attrs);
    let variants = enum_variants(&item);
    item.attrs.push(syn::parse_quote!(#[doc = "oxide:state"]));
    push_meta_doc(
        &mut item.attrs,
        &StateMeta {
            kind: "state",
            name,
            docs,
            fields: None,
            variants: Some(variants),
        },
    );
    let persistence_bounds = if cfg!(feature = "state-persistence") {
        quote!(+ ::oxide_core::serde::Serialize + for<'de> ::oxide_core::serde::Deserialize<'de>)
    } else {
        quote!()
    };

    quote!(
        #item
        const _: () = {
            fn _oxide_require_state_traits<T>()
            where
                T: ::core::fmt::Debug + Clone + PartialEq + Eq #persistence_bounds,
            {
            }
            let _ = _oxide_require_state_traits::<#ident>;
        };
    )
}

pub(crate) fn expand_actions_enum(mut item: ItemEnum) -> proc_macro2::TokenStream {
    let name = item.ident.to_string();
    let ident = item.ident.clone();
    ensure_required_derives(&mut item.attrs);
    if cfg!(feature = "state-persistence") {
        ensure_serde_crate_attr(&mut item.attrs);
    }
    let docs = collect_doc_lines(&item.attrs);
    let variants = enum_variants(&item);
    item.attrs.push(syn::parse_quote!(#[doc = "oxide:actions"]));
    push_meta_doc(
        &mut item.attrs,
        &ActionsMeta {
            kind: "actions",
            name,
            docs,
            variants,
        },
    );
    let persistence_bounds = if cfg!(feature = "state-persistence") {
        quote!(+ ::oxide_core::serde::Serialize + for<'de> ::oxide_core::serde::Deserialize<'de>)
    } else {
        quote!()
    };

    quote!(
        #item
        const _: () = {
            fn _oxide_require_actions_traits<T>()
            where
                T: ::core::fmt::Debug + Clone + PartialEq + Eq #persistence_bounds,
            {
            }
            let _ = _oxide_require_actions_traits::<#ident>;
        };
    )
}
