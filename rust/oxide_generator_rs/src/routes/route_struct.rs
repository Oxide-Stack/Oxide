use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{Attribute, ItemStruct, LitStr, Token, Type};

use crate::routes::args::{OxideRouteArgs, RouteFieldArgs};

pub fn expand_oxide_route_struct(
    args: OxideRouteArgs,
    mut item_struct: ItemStruct,
) -> syn::Result<TokenStream2> {
    let path_value = args.path.as_ref().map(|p| p.value());
    let ident = item_struct.ident.clone();
    validate_oxide_route_struct(&item_struct)?;
    ensure_route_struct_derives(&mut item_struct);
    ensure_frb_non_opaque_attr(&mut item_struct);
    normalize_route_struct_fields(&mut item_struct);
    let return_ty: Type = args
        .return_type
        .unwrap_or_else(|| syn::parse_quote!(oxide_core::navigation::NoReturn));
    let extra_ty: Type = args
        .extra_type
        .unwrap_or_else(|| syn::parse_quote!(oxide_core::navigation::NoExtra));

    let path_fn = match args.path {
        Some(path) => quote! {
            fn path() -> Option<&'static str> { Some(#path) }
        },
        None => quote! {},
    };

    let mut param_inserts = Vec::<TokenStream2>::new();
    let mut query_inserts = Vec::<TokenStream2>::new();
    let mut args_fields = 0usize;
    let mut extra_fields = 0usize;

    if let syn::Fields::Named(named) = &item_struct.fields {
        for field in &named.named {
            let Some(field_ident) = field.ident.clone() else {
                continue;
            };
            let field_name = field_ident.to_string();
            let key_lit = LitStr::new(&field_name, Span::call_site());

            let route_args = field
                .attrs
                .iter()
                .find(|a| {
                    a.path().segments.last().map(|s| s.ident.to_string())
                        == Some("route".to_string())
                })
                .and_then(|a| a.parse_args::<RouteFieldArgs>().ok())
                .or_else(|| {
                    let has_param_in_path = path_value
                        .as_ref()
                        .is_some_and(|p| p.contains(&format!(":{field_name}")));
                    if has_param_in_path {
                        Some(RouteFieldArgs {
                            kind: Some("param".to_string()),
                            key: None,
                        })
                    } else {
                        None
                    }
                });

            let Some(route_args) = route_args else {
                continue;
            };
            let Some(kind) = route_args.kind else {
                continue;
            };
            let key_lit = route_args.key.unwrap_or(key_lit);

            let is_option = is_option_type(&field.ty);
            match kind.as_str() {
                "param" => {
                    let insert = if is_option {
                        quote! {
                            if let Some(v) = &self.#field_ident {
                                map.insert(#key_lit, v.to_string());
                            }
                        }
                    } else {
                        quote! { map.insert(#key_lit, self.#field_ident.to_string()); }
                    };
                    param_inserts.push(insert);
                }
                "query" => {
                    let insert = if is_option {
                        quote! {
                            if let Some(v) = &self.#field_ident {
                                map.insert(#key_lit, v.to_string());
                            }
                        }
                    } else {
                        quote! { map.insert(#key_lit, self.#field_ident.to_string()); }
                    };
                    query_inserts.push(insert);
                }
                "args" => {
                    args_fields += 1;
                }
                "extra" => {
                    extra_fields += 1;
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        field,
                        "unknown #[route(kind = \"...\")] value; expected param|query|args|extra",
                    ));
                }
            }
        }
    }

    if args_fields > 1 {
        return Err(syn::Error::new_spanned(
            &ident,
            "route supports at most one #[route(kind = \"args\")] field",
        ));
    }
    if extra_fields > 1 {
        return Err(syn::Error::new_spanned(
            &ident,
            "route supports at most one #[route(kind = \"extra\")] field",
        ));
    }

    let params_fn = if param_inserts.is_empty() {
        quote! {}
    } else {
        quote! {
            fn params(&self) -> ::std::collections::HashMap<&'static str, String> {
                let mut map = ::std::collections::HashMap::new();
                #( #param_inserts )*
                map
            }
        }
    };

    let query_fn = if query_inserts.is_empty() {
        quote! {}
    } else {
        quote! {
            fn query(&self) -> ::std::collections::HashMap<&'static str, String> {
                let mut map = ::std::collections::HashMap::new();
                #( #query_inserts )*
                map
            }
        }
    };

    Ok(quote! {
        #item_struct

        impl oxide_core::navigation::Route for #ident {
            #path_fn
            #params_fn
            #query_fn
            type Return = #return_ty;
            type Extra = #extra_ty;
        }
    })
}

fn is_option_type(ty: &Type) -> bool {
    let Type::Path(p) = ty else { return false };
    let Some(seg) = p.path.segments.last() else {
        return false;
    };
    if seg.ident != "Option" {
        return false;
    }
    matches!(&seg.arguments, syn::PathArguments::AngleBracketed(args) if args.args.len() == 1)
}

fn ensure_frb_non_opaque_attr(item_struct: &mut ItemStruct) {
    let already_has_frb = item_struct
        .attrs
        .iter()
        .any(|a| a.path().segments.last().map(|s| s.ident.to_string()) == Some("frb".to_string()));
    if already_has_frb {
        return;
    }
    item_struct
        .attrs
        .push(syn::parse_quote!(#[flutter_rust_bridge::frb(non_opaque)]));
}

pub(super) fn ensure_route_struct_derives(item_struct: &mut ItemStruct) {
    let mut missing = Vec::<syn::Path>::new();
    if !has_derive_named(&item_struct.attrs, "Clone") {
        missing.push(syn::parse_quote!(Clone));
    }
    if !has_derive_named(&item_struct.attrs, "Debug") {
        missing.push(syn::parse_quote!(Debug));
    }
    if !has_derive_named(&item_struct.attrs, "Serialize") {
        missing.push(syn::parse_quote!(serde::Serialize));
    }
    if !has_derive_named(&item_struct.attrs, "Deserialize") {
        missing.push(syn::parse_quote!(serde::Deserialize));
    }

    if missing.is_empty() {
        return;
    }

    item_struct
        .attrs
        .push(syn::parse_quote!(#[derive(#(#missing),*)]));
}

fn normalize_route_struct_fields(item_struct: &mut ItemStruct) {
    if matches!(&item_struct.fields, syn::Fields::Unit) {
        item_struct.fields = syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: syn::punctuated::Punctuated::new(),
        });
    }
}

fn validate_oxide_route_struct(item_struct: &ItemStruct) -> syn::Result<()> {
    if !item_struct.generics.params.is_empty() || item_struct.generics.where_clause.is_some() {
        return Err(syn::Error::new_spanned(
            &item_struct.generics,
            "route structs cannot be generic; remove type parameters and where-clauses",
        ));
    }

    if let syn::Fields::Named(named) = &item_struct.fields {
        for field in &named.named {
            validate_route_field_type(&field.ty, field)?;
        }
    }

    Ok(())
}

fn has_derive_named(attrs: &[Attribute], needle: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident("derive") {
            let Ok(list) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, Token![,]>::parse_terminated,
            ) else {
                continue;
            };
            for p in list {
                if p.segments.last().map(|s| s.ident.to_string()) == Some(needle.to_string()) {
                    return true;
                }
            }
        }
    }
    false
}

fn validate_route_field_type(ty: &Type, span: impl ToTokens) -> syn::Result<()> {
    match ty {
        Type::Reference(_)
        | Type::Ptr(_)
        | Type::BareFn(_)
        | Type::ImplTrait(_)
        | Type::TraitObject(_)
        | Type::Infer(_)
        | Type::Macro(_)
        | Type::Verbatim(_)
        | Type::Never(_) => Err(syn::Error::new_spanned(
            span,
            "route fields must use concrete, owned types supported by FRB (no references, pointers, impl Trait, trait objects, or macros)",
        )),
        _ => Ok(()),
    }
}
