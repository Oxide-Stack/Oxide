use quote::{format_ident, quote};
use syn::ItemImpl;

use crate::meta::{ReducerMeta, push_meta_doc};
use crate::reducer::expand::analysis::ReducerAnalysis;
use crate::reducer::validate::type_path_last_segment;

pub(super) struct EmitArgs {
    pub(super) engine_ident: syn::Ident,
    pub(super) snapshot_ident: syn::Ident,
    pub(super) initial_state: syn::Expr,
    pub(super) reducer_expr: Option<syn::Expr>,
    pub(super) include_frb: bool,
    pub(super) persist_key: Option<syn::LitStr>,
    pub(super) persist_min_interval_ms: Option<u64>,
}

pub(super) fn emit_reducer_tokens(
    item_impl: ItemImpl,
    analysis: ReducerAnalysis,
    args: EmitArgs,
) -> proc_macro2::TokenStream {
    let EmitArgs {
        engine_ident,
        snapshot_ident,
        initial_state,
        reducer_expr,
        include_frb,
        persist_key,
        persist_min_interval_ms,
    } = args;
    let ReducerAnalysis {
        reducer_ident,
        state_ty,
        action_ty,
        uses_sliced_updates,
        state_slice_ty,
        sliced_state_assert,
    } = analysis;

    let reducer_name = reducer_ident.to_string();
    let state_name = type_path_last_segment(&state_ty);
    let actions_name = type_path_last_segment(&action_ty);

    let marker_ident = format_ident!("__OxideReducerMarker_{reducer_ident}");

    let mut marker_item: syn::ItemStruct = syn::parse_quote!(struct #marker_ident {});
    marker_item
        .attrs
        .push(syn::parse_quote!(#[doc = "oxide:reducer"]));
    push_meta_doc(
        &mut marker_item.attrs,
        &ReducerMeta {
            kind: "reducer",
            name: reducer_name,
            docs: Vec::new(),
            state: state_name,
            actions: actions_name,
        },
    );

    let reducer_init_expr = reducer_expr.unwrap_or_else(
        || syn::parse_quote!(<#reducer_ident as ::core::default::Default>::default()),
    );

    let core_engine_path = if let Some(state_slice_ty) = state_slice_ty.as_ref() {
        quote!(oxide_core::ReducerEngine::<#reducer_ident, #state_slice_ty>)
    } else {
        quote!(oxide_core::ReducerEngine::<#reducer_ident>)
    };

    let core_engine_type = if let Some(state_slice_ty) = state_slice_ty.as_ref() {
        quote!(oxide_core::ReducerEngine<#reducer_ident, #state_slice_ty>)
    } else {
        quote!(oxide_core::ReducerEngine<#reducer_ident>)
    };

    let engine_init_default = if cfg!(feature = "state-persistence") {
        if let Some(persist_key) = &persist_key {
            let ms = persist_min_interval_ms.unwrap_or(200);
            quote!(
                #core_engine_path::new_persistent(
                    #reducer_init_expr,
                    #initial_state,
                    oxide_core::persistence::PersistenceConfig {
                        key: #persist_key.to_string(),
                        min_interval: ::std::time::Duration::from_millis(#ms),
                    },
                ).await
            )
        } else {
            quote!(#core_engine_path::new(#reducer_init_expr, #initial_state).await)
        }
    } else {
        quote!(#core_engine_path::new(#reducer_init_expr, #initial_state).await)
    };

    let frb_tokens = if cfg!(feature = "frb") && include_frb {
        quote! {
            #[allow(unused_imports)]
            pub use oxide_core::OxideError;

            #[flutter_rust_bridge::frb]
            pub async fn create_engine() -> Result<std::sync::Arc<#engine_ident>, oxide_core::OxideError> {
                Ok(std::sync::Arc::new(#engine_ident::new().await?))
            }

            #[flutter_rust_bridge::frb]
            pub fn dispose_engine(_engine: &std::sync::Arc<#engine_ident>) {}

            #[flutter_rust_bridge::frb]
            pub async fn setup_rust_logs(sink: crate::frb_generated::StreamSink<(String, String, String)>) {
                if let Some(mut rx) = oxide_core::ffi::logger::setup_log_stream() {
                    while let Some(log) = rx.recv().await {
                        let _ = sink.add(log);
                    }
                }
            }

            #[flutter_rust_bridge::frb]
            pub async fn dispatch(
                engine: &std::sync::Arc<#engine_ident>,
                action: #action_ty,
            ) -> Result<#snapshot_ident, oxide_core::OxideError> {
                let snapshot = engine.dispatch(action).await?;
                Ok(snapshot)
            }

            #[flutter_rust_bridge::frb]
            pub async fn current(
                engine: &std::sync::Arc<#engine_ident>,
            ) -> #snapshot_ident {
                engine.current().await
            }

            #[flutter_rust_bridge::frb]
            pub async fn state_stream(
                engine: &std::sync::Arc<#engine_ident>,
                sink: crate::frb_generated::StreamSink<#snapshot_ident>,
            ) {
                let mut rx = engine.subscribe();
                let _ = sink.add(rx.borrow().clone().into());
                loop {
                    if rx.changed().await.is_err() {
                        break;
                    }
                    let _ = sink.add(rx.borrow().clone().into());
                }
            }
        }
    } else {
        quote! {}
    };

    let engine_extra_attrs = if cfg!(feature = "frb") && include_frb {
        quote!(#[flutter_rust_bridge::frb(opaque)])
    } else {
        quote!()
    };

    let engine_method_ignore_attr = if cfg!(feature = "frb") && include_frb {
        quote!(#[flutter_rust_bridge::frb(ignore)])
    } else {
        quote! {}
    };

    let engine_new_result_ty = if cfg!(feature = "frb") && include_frb {
        quote!(Result<Self, oxide_core::OxideError>)
    } else {
        quote!(oxide_core::CoreResult<Self>)
    };

    let engine_dispatch_result_ty = if cfg!(feature = "frb") && include_frb {
        quote!(Result<#snapshot_ident, oxide_core::OxideError>)
    } else {
        quote!(oxide_core::CoreResult<#snapshot_ident>)
    };

    let engine_persistence_encode_result_ty = if cfg!(feature = "frb") && include_frb {
        quote!(Result<Vec<u8>, oxide_core::OxideError>)
    } else {
        quote!(oxide_core::CoreResult<Vec<u8>>)
    };

    let engine_persistence_decode_result_ty = if cfg!(feature = "frb") && include_frb {
        quote!(Result<#state_ty, oxide_core::OxideError>)
    } else {
        quote!(oxide_core::CoreResult<#state_ty>)
    };

    let engine_persistence_tokens = if cfg!(feature = "state-persistence") {
        quote! {
            #engine_method_ignore_attr
            pub async fn encode_current_state(&self) -> #engine_persistence_encode_result_ty
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                let snapshot = self.inner.current().await;
                oxide_core::persistence::encode(&snapshot.state)
            }

            #engine_method_ignore_attr
            pub fn encode_state_value(value: &#state_ty) -> #engine_persistence_encode_result_ty
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                oxide_core::persistence::encode(value)
            }

            #engine_method_ignore_attr
            pub fn decode_state_value(bytes: &[u8]) -> #engine_persistence_decode_result_ty
            where
                #state_ty: oxide_core::serde::de::DeserializeOwned,
            {
                oxide_core::persistence::decode(bytes)
            }
        }
    } else {
        quote! {}
    };

    let core_snapshot_ty = if let Some(state_slice_ty) = state_slice_ty.as_ref() {
        quote!(oxide_core::StateSnapshot<#state_ty, #state_slice_ty>)
    } else {
        quote!(oxide_core::StateSnapshot<#state_ty>)
    };

    let snapshot_struct_tokens = if uses_sliced_updates {
        let state_slice_ty = state_slice_ty
            .as_ref()
            .expect("uses_sliced_updates implies state_slice_ty is Some");
        quote! {
            pub struct #snapshot_ident {
                pub revision: u64,
                pub state: #state_ty,
                pub slices: ::std::vec::Vec<#state_slice_ty>,
            }
        }
    } else {
        quote! {
            pub struct #snapshot_ident {
                pub revision: u64,
                pub state: #state_ty,
            }
        }
    };

    let snapshot_from_tokens = if uses_sliced_updates {
        quote! {
            impl From<#core_snapshot_ty> for #snapshot_ident {
                fn from(value: #core_snapshot_ty) -> Self {
                    Self {
                        revision: value.revision,
                        state: value.state,
                        slices: value.slices,
                    }
                }
            }
        }
    } else {
        quote! {
            impl From<#core_snapshot_ty> for #snapshot_ident {
                fn from(value: #core_snapshot_ty) -> Self {
                    Self {
                        revision: value.revision,
                        state: value.state,
                    }
                }
            }
        }
    };

    quote! {
        #item_impl

        #marker_item

        #sliced_state_assert

        #engine_extra_attrs
        pub struct #engine_ident {
            inner: #core_engine_type,
        }

        impl #engine_ident {
            #engine_method_ignore_attr
            pub async fn new() -> #engine_new_result_ty {
                let inner = (#engine_init_default)?;
                Ok(Self { inner })
            }

            #engine_method_ignore_attr
            pub async fn dispatch(&self, action: #action_ty) -> #engine_dispatch_result_ty {
                let snapshot = self.inner.dispatch(action).await?;
                Ok(snapshot.into())
            }

            #engine_method_ignore_attr
            pub async fn current(&self) -> #snapshot_ident {
                let snapshot = self.inner.current().await;
                snapshot.into()
            }

            #engine_method_ignore_attr
            pub fn subscribe(&self) -> ::oxide_core::tokio::sync::watch::Receiver<#core_snapshot_ty> {
                self.inner.subscribe()
            }

            #engine_persistence_tokens
        }

        #snapshot_struct_tokens

        #snapshot_from_tokens

        #frb_tokens
    }
}
