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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct DummyMeta {
        kind: &'static str,
        name: &'static str,
    }

    #[test]
    fn collect_doc_lines_filters_oxide_markers() {
        let item: ItemStruct = syn::parse_str(
            "#[doc = \"  user docs  \" ] #[doc = \"oxide:meta:{...}\"] struct S;",
        )
        .unwrap();
        assert_eq!(collect_doc_lines(&item.attrs), vec!["user docs".to_string()]);
    }

    #[test]
    fn push_meta_doc_appends_serialized_doc_attribute() {
        let mut attrs: Vec<Attribute> = Vec::new();
        push_meta_doc(
            &mut attrs,
            &DummyMeta {
                kind: "state",
                name: "AppState",
            },
        );
        assert_eq!(attrs.len(), 1);
        let rendered = quote::quote!(#(#attrs)*).to_string();
        assert!(rendered.contains("oxide:meta:"));
        assert!(rendered.contains("AppState"));
    }

    #[test]
    fn struct_fields_handles_named_unnamed_and_unit() {
        let named: ItemStruct = syn::parse_str("struct N { id: u64, name: String }").unwrap();
        let named_fields = struct_fields(&named);
        assert_eq!(named_fields.len(), 2);
        assert_eq!(named_fields[0].name.as_deref(), Some("id"));

        let unnamed: ItemStruct = syn::parse_str("struct U(u64, String);").unwrap();
        let unnamed_fields = struct_fields(&unnamed);
        assert_eq!(unnamed_fields.len(), 2);
        assert_eq!(unnamed_fields[0].name.as_deref(), Some("_0"));

        let unit: ItemStruct = syn::parse_str("struct Z;").unwrap();
        assert!(struct_fields(&unit).is_empty());
    }

    #[test]
    fn enum_variants_preserves_docs_and_fields() {
        let item: ItemEnum = syn::parse_str(
            "enum E { #[doc = \"v docs\"] A { id: u64 }, B(String), C }",
        )
        .unwrap();
        let variants = enum_variants(&item);
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].name, "A");
        assert_eq!(variants[0].docs, vec!["v docs".to_string()]);
        assert_eq!(variants[0].fields[0].name.as_deref(), Some("id"));
        assert_eq!(variants[1].fields[0].name.as_deref(), Some("_0"));
        assert!(variants[2].fields.is_empty());
    }
}
