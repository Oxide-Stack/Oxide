use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{ImplItem, ItemEnum, ItemImpl, Type};

use super::naming::to_snake_case;
use super::scan::find_enum_in_crate_src;
use super::validate::{type_to_simple_ident, validate_enum_payload};

/// Arguments supported by `#[oxide_callback(...)]`.
pub struct OxideCallbackArgs {
    pub no_frb: bool,
}

impl syn::parse::Parse for OxideCallbackArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { no_frb: false });
        }

        let args = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;
        let mut no_frb = false;
        for meta in args {
            match meta {
                syn::Meta::Path(path) if path.is_ident("no_frb") => {
                    no_frb = true;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "unsupported arguments; supported: `no_frb`",
                    ));
                }
            }
        }
        Ok(Self { no_frb })
    }
}

/// Expands `#[oxide_callback]` for `OxideCallbacking`.
pub fn expand_oxide_callback(args: OxideCallbackArgs, item_impl: ItemImpl) -> syn::Result<TokenStream2> {
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return Err(syn::Error::new_spanned(
            &item_impl,
            "#[oxide_callback] must be applied to an impl of OxideCallbacking",
        ));
    };

    let trait_ident = trait_path.segments.last().map(|s| s.ident.to_string());
    if trait_ident.as_deref() != Some("OxideCallbacking") {
        return Err(syn::Error::new_spanned(
            trait_path,
            "#[oxide_callback] must be applied to an impl of OxideCallbacking",
        ));
    }

    let self_ident = impl_self_ident(&item_impl.self_ty)?;
    let request_ty = find_assoc_type(&item_impl, "Request")?;
    let response_ty = find_assoc_type(&item_impl, "Response")?;

    let request_ident = type_to_simple_ident(&request_ty)?;
    let response_ident = type_to_simple_ident(&response_ty)?;

    let Some(request_enum) = find_enum_in_crate_src(&request_ident.to_string())? else {
        return Err(syn::Error::new_spanned(
            request_ty,
            "Request type must be an enum defined in the current crate `src/`",
        ));
    };
    validate_enum_payload(&request_enum)?;

    let Some(response_enum) = find_enum_in_crate_src(&response_ident.to_string())? else {
        return Err(syn::Error::new_spanned(
            response_ty,
            "Response type must be an enum defined in the current crate `src/`",
        ));
    };
    validate_enum_payload(&response_enum)?;

    validate_request_response_parity(&request_enum, &response_enum)?;

    let service_snake = to_snake_case(&self_ident.to_string());
    let runtime_mod_ident = format_ident!("__oxide_isolated_callback_{service_snake}");
    let envelope_ident = format_ident!("__OxideCallbackRequest_{service_snake}");
    let stream_fn_ident = format_ident!("oxide_{service_snake}_requests_stream");
    let respond_fn_ident = format_ident!("oxide_{service_snake}_respond");

    let methods = generate_callback_methods(&self_ident, &request_ty, &request_enum, &response_ty, &response_enum)?;

    let frb_mod = if args.no_frb {
        quote! {}
    } else {
        quote! {
            pub mod frb {
                use super::*;

                #[flutter_rust_bridge::frb]
                pub async fn #stream_fn_ident(
                    sink: crate::frb_generated::StreamSink<#envelope_ident>,
                ) {
                    loop {
                        let Some((id, request)) = super::runtime().recv_request().await else {
                            break;
                        };
                        let _ = sink.add(#envelope_ident { id, request });
                    }
                }

                #[flutter_rust_bridge::frb]
                pub async fn #respond_fn_ident(
                    id: u64,
                    response: #response_ty,
                ) -> Result<(), ::oxide_core::OxideChannelError> {
                    super::runtime().respond(id, response).await
                }
            }
        }
    };

    Ok(quote! {
        #item_impl

        #methods

        #[doc(hidden)]
        pub mod #runtime_mod_ident {
            use super::*;

            #[derive(Clone, Debug)]
            pub struct #envelope_ident {
                pub id: u64,
                pub request: #request_ty,
            }

            static RUNTIME: ::std::sync::OnceLock<::oxide_core::CallbackRuntime<#request_ty, #response_ty>> =
                ::std::sync::OnceLock::new();

            pub fn runtime() -> &'static ::oxide_core::CallbackRuntime<#request_ty, #response_ty> {
                let _ = ::oxide_core::init_isolated_channels();
                RUNTIME.get_or_init(|| ::oxide_core::CallbackRuntime::new(64))
            }

            #frb_mod
        }
    })
}

fn validate_request_response_parity(request_enum: &ItemEnum, response_enum: &ItemEnum) -> syn::Result<()> {
    for req_variant in &request_enum.variants {
        let name = &req_variant.ident;
        let has_match = response_enum.variants.iter().any(|v| v.ident == *name);
        if !has_match {
            return Err(syn::Error::new_spanned(
                req_variant,
                format!("missing Response variant with identical name: `{name}`"),
            ));
        }
    }
    Ok(())
}

fn generate_callback_methods(
    self_ident: &syn::Ident,
    request_ty: &Type,
    request_enum: &ItemEnum,
    response_ty: &Type,
    response_enum: &ItemEnum,
) -> syn::Result<TokenStream2> {
    let service_snake = to_snake_case(&self_ident.to_string());
    let runtime_mod_ident = format_ident!("__oxide_isolated_callback_{service_snake}");

    let mut out = Vec::new();
    for req_variant in &request_enum.variants {
        let variant_ident = &req_variant.ident;
        let method_ident = format_ident!("{}", to_snake_case(&variant_ident.to_string()));

        let (args, req_ctor) = variant_ctor(request_ty, req_variant)?;

        let resp_variant = response_enum
            .variants
            .iter()
            .find(|v| v.ident == *variant_ident)
            .expect("parity checked above");

        let (ret_ty, match_arm) = response_match_arm(response_ty, resp_variant)?;

        out.push(quote! {
            pub async fn #method_ident(#args) -> Result<#ret_ty, ::oxide_core::OxideChannelError> {
                let response = #runtime_mod_ident::runtime().invoke(#req_ctor).await?;
                match response {
                    #match_arm,
                    _ => Err(::oxide_core::OxideChannelError::UnexpectedResponse),
                }
            }
        });
    }

    Ok(quote! {
        impl #self_ident {
            #(#out)*
        }
    })
}

fn response_match_arm(response_ty: &Type, variant: &syn::Variant) -> syn::Result<(Type, TokenStream2)> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        syn::Fields::Unit => Ok((
            syn::parse_quote!(()),
            quote! { #response_ty::#variant_ident => Ok(()) },
        )),
        syn::Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                return Err(syn::Error::new_spanned(
                    fields,
                    "response variants must have exactly one field (or be unit) to map to a return type",
                ));
            }
            let ty = fields.unnamed.first().unwrap().ty.clone();
            Ok((ty, quote! { #response_ty::#variant_ident(v) => Ok(v) }))
        }
        syn::Fields::Named(fields) => {
            if fields.named.len() != 1 {
                return Err(syn::Error::new_spanned(
                    fields,
                    "response variants must have exactly one field (or be unit) to map to a return type",
                ));
            }
            let field = fields.named.first().unwrap();
            let Some(ident) = &field.ident else {
                return Err(syn::Error::new_spanned(fields, "expected a named field"));
            };
            let ty = field.ty.clone();
            Ok((
                ty,
                quote! { #response_ty::#variant_ident { #ident } => Ok(#ident) },
            ))
        }
    }
}

fn variant_ctor(enum_ty: &Type, variant: &syn::Variant) -> syn::Result<(TokenStream2, TokenStream2)> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        syn::Fields::Unit => Ok((quote! {}, quote! { #enum_ty::#variant_ident })),
        syn::Fields::Unnamed(fields) => {
            let mut args = Vec::new();
            let mut vals = Vec::new();
            for (idx, field) in fields.unnamed.iter().enumerate() {
                let arg_ident = format_ident!("arg{idx}");
                let ty = &field.ty;
                args.push(quote! { #arg_ident: #ty });
                vals.push(quote! { #arg_ident });
            }
            Ok((
                quote! { #(#args),* },
                quote! { #enum_ty::#variant_ident ( #(#vals),* ) },
            ))
        }
        syn::Fields::Named(fields) => {
            let mut args = Vec::new();
            let mut vals = Vec::new();
            for field in &fields.named {
                let Some(ident) = &field.ident else { continue };
                let ty = &field.ty;
                args.push(quote! { #ident: #ty });
                vals.push(quote! { #ident });
            }
            Ok((
                quote! { #(#args),* },
                quote! { #enum_ty::#variant_ident { #(#vals),* } },
            ))
        }
    }
}

fn find_assoc_type(item_impl: &ItemImpl, assoc: &str) -> syn::Result<Type> {
    for item in &item_impl.items {
        let ImplItem::Type(ty_item) = item else { continue };
        if ty_item.ident == assoc {
            return Ok(ty_item.ty.clone());
        }
    }
    Err(syn::Error::new_spanned(
        item_impl,
        format!("missing associated type `{assoc}`"),
    ))
}

fn impl_self_ident(ty: &Type) -> syn::Result<syn::Ident> {
    let Type::Path(type_path) = ty else {
        return Err(syn::Error::new_spanned(ty, "expected a concrete self type"));
    };
    let Some(seg) = type_path.path.segments.last() else {
        return Err(syn::Error::new_spanned(ty, "expected a concrete self type"));
    };
    Ok(seg.ident.clone())
}
