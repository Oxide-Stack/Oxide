use quote::{format_ident, quote};
use syn::{ImplItem, ItemImpl};

// Token emission for `#[reducer(...)]`.
//
// Why: keep all generated Rust/FRB surface construction in one place so it can
// be reviewed as a coherent output contract.
use crate::meta::{ReducerMeta, push_meta_doc};
use crate::reducer::args::ReducerArgs;
use crate::reducer::sliced_usage::fn_uses_sliced_state_change;
use crate::reducer::validate::{
    find_impl_fn, impl_assoc_type, impl_reducer_ident, is_reducer_trait, type_path_last_segment,
    validate_init_sig, validate_reduce_like_sig,
};

pub(crate) fn expand_reducer_impl(
    args: ReducerArgs,
    mut item_impl: ItemImpl,
) -> proc_macro2::TokenStream {
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
                "Reducer impl is missing `fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange<...>>`",
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
                "Reducer impl is missing `fn effect(&mut self, state: &mut Self::State, effect: Self::SideEffect) -> CoreResult<StateChange<...>>`",
            )
            .to_compile_error();
        }
    };
    if let Err(e) = validate_reduce_like_sig(effect_fn, "effect") {
        return e.to_compile_error();
    }

    let uses_sliced_updates =
        fn_uses_sliced_state_change(reduce_fn) || fn_uses_sliced_state_change(effect_fn);

    let sliced_state_assert = if uses_sliced_updates {
        quote! {
            const _: () = {
                fn _oxide_require_sliced_state<T: ::oxide_core::SlicedState>() {}
                let _ = _oxide_require_sliced_state::<#state_ty>;
            };
        }
    } else {
        quote!()
    };

    let state_slice_ty: Option<syn::Type> = if uses_sliced_updates {
        if include_frb {
            let Some(state_name) = type_path_last_segment(&state_ty) else {
                return syn::Error::new_spanned(
                    &state_ty,
                    "state type must be a path type to enable sliced updates",
                )
                .to_compile_error();
            };
            let slice_ident = quote::format_ident!("{state_name}Slice");
            Some(syn::parse_quote!(#slice_ident))
        } else {
            Some(syn::parse_quote!(<#state_ty as ::oxide_core::SlicedState>::StateSlice))
        }
    } else {
        None
    };

    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        let Some((_, trait_path, _)) = item_impl.trait_.as_mut() else {
            return syn::Error::new_spanned(
                &item_impl.impl_token,
                "#[reducer(...)] must be applied to an `impl oxide_core::Reducer for <Type>` block",
            )
            .to_compile_error();
        };
        if let Some(last) = trait_path.segments.last_mut() {
            let args: syn::AngleBracketedGenericArguments = syn::parse_quote!(<#state_slice_ty>);
            last.arguments = syn::PathArguments::AngleBracketed(args);
        }
    }

    let has_infer_slices = item_impl.items.iter().any(|item| match item {
        ImplItem::Fn(f) if f.sig.ident == "infer_slices" => true,
        _ => false,
    });
    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        if !has_infer_slices {
            let body: syn::Expr = if include_frb {
                syn::parse_quote!(Self::State::infer_slices_impl(before, after))
            } else {
                syn::parse_quote!(<Self::State as ::oxide_core::SlicedState>::infer_slices(before, after))
            };
            item_impl.items.push(syn::parse_quote!(
                fn infer_slices(
                    &self,
                    before: &Self::State,
                    after: &Self::State,
                ) -> ::std::vec::Vec<#state_slice_ty> {
                    #body
                }
            ));
        }
    }

    if let (true, Some(state_slice_ty)) = (uses_sliced_updates, state_slice_ty.as_ref()) {
        for item in item_impl.items.iter_mut() {
            let ImplItem::Fn(f) = item else {
                continue;
            };
            if f.sig.ident != "reduce" && f.sig.ident != "effect" {
                continue;
            }
            f.sig.output = syn::parse_quote!(
                -> ::oxide_core::CoreResult<::oxide_core::StateChange<#state_slice_ty>>
            );
        }
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
