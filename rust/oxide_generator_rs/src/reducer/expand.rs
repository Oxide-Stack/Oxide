use quote::{format_ident, quote};
use syn::ItemImpl;

// Token emission for `#[reducer(...)]`.
//
// Why: keep all generated Rust/FRB surface construction in one place so it can
// be reviewed as a coherent output contract.
use crate::meta::{ReducerMeta, push_meta_doc};
use crate::reducer::args::ReducerArgs;
use crate::reducer::validate::{
    find_impl_fn, impl_assoc_type, impl_reducer_ident, is_reducer_trait, type_path_last_segment,
    validate_init_sig, validate_reduce_like_sig,
};

pub(crate) fn expand_reducer_impl(args: ReducerArgs, item_impl: ItemImpl) -> proc_macro2::TokenStream {
    let ReducerArgs {
        engine_ident,
        snapshot_ident,
        initial_state,
        reducer_expr,
        include_frb,
        persist_key,
        persist_min_interval_ms,
    } = args;

    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return syn::Error::new_spanned(
            &item_impl.impl_token,
            "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
        )
        .to_compile_error();
    };
    if !is_reducer_trait(trait_path) {
        return syn::Error::new_spanned(
            trait_path,
            "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
        )
        .to_compile_error();
    }

    let reducer_ident = match impl_reducer_ident(&item_impl) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };

    let state_ty = match impl_assoc_type(&item_impl, "State") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `type State = ...;`",
            )
            .to_compile_error();
        }
    };
    let action_ty = match impl_assoc_type(&item_impl, "Action") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `type Action = ...;`",
            )
            .to_compile_error();
        }
    };
    let _sideeffect_ty = match impl_assoc_type(&item_impl, "SideEffect") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `type SideEffect = ...;`",
            )
            .to_compile_error();
        }
    };

    let init_fn = match find_impl_fn(&item_impl, "init") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>)`",
            )
            .to_compile_error();
        }
    };
    if let Err(e) = validate_init_sig(init_fn) {
        return e.to_compile_error();
    }

    let reduce_fn = match find_impl_fn(&item_impl, "reduce") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange>`",
            )
            .to_compile_error();
        }
    };
    if let Err(e) = validate_reduce_like_sig(reduce_fn, "reduce") {
        return e.to_compile_error();
    }

    let effect_fn = match find_impl_fn(&item_impl, "effect") {
        Some(v) => v,
        None => {
            return syn::Error::new_spanned(
                &item_impl.self_ty,
                "Reducer impl is missing `fn effect(&mut self, state: &mut Self::State, effect: Self::SideEffect) -> CoreResult<StateChange>`",
            )
            .to_compile_error();
        }
    };
    if let Err(e) = validate_reduce_like_sig(effect_fn, "effect") {
        return e.to_compile_error();
    }

    if persist_key.is_some() && !cfg!(feature = "state-persistence") {
        return quote! {
            compile_error!("oxide_generator_rs: reducer persistence requires enabling the `state-persistence` feature on oxide_generator_rs and oxide_core");
        };
    }

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

    let engine_init_default = if cfg!(feature = "state-persistence") {
        if let Some(persist_key) = &persist_key {
            let ms = persist_min_interval_ms.unwrap_or(200);
            quote!(
                oxide_core::ReducerEngine::<#reducer_ident>::new_persistent(
                    #reducer_init_expr,
                    #initial_state,
                    oxide_core::persistence::PersistenceConfig {
                        key: #persist_key.to_string(),
                        min_interval: ::std::time::Duration::from_millis(#ms),
                    },
                ).await
            )
        } else {
            quote!(oxide_core::ReducerEngine::<#reducer_ident>::new(#reducer_init_expr, #initial_state).await)
        }
    } else {
        quote!(oxide_core::ReducerEngine::<#reducer_ident>::new(#reducer_init_expr, #initial_state).await)
    };

    let frb_tokens = if cfg!(feature = "frb") && include_frb {
        quote! {
            #[flutter_rust_bridge::frb]
            pub async fn create_engine() -> Result<std::sync::Arc<#engine_ident>, oxide_core::OxideError> {
                Ok(std::sync::Arc::new(#engine_ident::new().await?))
            }

            #[flutter_rust_bridge::frb]
            pub fn dispose_engine(_engine: &std::sync::Arc<#engine_ident>) {}

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

    let engine_persistence_tokens = if cfg!(feature = "state-persistence") {
        quote! {
            #engine_method_ignore_attr
            pub async fn encode_current_state(&self) -> oxide_core::CoreResult<Vec<u8>>
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                let snapshot = self.inner.current().await;
                oxide_core::persistence::encode(&snapshot.state)
            }

            #engine_method_ignore_attr
            pub fn encode_state_value(value: &#state_ty) -> oxide_core::CoreResult<Vec<u8>>
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                oxide_core::persistence::encode(value)
            }

            #engine_method_ignore_attr
            pub fn decode_state_value(bytes: &[u8]) -> oxide_core::CoreResult<#state_ty>
            where
                #state_ty: oxide_core::serde::de::DeserializeOwned,
            {
                oxide_core::persistence::decode(bytes)
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #item_impl

        #marker_item

        #engine_extra_attrs
        pub struct #engine_ident {
            inner: oxide_core::ReducerEngine<#reducer_ident>,
        }

        impl #engine_ident {
            #engine_method_ignore_attr
            pub async fn new() -> oxide_core::CoreResult<Self> {
                let inner = (#engine_init_default)?;
                Ok(Self { inner })
            }

            #engine_method_ignore_attr
            pub async fn dispatch(&self, action: #action_ty) -> oxide_core::CoreResult<#snapshot_ident> {
                let snapshot = self.inner.dispatch(action).await?;
                Ok(snapshot.into())
            }

            #engine_method_ignore_attr
            pub async fn current(&self) -> #snapshot_ident {
                let snapshot = self.inner.current().await;
                snapshot.into()
            }

            #engine_method_ignore_attr
            pub fn subscribe(&self) -> ::oxide_core::tokio::sync::watch::Receiver<oxide_core::StateSnapshot<#state_ty>> {
                self.inner.subscribe()
            }

            #engine_persistence_tokens
        }

        pub struct #snapshot_ident {
            pub revision: u64,
            pub state: #state_ty,
        }

        impl From<oxide_core::StateSnapshot<#state_ty>> for #snapshot_ident {
            fn from(value: oxide_core::StateSnapshot<#state_ty>) -> Self {
                Self {
                    revision: value.revision,
                    state: value.state,
                }
            }
        }

        #frb_tokens
    }
}
