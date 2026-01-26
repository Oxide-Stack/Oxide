use quote::quote;
use syn::{Attribute, ItemEnum, ItemStruct, Token};

use crate::meta::{collect_doc_lines, enum_variants, push_meta_doc, struct_fields, ActionsMeta, StateMeta};

fn ensure_required_derives(attrs: &mut Vec<Attribute>) {
    let mut required: Vec<syn::Path> = vec![
        syn::parse_quote!(Debug),
        syn::parse_quote!(Clone),
        syn::parse_quote!(PartialEq),
        syn::parse_quote!(Eq),
    ];

    if cfg!(feature = "state-persistence") {
        required.push(syn::parse_quote!(::oxide_core::serde::Serialize));
        required.push(syn::parse_quote!(::oxide_core::serde::Deserialize));
    }

    ensure_derive(attrs, &required);
}

fn ensure_derive(attrs: &mut Vec<Attribute>, required: &[syn::Path]) {
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

pub(crate) fn expand_state_struct(mut item: ItemStruct) -> proc_macro2::TokenStream {
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

pub(crate) fn expand_state_enum(mut item: ItemEnum) -> proc_macro2::TokenStream {
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
