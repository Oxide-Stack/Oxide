use quote::quote;
use syn::{Fields, Ident, ItemEnum, ItemStruct};

use crate::derive::common::{ensure_required_derives, ensure_serde_crate_attr};
use crate::derive::state_args::StateArgs;
use crate::meta::{StateMeta, collect_doc_lines, enum_variants, push_meta_doc, struct_fields};

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

fn find_duplicate_slice_variants(
    field_variants: &[(&Ident, Ident)],
) -> Option<(String, Vec<String>)> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();
    for (field, variant) in field_variants {
        let variant = variant.to_string();
        let field = field.to_string();
        match groups.iter_mut().find(|(v, _)| *v == variant) {
            Some((_, fields)) => fields.push(field),
            None => groups.push((variant, vec![field])),
        }
    }
    groups.into_iter().find(|(_, fields)| fields.len() > 1)
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


        let slice_enum_ident = quote::format_ident!("{}Slice", item.ident);

        let field_variants: Vec<(&Ident, Ident)> = named
            .named
            .iter()
            .filter_map(|f| f.ident.as_ref())
            .map(|field| (field, slice_variant_ident(field)))
            .collect();

        if let Some((variant, colliding_fields)) = find_duplicate_slice_variants(&field_variants) {
            let field_list = colliding_fields
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ");
            return syn::Error::new_spanned(
                &item,
                format!(
                    "fields {field_list} all map to the duplicate slice variant `{variant}`; rename these fields so each generates a unique variant"
                ),
            )
            .to_compile_error();
        }

        let slice_variants: Vec<Ident> = field_variants
            .into_iter()
            .map(|(_, variant)| variant)
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

                pub(crate) const __OXIDE_SLICED_STATE: bool = true;
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

    let slice_enum_ident = quote::format_ident!("{}Slice", item.ident);
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #slice_enum_ident {
            __OxideUnused,
        }

        impl #ident {
            pub fn infer_slices_impl(
                _before: &Self,
                _after: &Self,
            ) -> Vec<#slice_enum_ident> {
                Vec::new()
            }

            pub(crate) const __OXIDE_SLICED_STATE: bool = false;
        }

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

    let slice_enum_ident = quote::format_ident!("{}Slice", item.ident);
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum #slice_enum_ident {
            __OxideUnused,
        }

        impl #ident {
            pub fn infer_slices_impl(
                _before: &Self,
                _after: &Self,
            ) -> Vec<#slice_enum_ident> {
                Vec::new()
            }

            pub(crate) const __OXIDE_SLICED_STATE: bool = false;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_field(name: &str) -> Ident {
        syn::parse_str::<Ident>(name).unwrap()
    }

    #[test]
    fn colliding_snake_and_camel_fields_are_detected() {
        let f1 = parse_field("a_b");
        let f2 = parse_field("aB");
        let field_variants = vec![
            (&f1, slice_variant_ident(&f1)),
            (&f2, slice_variant_ident(&f2)),
        ];

        let (variant, fields) = find_duplicate_slice_variants(&field_variants).unwrap();
        assert_eq!(variant, "AB");
        assert_eq!(fields, vec!["a_b".to_string(), "aB".to_string()]);
    }

    #[test]
    fn underscore_prefix_collision_is_detected() {
        let f1 = parse_field("value");
        let f2 = parse_field("_value");
        let f3 = parse_field("unrelated");
        let field_variants = vec![
            (&f1, slice_variant_ident(&f1)),
            (&f2, slice_variant_ident(&f2)),
            (&f3, slice_variant_ident(&f3)),
        ];

        let (variant, fields) = find_duplicate_slice_variants(&field_variants).unwrap();
        assert_eq!(variant, "Value");
        assert_eq!(fields, vec!["value".to_string(), "_value".to_string()]);
    }

    #[test]
    fn distinct_fields_have_no_collisions() {
        let f1 = parse_field("a");
        let f2 = parse_field("b_c");
        let f3 = parse_field("dB");
        let f4 = parse_field("_leading");
        let field_variants = vec![
            (&f1, slice_variant_ident(&f1)),
            (&f2, slice_variant_ident(&f2)),
            (&f3, slice_variant_ident(&f3)),
            (&f4, slice_variant_ident(&f4)),
        ];

        assert!(find_duplicate_slice_variants(&field_variants).is_none());
    }
}
