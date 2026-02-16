//! Metadata extraction and emission for Oxide generators.
//!
//! Oxide uses a deliberately simple cross-tooling interface: proc-macros emit
//! structured JSON into doc strings (`#[doc = "oxide:meta:<json>"]`).
//!
//! This allows downstream tooling (including non-Rust tooling) to discover type
//! shapes without requiring a Rust compiler plugin or full type-checking.
//!
//! - `#[state]` emits `StateMeta`
//! - `#[actions]` emits `ActionsMeta`
//! - `#[reducer(...)]` emits `ReducerMeta`

use quote::quote;
use serde::Serialize;
use syn::{Attribute, ItemEnum, ItemStruct};

#[derive(Debug, Serialize)]
pub(crate) struct FieldMeta {
    pub(crate) name: Option<String>,
    pub(crate) ty: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct StateMeta {
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) docs: Vec<String>,
    pub(crate) fields: Option<Vec<FieldMeta>>,
    pub(crate) variants: Option<Vec<VariantMeta>>,
}

#[derive(Debug, Serialize)]
pub(crate) struct VariantMeta {
    pub(crate) name: String,
    pub(crate) docs: Vec<String>,
    pub(crate) fields: Vec<FieldMeta>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ActionsMeta {
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) docs: Vec<String>,
    pub(crate) variants: Vec<VariantMeta>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ReducerMeta {
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) docs: Vec<String>,
    pub(crate) state: Option<String>,
    pub(crate) actions: Option<String>,
}

pub(crate) fn collect_doc_lines(attrs: &[Attribute]) -> Vec<String> {
    // Capture user-authored documentation while intentionally stripping the
    // `oxide:*` marker/docs this crate injects, so metadata doesn't recurse.
    attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            let meta = attr.meta.clone();
            match meta {
                syn::Meta::NameValue(nv) => match nv.value {
                    syn::Expr::Lit(expr_lit) => match expr_lit.lit {
                        syn::Lit::Str(s) => Some(s.value().trim().to_string()),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            }
        })
        .filter(|line| !line.starts_with("oxide:"))
        .collect()
}

pub(crate) fn push_meta_doc(attrs: &mut Vec<Attribute>, meta: &impl Serialize) {
    // Serialize metadata into a single doc string to keep it easy to locate and parse.
    let meta_json =
        serde_json::to_string(meta).expect("oxide_generator_rs: failed to serialize metadata");
    let meta_doc = syn::LitStr::new(
        &format!("oxide:meta:{meta_json}"),
        proc_macro2::Span::call_site(),
    );
    attrs.push(syn::parse_quote!(#[doc = #meta_doc]));
}

pub(crate) fn struct_fields(item: &ItemStruct) -> Vec<FieldMeta> {
    // Represent fields in a uniform way so tooling doesn't need to handle named/tuple/unit separately.
    match &item.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|f| {
                let ty = &f.ty;
                FieldMeta {
                    name: f.ident.as_ref().map(|x| x.to_string()),
                    ty: quote!(#ty).to_string(),
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(idx, f)| {
                let ty = &f.ty;
                FieldMeta {
                    name: Some(format!("_{idx}")),
                    ty: quote!(#ty).to_string(),
                }
            })
            .collect(),
        syn::Fields::Unit => Vec::new(),
    }
}

pub(crate) fn enum_variants(item: &ItemEnum) -> Vec<VariantMeta> {
    // Preserve per-variant docs and field shapes so downstream generators can produce
    // ergonomic APIs in other languages.
    item.variants
        .iter()
        .map(|v| VariantMeta {
            name: v.ident.to_string(),
            docs: collect_doc_lines(&v.attrs),
            fields: match &v.fields {
                syn::Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let ty = &f.ty;
                        FieldMeta {
                            name: f.ident.as_ref().map(|x| x.to_string()),
                            ty: quote!(#ty).to_string(),
                        }
                    })
                    .collect(),
                syn::Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(idx, f)| {
                        let ty = &f.ty;
                        FieldMeta {
                            name: Some(format!("_{idx}")),
                            ty: quote!(#ty).to_string(),
                        }
                    })
                    .collect(),
                syn::Fields::Unit => Vec::new(),
            },
        })
        .collect()
}
