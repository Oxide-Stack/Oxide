use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ImplItem, ItemImpl, Token};

use crate::meta::{ReducerMeta, push_meta_doc};

pub(crate) struct ReducerArgs {
    pub(crate) engine_ident: Ident,
    pub(crate) snapshot_ident: Ident,
    pub(crate) initial_state: syn::Expr,
    pub(crate) reducer_expr: Option<syn::Expr>,
    pub(crate) include_frb: bool,
    pub(crate) persist_key: Option<syn::LitStr>,
    pub(crate) persist_min_interval_ms: Option<u64>,
    pub(crate) tokio_handle_expr: Option<syn::Expr>,
    pub(crate) tokio_runtime_expr: Option<syn::Expr>,
}

impl Parse for ReducerArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut engine_ident: Option<Ident> = None;
        let mut snapshot_ident: Option<Ident> = None;
        let mut initial_state: Option<syn::Expr> = None;
        let mut reducer_expr: Option<syn::Expr> = None;
        let mut include_frb = true;
        let mut persist_key: Option<syn::LitStr> = None;
        let mut persist_min_interval_ms: Option<u64> = None;
        let mut tokio_handle_expr: Option<syn::Expr> = None;
        let mut tokio_runtime_expr: Option<syn::Expr> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_string = key.to_string();

            match key_string.as_str() {
                "engine" => {
                    input.parse::<Token![=]>()?;
                    engine_ident = Some(input.parse()?);
                }
                "snapshot" => {
                    input.parse::<Token![=]>()?;
                    snapshot_ident = Some(input.parse()?);
                }
                "initial" => {
                    input.parse::<Token![=]>()?;
                    initial_state = Some(input.parse()?);
                }
                "reducer" => {
                    input.parse::<Token![=]>()?;
                    reducer_expr = Some(input.parse()?);
                }
                "no_frb" => {
                    include_frb = false;
                }
                "persist" => {
                    input.parse::<Token![=]>()?;
                    persist_key = Some(input.parse()?);
                }
                "persist_min_interval_ms" => {
                    input.parse::<Token![=]>()?;
                    let lit: syn::LitInt = input.parse()?;
                    persist_min_interval_ms = Some(lit.base10_parse()?);
                }
                "tokio_handle" => {
                    input.parse::<Token![=]>()?;
                    tokio_handle_expr = Some(input.parse()?);
                }
                "tokio_runtime" => {
                    input.parse::<Token![=]>()?;
                    tokio_runtime_expr = Some(input.parse()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown #[reducer] argument (expected `engine = Name`, `snapshot = Name`, `initial = <expr>`, optional `reducer = <expr>`, optional `no_frb`, optional `persist = \"key\"`, optional `persist_min_interval_ms = 200`, optional `tokio_handle = <expr>`, optional `tokio_runtime = <expr>`)",
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        if tokio_handle_expr.is_some() && tokio_runtime_expr.is_some() {
            return Err(syn::Error::new(
                input.span(),
                "#[reducer] cannot specify both `tokio_handle` and `tokio_runtime`",
            ));
        }

        Ok(Self {
            engine_ident: engine_ident.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `engine = <Name>`")
            })?,
            snapshot_ident: snapshot_ident.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `snapshot = <Name>`")
            })?,
            initial_state: initial_state.ok_or_else(|| {
                syn::Error::new(input.span(), "#[reducer] is missing `initial = <expr>`")
            })?,
            reducer_expr,
            include_frb,
            persist_key,
            persist_min_interval_ms,
            tokio_handle_expr,
            tokio_runtime_expr,
        })
    }
}

fn type_path_last_segment(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn is_reducer_trait(trait_path: &syn::Path) -> bool {
    trait_path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Reducer")
}

fn impl_reducer_ident(item_impl: &ItemImpl) -> syn::Result<Ident> {
    let self_ty = &*item_impl.self_ty;
    match self_ty {
        syn::Type::Path(p) => p
            .path
            .segments
            .last()
            .map(|segment| segment.ident.clone())
            .ok_or_else(|| syn::Error::new_spanned(self_ty, "reducer type must be a path type")),
        other => Err(syn::Error::new_spanned(
            other,
            "reducer type must be a path type",
        )),
    }
}

fn impl_assoc_type(item_impl: &ItemImpl, name: &str) -> Option<syn::Type> {
    item_impl.items.iter().find_map(|item| match item {
        ImplItem::Type(t) if t.ident == name => Some(t.ty.clone()),
        _ => None,
    })
}

fn find_impl_fn<'a>(item_impl: &'a ItemImpl, name: &str) -> Option<&'a syn::ImplItemFn> {
    item_impl.items.iter().find_map(|item| match item {
        ImplItem::Fn(f) if f.sig.ident == name => Some(f),
        _ => None,
    })
}

fn validate_init_sig(item_fn: &syn::ImplItemFn) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.fn_token,
            "`init` must not be async",
        ));
    }
    if item_fn.sig.inputs.len() != 2 {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            "`init` must take exactly 2 arguments: `&mut self` and `UnboundedSender<SideEffect>`",
        ));
    }
    let mut inputs = item_fn.sig.inputs.iter();
    let first = inputs.next().expect("first arg");
    let second = inputs.next().expect("second arg");

    match first {
        syn::FnArg::Receiver(r) if r.mutability.is_some() => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "`init` first argument must be `&mut self`",
            ));
        }
    }

    match second {
        syn::FnArg::Typed(_) => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "`init` second argument must be `UnboundedSender<SideEffect>`",
            ));
        }
    }

    if !matches!(item_fn.sig.output, syn::ReturnType::Default) {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.output,
            "`init` must not return a value",
        ));
    }

    Ok(())
}

fn return_is_core_result_state_change(output: &syn::ReturnType) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(last_segment) if last_segment.ident == "CoreResult" => {
                    match &last_segment.arguments {
                        syn::PathArguments::AngleBracketed(args) => {
                            if args.args.len() != 1 {
                                return false;
                            }
                            match args.args.first().unwrap() {
                                syn::GenericArgument::Type(syn::Type::Path(inner)) => inner
                                    .path
                                    .segments
                                    .last()
                                    .is_some_and(|seg| seg.ident == "StateChange"),
                                _ => false,
                            }
                        }
                        _ => false,
                    }
                }
                _ => false,
            },
            _ => false,
        },
    }
}

fn validate_reduce_like_sig(item_fn: &syn::ImplItemFn, name: &str) -> syn::Result<()> {
    if item_fn.sig.asyncness.is_some() {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.fn_token,
            format!("`{name}` must not be async"),
        ));
    }
    if item_fn.sig.inputs.len() != 3 {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.inputs,
            format!(
                "`{name}` must take exactly 3 arguments: `&mut self`, `&mut State`, and the input value"
            ),
        ));
    }

    let mut inputs = item_fn.sig.inputs.iter();
    let first = inputs.next().expect("first arg");
    let second = inputs.next().expect("second arg");
    let third = inputs.next().expect("third arg");

    match first {
        syn::FnArg::Receiver(r) if r.mutability.is_some() => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` first argument must be `&mut self`"),
            ));
        }
    }

    match second {
        syn::FnArg::Typed(pat_ty) => match &*pat_ty.ty {
            syn::Type::Reference(r) if r.mutability.is_some() => {}
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    format!("`{name}` second argument must be `&mut State`"),
                ));
            }
        },
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` second argument must be `&mut State`"),
            ));
        }
    }

    match third {
        syn::FnArg::Typed(_) => {}
        other => {
            return Err(syn::Error::new_spanned(
                other,
                format!("`{name}` third argument must be the input value (not `self`)"),
            ));
        }
    }

    if !return_is_core_result_state_change(&item_fn.sig.output) {
        return Err(syn::Error::new_spanned(
            &item_fn.sig.output,
            format!("`{name}` must return `oxide_core::CoreResult<oxide_core::StateChange>`"),
        ));
    }

    Ok(())
}

pub(crate) fn expand_reducer_impl(
    args: ReducerArgs,
    item_impl: ItemImpl,
) -> proc_macro2::TokenStream {
    let ReducerArgs {
        engine_ident,
        snapshot_ident,
        initial_state,
        reducer_expr,
        include_frb,
        persist_key,
        persist_min_interval_ms,
        tokio_handle_expr,
        tokio_runtime_expr,
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
                "Reducer impl is missing `fn init(&mut self, sideeffect_tx: UnboundedSender<Self::SideEffect>)`",
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

    let mut marker_item: syn::ItemStruct = syn::parse_quote!(struct #marker_ident;);
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
                )
            )
        } else {
            quote!(oxide_core::ReducerEngine::<#reducer_ident>::new(#reducer_init_expr, #initial_state))
        }
    } else {
        quote!(oxide_core::ReducerEngine::<#reducer_ident>::new(#reducer_init_expr, #initial_state))
    };

    let engine_init = if let Some(tokio_handle_expr) = &tokio_handle_expr {
        if cfg!(feature = "state-persistence") {
            if let Some(persist_key) = &persist_key {
                let ms = persist_min_interval_ms.unwrap_or(200);
                quote!(
                    oxide_core::ReducerEngine::<#reducer_ident>::new_persistent_with_handle(
                        #reducer_init_expr,
                        #initial_state,
                        oxide_core::persistence::PersistenceConfig {
                            key: #persist_key.to_string(),
                            min_interval: ::std::time::Duration::from_millis(#ms),
                        },
                        #tokio_handle_expr,
                    )
                )
            } else {
                quote!(
                    oxide_core::ReducerEngine::<#reducer_ident>::new_with_handle(
                        #reducer_init_expr,
                        #initial_state,
                        #tokio_handle_expr,
                    )
                )
            }
        } else {
            quote!(
                oxide_core::ReducerEngine::<#reducer_ident>::new_with_handle(
                    #reducer_init_expr,
                    #initial_state,
                    #tokio_handle_expr,
                )
            )
        }
    } else if let Some(tokio_runtime_expr) = &tokio_runtime_expr {
        if cfg!(feature = "state-persistence") {
            if let Some(persist_key) = &persist_key {
                let ms = persist_min_interval_ms.unwrap_or(200);
                quote!(
                    oxide_core::ReducerEngine::<#reducer_ident>::new_persistent_with_runtime(
                        #reducer_init_expr,
                        #initial_state,
                        oxide_core::persistence::PersistenceConfig {
                            key: #persist_key.to_string(),
                            min_interval: ::std::time::Duration::from_millis(#ms),
                        },
                        #tokio_runtime_expr,
                    )
                )
            } else {
                quote!(
                    oxide_core::ReducerEngine::<#reducer_ident>::new_with_runtime(
                        #reducer_init_expr,
                        #initial_state,
                        #tokio_runtime_expr,
                    )
                )
            }
        } else {
            quote!(
                oxide_core::ReducerEngine::<#reducer_ident>::new_with_runtime(
                    #reducer_init_expr,
                    #initial_state,
                    #tokio_runtime_expr,
                )
            )
        }
    } else {
        engine_init_default
    };

    let engine_init_with_handle = if cfg!(feature = "state-persistence") {
        if let Some(persist_key) = &persist_key {
            let ms = persist_min_interval_ms.unwrap_or(200);
            quote!(
                oxide_core::ReducerEngine::<#reducer_ident>::new_persistent_with_handle(
                    #reducer_init_expr,
                    #initial_state,
                    oxide_core::persistence::PersistenceConfig {
                        key: #persist_key.to_string(),
                        min_interval: ::std::time::Duration::from_millis(#ms),
                    },
                    handle,
                )
            )
        } else {
            quote!(oxide_core::ReducerEngine::<#reducer_ident>::new_with_handle(#reducer_init_expr, #initial_state, handle))
        }
    } else {
        quote!(oxide_core::ReducerEngine::<#reducer_ident>::new_with_handle(#reducer_init_expr, #initial_state, handle))
    };

    let engine_init_with_runtime = if cfg!(feature = "state-persistence") {
        if let Some(persist_key) = &persist_key {
            let ms = persist_min_interval_ms.unwrap_or(200);
            quote!(
                oxide_core::ReducerEngine::<#reducer_ident>::new_persistent_with_runtime(
                    #reducer_init_expr,
                    #initial_state,
                    oxide_core::persistence::PersistenceConfig {
                        key: #persist_key.to_string(),
                        min_interval: ::std::time::Duration::from_millis(#ms),
                    },
                    runtime,
                )
            )
        } else {
            quote!(oxide_core::ReducerEngine::<#reducer_ident>::new_with_runtime(#reducer_init_expr, #initial_state, runtime))
        }
    } else {
        quote!(oxide_core::ReducerEngine::<#reducer_ident>::new_with_runtime(#reducer_init_expr, #initial_state, runtime))
    };

    let frb_tokens = if cfg!(feature = "frb") && include_frb {
        quote! {
            #[flutter_rust_bridge::frb]
            pub fn create_engine() -> std::sync::Arc<#engine_ident> {
                std::sync::Arc::new(#engine_ident::new())
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

    let engine_persistence_tokens = if cfg!(feature = "state-persistence") {
        quote! {
            pub async fn encode_current_state(&self) -> oxide_core::CoreResult<Vec<u8>>
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                let snapshot = self.inner.current().await;
                oxide_core::persistence::encode(&snapshot.state)
            }

            pub fn encode_state_value(value: &#state_ty) -> oxide_core::CoreResult<Vec<u8>>
            where
                #state_ty: oxide_core::serde::Serialize,
            {
                oxide_core::persistence::encode(value)
            }

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

    let engine_method_ignore_attr = if cfg!(feature = "frb") && include_frb {
        quote!(#[flutter_rust_bridge::frb(ignore)])
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
            pub fn new() -> Self {
                Self {
                    inner: #engine_init,
                }
            }

            #engine_method_ignore_attr
            pub fn new_with_handle(handle: ::oxide_core::tokio::runtime::Handle) -> Self {
                Self {
                    inner: #engine_init_with_handle,
                }
            }

            #engine_method_ignore_attr
            pub fn new_with_runtime(runtime: &::oxide_core::tokio::runtime::Runtime) -> Self {
                Self {
                    inner: #engine_init_with_runtime,
                }
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
