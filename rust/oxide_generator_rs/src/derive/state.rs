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

        // The previously-generated enum name was always `StateSlice`, which
        // collides when multiple sliced state structs exist in the same module.
        //
        // Make the generated enum name state-specific, e.g. `MyStateSlice`.
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
