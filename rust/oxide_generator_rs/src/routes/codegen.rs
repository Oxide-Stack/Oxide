use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::LitStr;

use crate::routes::args::RoutesArgs;
use crate::routes::model::RouteMeta;

pub(super) fn generate_navigation_module(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
    // determine a candidate initial route (first route with no fields)
    let initial_route = routes.iter().find(|r| r.fields.is_empty()).map(|route| {
        let ident = syn::Ident::new(&route.rust_type, Span::call_site());
        quote! {
            crate::routes::#ident {}
        }
    });

    let start_body = if let Some(initial_route) = initial_route {
        quote! {
            static STARTED: ::std::sync::OnceLock<()> = ::std::sync::OnceLock::new();
            STARTED.get_or_init(|| {
                if let Ok(runtime) = oxide_core::navigation_runtime() {
                    let _ = runtime.push(#initial_route);
                }
            });
            Ok(())
        }
    } else {
        quote! {
            Ok(())
        }
    };

    Ok(quote! {
        pub mod navigation {
            pub mod runtime {
                /// Initializes the Oxide navigation runtime singleton.
                ///
                /// reducers/effects may emit navigation intents, and the Dart runtime
                /// must be able to subscribe to those commands.
                ///
                /// this only ensures the global navigation runtime exists.
                pub(crate) fn init() -> oxide_core::CoreResult<()> {
                    oxide_core::init_navigation()?;
                    Ok(())
                }

                /// Starts navigation bootstrap exactly once.
                ///
                /// initial-route emission must be explicit and idempotent so route
                /// synchronization from Dart does not re-trigger startup pushes.
                ///
                /// guards the generated initial push behind a process-local `OnceLock`.
                pub(crate) fn start() -> oxide_core::CoreResult<()> {
                    init()?;
                    #start_body
                }
            }
        }
    })
}

pub(super) fn generate_oxide_init_module() -> syn::Result<TokenStream2> {
    Ok(quote! {
        pub mod oxide {
            pub mod init {
                #[flutter_rust_bridge::frb(init)]
                pub fn init_oxide() -> Result<(), oxide_core::OxideError> {
                    flutter_rust_bridge::setup_default_user_utils();
                    fn thread_pool() -> oxide_core::runtime::ThreadPool {
                        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
                    }
                    oxide_core::init_from_frb(thread_pool)
                }
            }
        }
    })
}

pub(super) fn generate_route_context_types() -> syn::Result<TokenStream2> {
    Ok(quote! {
        #[flutter_rust_bridge::frb(non_opaque)]
        #[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
        pub enum RouteOperation {
            Push,
            Pop,
            PopUntil,
            Reset,
            Sync,
        }

        #[flutter_rust_bridge::frb(non_opaque)]
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct RouteInitContext {
            pub route: RoutePayload,
        }

        #[flutter_rust_bridge::frb(non_opaque)]
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct RouteUpdateContext {
            pub operation: RouteOperation,
            pub route: Option<RoutePayload>,
            pub result_json: Option<String>,
            pub arguments_json: Option<String>,
            pub first_push: bool,
        }
    })
}

pub(super) fn generate_navigation_bridge_module(args: &RoutesArgs) -> syn::Result<TokenStream2> {
    let init_hook_call = if let Some(path) = &args.init {
        quote! {
            super::#path(ctx)
        }
    } else {
        quote! {
            Ok(())
        }
    };

    let on_change_hook_call = if let Some(path) = &args.on_route_change {
        quote! {
            super::#path(ctx)
        }
    } else {
        quote! {
            Ok(())
        }
    };

    Ok(quote! {
        pub mod oxide_navigation {
            pub use crate::routes::{RouteKind, RoutePayload};

            #[flutter_rust_bridge::frb]
            pub async fn init_navigation() -> Result<(), oxide_core::OxideError> {
                crate::navigation::runtime::start()?;
                Ok(())
            }

            #[flutter_rust_bridge::frb]
            pub async fn oxide_nav_commands_stream(
                sink: crate::frb_generated::StreamSink<OxideNavCommand>,
            ) -> Result<(), oxide_core::OxideError> {
                crate::navigation::runtime::init()?;
                let runtime = oxide_core::navigation_runtime()?;
                let mut rx = runtime.subscribe_commands()?;

                while let Some(cmd) = rx.recv().await {
                    let out = map_nav_command(cmd)?;
                    __oxide_nav_require_fresh_frb_bindings(&sink, out);
                }

                Ok(())
            }

            #[flutter_rust_bridge::frb]
            pub async fn oxide_nav_emit_result(
                ticket: String,
                result_json: String,
            ) -> Result<(), oxide_core::OxideError> {
                crate::navigation::runtime::init()?;
                let runtime = oxide_core::navigation_runtime()?;
                let value: ::serde_json::Value = ::serde_json::from_str(&result_json).map_err(|e| {
                    oxide_core::OxideError::Validation {
                        message: format!("invalid navigation result JSON: {e}"),
                    }
                })?;
                let _ = runtime.emit_result(&ticket, value).await;
                Ok(())
            }

            #[flutter_rust_bridge::frb]
            pub fn oxide_nav_set_current_route(route: Option<RoutePayload>) -> Result<(), oxide_core::OxideError> {
                crate::navigation::runtime::init()?;
                let runtime = oxide_core::navigation_runtime()?;
                let Some(route) = route else {
                    runtime.set_current_route(None);
                    return Ok(());
                };

                let kind = route.kind().as_str().to_string();
                let payload = route.payload_json()?;
                runtime.set_current_route(Some(oxide_core::navigation::NavRoute {
                    kind,
                    payload,
                    extras: None,
                }));
                Ok(())
            }

            #[flutter_rust_bridge::frb]
            pub fn oxide_nav_route_update(
                update: crate::routes::RouteUpdateContext,
            ) -> Result<(), oxide_core::OxideError> {
                crate::navigation::runtime::init()?;
                let runtime = oxide_core::navigation_runtime()?;
                let crate::routes::RouteUpdateContext {
                    operation,
                    route,
                    result_json,
                    arguments_json,
                    first_push,
                } = update.clone();

                if first_push {
                    if let Some(route_payload) = route.clone() {
                        __oxide_nav_call_init(crate::routes::RouteInitContext { route: route_payload })?;
                    }
                }
                __oxide_nav_call_on_route_change(update)?;

                let route_for_runtime = match route {
                    Some(route) => {
                        let kind = route.kind().as_str().to_string();
                        let payload = route.payload_json()?;
                        Some(oxide_core::navigation::NavRoute {
                            kind,
                            payload,
                            extras: None,
                        })
                    }
                    None => None,
                };
                let result = parse_optional_json(result_json, "result_json")?;
                let arguments = parse_optional_json(arguments_json, "arguments_json")?;

                runtime.set_route_update(oxide_core::navigation::RouteUpdate {
                    operation: map_route_operation(operation),
                    route: route_for_runtime.clone(),
                    result,
                    arguments,
                    first_push,
                });
                runtime.set_current_route(route_for_runtime);
                Ok(())
            }

            #[flutter_rust_bridge::frb(non_opaque)]
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
            pub enum OxideNavCommand {
                Push {
                    route: RoutePayload,
                    ticket: Option<String>,
                },
                Pop {
                    result_json: Option<String>,
                },
                PopUntil {
                    kind: String,
                },
                Reset {
                    routes: Vec<RoutePayload>,
                },
            }

            /// Compile-time guardrail for stale FRB bindings.
            ///
            /// If this fails with trait-bound errors around `IntoIntoDart` or
            /// `StreamSink<OxideNavCommand>::add`, regenerate FRB bindings from your
            /// Flutter app root:
            ///
            /// `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml`
            #[inline(always)]
            fn __oxide_nav_require_fresh_frb_bindings(
                sink: &crate::frb_generated::StreamSink<OxideNavCommand>,
                command: OxideNavCommand,
            )
            where
                OxideNavCommand: flutter_rust_bridge::IntoIntoDart<OxideNavCommand>,
            {
                let _ = sink.add(command);
            }

            fn map_nav_command(cmd: oxide_core::navigation::NavCommand) -> oxide_core::CoreResult<OxideNavCommand> {
                match cmd {
                    oxide_core::navigation::NavCommand::Push { route, ticket } => {
                        Ok(OxideNavCommand::Push {
                            route: nav_route_to_route_payload(route)?,
                            ticket,
                        })
                    }
                    oxide_core::navigation::NavCommand::Pop { result } => Ok(OxideNavCommand::Pop {
                        result_json: result.map(|v| v.to_string()),
                    }),
                    oxide_core::navigation::NavCommand::PopUntil { kind } => {
                        Ok(OxideNavCommand::PopUntil { kind })
                    }
                    oxide_core::navigation::NavCommand::Reset { routes } => {
                        let mut out = Vec::with_capacity(routes.len());
                        for r in routes {
                            out.push(nav_route_to_route_payload(r)?);
                        }
                        Ok(OxideNavCommand::Reset { routes: out })
                    }
                }
            }

            fn nav_route_to_route_payload(
                route: ::oxide_core::navigation::NavRoute,
            ) -> ::oxide_core::CoreResult<RoutePayload> {
                let ::oxide_core::navigation::NavRoute { kind, payload, .. } = route;
                RoutePayload::from_kind_and_payload(&kind, payload)
            }

            fn map_route_operation(
                operation: crate::routes::RouteOperation,
            ) -> oxide_core::navigation::RouteOperation {
                match operation {
                    crate::routes::RouteOperation::Push => oxide_core::navigation::RouteOperation::Push,
                    crate::routes::RouteOperation::Pop => oxide_core::navigation::RouteOperation::Pop,
                    crate::routes::RouteOperation::PopUntil => oxide_core::navigation::RouteOperation::PopUntil,
                    crate::routes::RouteOperation::Reset => oxide_core::navigation::RouteOperation::Reset,
                    crate::routes::RouteOperation::Sync => oxide_core::navigation::RouteOperation::Sync,
                }
            }

            fn parse_optional_json(
                value: Option<String>,
                field_name: &str,
            ) -> Result<Option<::serde_json::Value>, oxide_core::OxideError> {
                let Some(value) = value else {
                    return Ok(None);
                };
                let parsed = ::serde_json::from_str::<::serde_json::Value>(&value).map_err(|e| {
                    oxide_core::OxideError::Validation {
                        message: format!("invalid {field_name} JSON: {e}"),
                    }
                })?;
                Ok(Some(parsed))
            }

            fn __oxide_nav_call_init(ctx: crate::routes::RouteInitContext) -> oxide_core::CoreResult<()> {
                static INIT_HOOK_TRIGGERED: ::std::sync::OnceLock<()> = ::std::sync::OnceLock::new();
                if INIT_HOOK_TRIGGERED.get().is_some() {
                    return Ok(());
                }
                if INIT_HOOK_TRIGGERED.set(()).is_err() {
                    return Ok(());
                }
                #init_hook_call
            }

            fn __oxide_nav_call_on_route_change(
                ctx: crate::routes::RouteUpdateContext,
            ) -> oxide_core::CoreResult<()> {
                #on_change_hook_call
            }
        }
    })
}

pub(super) fn generate_route_kind_enum(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
    let variants: Vec<Ident> = routes
        .iter()
        .map(|r| Ident::new(&r.kind, Span::call_site()))
        .collect();
    let kind_strings: Vec<String> = routes.iter().map(|r| r.kind.clone()).collect();

    Ok(quote! {
        #[flutter_rust_bridge::frb(non_opaque)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum RouteKind {
            #( #variants, )*
        }

        impl RouteKind {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #( Self::#variants => #kind_strings, )*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    #( #kind_strings => Some(Self::#variants), )*
                    _ => None,
                }
            }
        }

        impl ::oxide_core::navigation::OxideRouteKind for RouteKind {
            fn as_str(&self) -> &'static str {
                self.as_str()
            }
        }
    })
}

pub(super) fn generate_route_payload_enum(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
    let variants: Vec<Ident> = routes
        .iter()
        .map(|r| Ident::new(&r.kind, Span::call_site()))
        .collect();
    let tys: Vec<Ident> = routes
        .iter()
        .map(|r| Ident::new(&r.rust_type, Span::call_site()))
        .collect();

    Ok(quote! {
        #[flutter_rust_bridge::frb(non_opaque)]
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub enum RoutePayload {
            #( #variants(#tys), )*
        }
    })
}

pub(super) fn generate_payload_helpers(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
    let variants: Vec<Ident> = routes
        .iter()
        .map(|r| Ident::new(&r.kind, Span::call_site()))
        .collect();
    let tys: Vec<Ident> = routes
        .iter()
        .map(|r| Ident::new(&r.rust_type, Span::call_site()))
        .collect();
    let kind_strings: Vec<LitStr> = routes
        .iter()
        .map(|r| LitStr::new(&r.kind, Span::call_site()))
        .collect();

    Ok(quote! {
        impl RoutePayload {
            pub fn kind(&self) -> RouteKind {
                match self {
                    #( Self::#variants(_) => RouteKind::#variants, )*
                }
            }

            pub(crate) fn payload_json(&self) -> oxide_core::CoreResult<::serde_json::Value> {
                match self {
                    #( Self::#variants(v) => ::serde_json::to_value(v).map_err(|e| oxide_core::OxideError::Internal {
                        message: format!("failed to serialize route payload for kind {}: {e}", #kind_strings),
                    }), )*
                }
            }

            pub(crate) fn from_kind_and_payload(
                kind: &str,
                payload: ::serde_json::Value,
            ) -> oxide_core::CoreResult<Self> {
                let kind = RouteKind::from_str(kind).ok_or_else(|| oxide_core::OxideError::Validation {
                    message: format!("unknown route kind: {kind}"),
                })?;
                match kind {
                    #( RouteKind::#variants => {
                        let v = ::serde_json::from_value::<#tys>(payload).map_err(|e| oxide_core::OxideError::Validation {
                            message: format!("failed to decode route payload for kind {}: {e}", #kind_strings),
                        })?;
                        Ok(Self::#variants(v))
                    } )*
                }
            }
        }

        #( impl From<#tys> for RoutePayload {
            fn from(v: #tys) -> Self { Self::#variants(v) }
        } )*

        impl ::oxide_core::navigation::OxideRoutePayload for RoutePayload {
            type Kind = RouteKind;

            fn kind(&self) -> Self::Kind {
                self.kind()
            }
        }

        #( impl ::oxide_core::navigation::OxideRoute for #tys {
            type Payload = RoutePayload;

            fn into_payload(self) -> Self::Payload {
                RoutePayload::from(self)
            }
        } )*
    })
}
