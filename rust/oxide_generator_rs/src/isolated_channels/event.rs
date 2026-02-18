use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{ImplItem, ItemEnum, ItemImpl, LitBool, Type};

use super::naming::to_snake_case;
use super::scan::find_enum_in_crate_src;
use super::validate::{type_to_simple_ident, validate_enum_payload};

/// Arguments supported by `#[oxide_event_channel(...)]`.
pub struct OxideEventChannelArgs {
    pub orphaned: bool,
    pub no_frb: bool,
}

impl syn::parse::Parse for OxideEventChannelArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { orphaned: false, no_frb: false });
        }

        let mut orphaned = false;
        let mut no_frb = false;

        let args = syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;
        for meta in args {
            match meta {
                syn::Meta::Path(path) if path.is_ident("no_frb") => {
                    no_frb = true;
                }
                syn::Meta::NameValue(nv) if nv.path.is_ident("orphaned") => {
                    let expr = nv.value;
                    let syn::Expr::Lit(expr_lit) = expr else {
                        return Err(syn::Error::new_spanned(
                            expr,
                            "expected `orphaned = true` or `orphaned = false`",
                        ));
                    };
                    let LitBool { value, .. } = syn::parse2::<LitBool>(expr_lit.lit.to_token_stream())?;
                    orphaned = value;
                }
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "unsupported arguments; supported: `orphaned = true`, `no_frb`",
                    ));
                }
            }
        }

        Ok(Self { orphaned, no_frb })
    }
}

/// Expands `#[oxide_event_channel]` for `OxideEventChannel` and `OxideEventDuplexChannel`.
pub fn expand_oxide_event_channel(
    args: OxideEventChannelArgs,
    item_impl: ItemImpl,
) -> syn::Result<TokenStream2> {
    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return Err(syn::Error::new_spanned(
            &item_impl,
            "#[oxide_event_channel] must be applied to an impl of OxideEventChannel or OxideEventDuplexChannel",
        ));
    };

    let trait_ident = trait_path.segments.last().map(|s| s.ident.to_string());
    match trait_ident.as_deref() {
        Some("OxideEventChannel") => expand_event_channel(args, item_impl),
        Some("OxideEventDuplexChannel") => expand_duplex_channel(args, item_impl),
        _ => Err(syn::Error::new_spanned(
            trait_path,
            "#[oxide_event_channel] must be applied to an impl of OxideEventChannel or OxideEventDuplexChannel",
        )),
    }
}

fn expand_event_channel(args: OxideEventChannelArgs, item_impl: ItemImpl) -> syn::Result<TokenStream2> {
    let self_ident = impl_self_ident(&item_impl.self_ty)?;
    let events_ty = find_assoc_type(&item_impl, "Events")?;
    let events_ident = type_to_simple_ident(&events_ty)?;

    let Some(events_enum) = find_enum_in_crate_src(&events_ident.to_string())? else {
        return Err(syn::Error::new_spanned(
            events_ty,
            "Events type must be an enum defined in the current crate `src/`",
        ));
    };
    validate_enum_payload(&events_enum)?;

    let runtime_mod_ident = format_ident!("__oxide_isolated_events_{}", to_snake_case(&self_ident.to_string()));
    let send_helpers =
        generate_event_send_helpers(&self_ident, &events_ty, &events_enum, &runtime_mod_ident)?;
    let stream_fn_ident = if args.orphaned {
        format_ident!(
            "oxide_{}_events_stream",
            to_snake_case(&self_ident.to_string())
        )
    } else {
        format_ident!("oxide_events_stream")
    };

    let frb_mod = if args.no_frb {
        quote! {}
    } else {
        quote! {
            pub mod frb {
                use super::*;

                #[flutter_rust_bridge::frb]
                pub async fn #stream_fn_ident(
                    sink: crate::frb_generated::StreamSink<#events_ty>,
                ) {
                    let mut rx = super::runtime().subscribe();
                    loop {
                        match rx.recv().await {
                            Ok(event) => {
                                let _ = sink.add(event);
                            }
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }
            }
        }
    };

    let expanded = quote! {
        #item_impl

        #send_helpers

        #[doc(hidden)]
        pub mod #runtime_mod_ident {
            use super::*;

            static RUNTIME: ::std::sync::OnceLock<::oxide_core::EventChannelRuntime<#events_ty>> =
                ::std::sync::OnceLock::new();

            pub fn runtime() -> &'static ::oxide_core::EventChannelRuntime<#events_ty> {
                let _ = ::oxide_core::init_isolated_channels();
                RUNTIME.get_or_init(|| ::oxide_core::EventChannelRuntime::new(64))
            }

            #frb_mod
        }
    };

    Ok(expanded)
}

fn expand_duplex_channel(args: OxideEventChannelArgs, item_impl: ItemImpl) -> syn::Result<TokenStream2> {
    let self_ident = impl_self_ident(&item_impl.self_ty)?;
    let outgoing_ty = find_assoc_type(&item_impl, "Outgoing")?;
    let incoming_ty = find_assoc_type(&item_impl, "Incoming")?;

    let outgoing_ident = type_to_simple_ident(&outgoing_ty)?;
    let incoming_ident = type_to_simple_ident(&incoming_ty)?;

    let Some(outgoing_enum) = find_enum_in_crate_src(&outgoing_ident.to_string())? else {
        return Err(syn::Error::new_spanned(
            outgoing_ty,
            "Outgoing type must be an enum defined in the current crate `src/`",
        ));
    };
    validate_enum_payload(&outgoing_enum)?;

    let Some(incoming_enum) = find_enum_in_crate_src(&incoming_ident.to_string())? else {
        return Err(syn::Error::new_spanned(
            incoming_ty,
            "Incoming type must be an enum defined in the current crate `src/`",
        ));
    };
    validate_enum_payload(&incoming_enum)?;

    let runtime_mod_ident = format_ident!("__oxide_isolated_duplex_{}", to_snake_case(&self_ident.to_string()));
    let send_helpers = generate_duplex_outgoing_helpers(
        &self_ident,
        &outgoing_ty,
        &outgoing_enum,
        &incoming_ty,
        &runtime_mod_ident,
    )?;

    let out_stream_fn_ident = if args.orphaned {
        format_ident!(
            "oxide_{}_outgoing_stream",
            to_snake_case(&self_ident.to_string())
        )
    } else {
        format_ident!("oxide_outgoing_stream")
    };

    let incoming_fn_ident = format_ident!(
        "oxide_{}_incoming",
        to_snake_case(&self_ident.to_string())
    );

    let frb_mod = if args.no_frb {
        quote! {}
    } else {
        quote! {
            pub mod frb {
                use super::*;

                #[flutter_rust_bridge::frb]
                pub async fn #out_stream_fn_ident(
                    sink: crate::frb_generated::StreamSink<#outgoing_ty>,
                ) {
                    let mut rx = super::outgoing().subscribe();
                    loop {
                        match rx.recv().await {
                            Ok(event) => {
                                let _ = sink.add(event);
                            }
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }

                #[flutter_rust_bridge::frb]
                pub fn #incoming_fn_ident(
                    event: #incoming_ty,
                ) -> Result<(), ::oxide_core::OxideChannelError> {
                    super::incoming().handle(event)
                }
            }
        }
    };

    let expanded = quote! {
        #item_impl

        #send_helpers

        #[doc(hidden)]
        pub mod #runtime_mod_ident {
            use super::*;

            static OUTGOING: ::std::sync::OnceLock<::oxide_core::EventChannelRuntime<#outgoing_ty>> =
                ::std::sync::OnceLock::new();
            static INCOMING: ::std::sync::OnceLock<::oxide_core::IncomingHandler<#incoming_ty>> =
                ::std::sync::OnceLock::new();

            pub fn outgoing() -> &'static ::oxide_core::EventChannelRuntime<#outgoing_ty> {
                let _ = ::oxide_core::init_isolated_channels();
                OUTGOING.get_or_init(|| ::oxide_core::EventChannelRuntime::new(64))
            }

            pub fn incoming() -> &'static ::oxide_core::IncomingHandler<#incoming_ty> {
                let _ = ::oxide_core::init_isolated_channels();
                INCOMING.get_or_init(::oxide_core::IncomingHandler::new)
            }

            #frb_mod
        }
    };

    Ok(expanded)
}

fn generate_event_send_helpers(
    self_ident: &syn::Ident,
    events_ty: &Type,
    events_enum: &ItemEnum,
    runtime_mod_ident: &syn::Ident,
) -> syn::Result<TokenStream2> {
    let mut helper_fns = Vec::new();

    for variant in &events_enum.variants {
        let variant_ident = &variant.ident;
        let method_ident = format_ident!("{}", to_snake_case(&variant_ident.to_string()));

        let (args, ctor) = variant_ctor(events_ty, variant)?;
        helper_fns.push(quote! {
            pub fn #method_ident(#args) {
                let event = #ctor;
                Self::__oxide_send_event(event);
            }
        });
    }

    Ok(quote! {
        impl #self_ident {
            fn __oxide_send_event(event: #events_ty) {
                #runtime_mod_ident::runtime().emit(event);
            }

            #(#helper_fns)*
        }
    })
}

fn generate_duplex_outgoing_helpers(
    self_ident: &syn::Ident,
    outgoing_ty: &Type,
    _outgoing_enum: &ItemEnum,
    incoming_ty: &Type,
    runtime_mod_ident: &syn::Ident,
) -> syn::Result<TokenStream2> {
    Ok(quote! {
        impl #self_ident {
            pub fn send(event: #outgoing_ty) {
                #runtime_mod_ident::outgoing().emit(event);
            }

            pub fn register_incoming(
                handler: impl Fn(#incoming_ty) + Send + Sync + 'static
            ) {
                #runtime_mod_ident::incoming().register(handler);
            }
        }
    })
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
