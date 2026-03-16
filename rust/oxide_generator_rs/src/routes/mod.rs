use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use serde::Serialize;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, LitStr, Token};
use syn::{Item, ItemImpl, ItemMod, ItemStruct, Type};

#[derive(Debug, Clone, Serialize)]
struct RouteFieldMeta {
    name: String,
    ty: String,
}

#[derive(Debug, Clone, Serialize)]
struct RouteMeta {
    kind: String,
    rust_type: String,
    path: Option<String>,
    return_type: String,
    extra_type: String,
    fields: Vec<RouteFieldMeta>,
}

#[derive(Debug, Clone, Serialize)]
struct RouteMetadataFile {
    crate_name: String,
    routes: Vec<RouteMeta>,
}

#[derive(Default)]
pub struct OxideRouteArgs {
    path: Option<LitStr>,
    return_type: Option<Type>,
    extra_type: Option<Type>,
}

impl Parse for OxideRouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = OxideRouteArgs::default();
        while !input.is_empty() {
            let key: Ident = input.call(Ident::parse_any)?;
            input.parse::<Token![=]>()?;
            match key.to_string().as_str() {
                "path" => {
                    args.path = Some(input.parse()?);
                }
                "return" => {
                    args.return_type = Some(input.parse()?);
                }
                "extra" => {
                    args.extra_type = Some(input.parse()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        key,
                        "unknown #[oxide_route] argument",
                    ));
                }
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(args)
    }
}

pub fn expand_oxide_route_struct(
    args: OxideRouteArgs,
    mut item_struct: ItemStruct,
) -> syn::Result<TokenStream2> {
    let path_value = args.path.as_ref().map(|p| p.value());
    let ident = item_struct.ident.clone();
    validate_oxide_route_struct(&item_struct)?;
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

#[derive(Default)]
struct RouteFieldArgs {
    kind: Option<String>,
    key: Option<LitStr>,
}

impl Parse for RouteFieldArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = RouteFieldArgs::default();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "kind" {
                let v: LitStr = input.parse()?;
                args.kind = Some(v.value());
            } else if key == "key" {
                args.key = Some(input.parse()?);
            } else {
                return Err(syn::Error::new_spanned(key, "unknown #[route] argument"));
            }
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(args)
    }
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

    let has_clone = has_derive_named(&item_struct.attrs, "Clone");
    if !has_clone {
        return Err(syn::Error::new_spanned(
            &item_struct.ident,
            "route structs must derive Clone (e.g. #[derive(Clone, ...)])",
        ));
    }

    let has_serialize = has_derive_named(&item_struct.attrs, "Serialize");
    let has_deserialize = has_derive_named(&item_struct.attrs, "Deserialize");
    if !has_serialize || !has_deserialize {
        return Err(syn::Error::new_spanned(
            &item_struct.ident,
            "route structs must derive serde::Serialize and serde::Deserialize (required for RoutePayload encoding)",
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

pub fn expand_routes_module(item_mod: ItemMod) -> syn::Result<TokenStream2> {
    let mod_ident = item_mod.ident.clone();

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into()));
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "unknown".into());

    let src_dir = manifest_dir.join("src");
    let routes_dir = src_dir.join(mod_ident.to_string());

    let mut files_to_scan = Vec::new();
    if routes_dir.is_dir() {
        files_to_scan.extend(discover_rs_files(&routes_dir)?);
    }

    let mut items = Vec::new();
    if let Some((_brace, inline_items)) = &item_mod.content {
        items.extend(inline_items.iter().cloned());
    } else {
        let mod_rs = routes_dir.join("mod.rs");
        let routes_rs = src_dir.join(format!("{mod_ident}.rs"));
        if mod_rs.is_file() {
            items.extend(parse_items_from_file(&mod_rs)?);
        } else if routes_rs.is_file() {
            items.extend(parse_items_from_file(&routes_rs)?);
        }
    }

    for file in files_to_scan {
        items.extend(parse_items_from_file(&file)?);
    }

    let routes = collect_routes(&items)?;
    emit_metadata_json(&crate_name, &routes, &manifest_dir)?;

    let kind_enum = generate_route_kind_enum(&routes)?;
    let payload_enum = generate_route_payload_enum(&routes)?;
    let payload_helpers = generate_payload_helpers(&routes)?;
    // pass routes list to generation functions that may need metadata
    let navigation_module = generate_navigation_module(&routes)?;
    let init_module = generate_oxide_init_module()?;
    let navigation_bridge_module = generate_navigation_bridge_module()?;

    if item_mod.content.is_none() {
        return Ok(quote! {
            #item_mod
            #kind_enum
            #payload_enum
            #payload_helpers
            #navigation_module
            #init_module
            #navigation_bridge_module
        });
    }

    let mut out_mod = item_mod;
    let Some((_brace, mod_items)) = &mut out_mod.content else {
        unreachable!("checked content is_some above")
    };

    mod_items.push(Item::Verbatim(kind_enum));
    mod_items.push(Item::Verbatim(payload_enum));
    mod_items.push(Item::Verbatim(payload_helpers));
    mod_items.push(Item::Verbatim(navigation_bridge_module));

    Ok(quote! {
        #out_mod
        #navigation_module
        #init_module
    })
}

fn generate_navigation_module(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
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
                /// Why: reducers/effects may emit navigation intents, and the Dart runtime
                /// must be able to subscribe to those commands.
                ///
                /// How: this only ensures the global navigation runtime exists.
                pub(crate) fn init() -> oxide_core::CoreResult<()> {
                    oxide_core::init_navigation()?;
                    Ok(())
                }

                /// Starts navigation bootstrap exactly once.
                ///
                /// Why: initial-route emission must be explicit and idempotent so route
                /// synchronization from Dart does not re-trigger startup pushes.
                ///
                /// How: guards the generated initial push behind a process-local `OnceLock`.
                pub(crate) fn start() -> oxide_core::CoreResult<()> {
                    init()?;
                    #start_body
                }
            }
        }
    })
}

fn generate_oxide_init_module() -> syn::Result<TokenStream2> {
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

fn generate_navigation_bridge_module() -> syn::Result<TokenStream2> {
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
                    let _ = sink.add(out);
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
        }
    })
}

fn discover_rs_files(dir: &Path) -> syn::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| syn::Error::new(dir.span(), e.to_string()))? {
        let entry = entry.map_err(|e| syn::Error::new(dir.span(), e.to_string()))?;
        let path = entry.path();
        if path.file_name().and_then(|n| n.to_str()) == Some("mod.rs") {
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(files)
}

fn parse_items_from_file(path: &Path) -> syn::Result<Vec<Item>> {
    let src = fs::read_to_string(path).map_err(|e| syn::Error::new(path.span(), e.to_string()))?;
    let file = syn::parse_file(&src)?;
    Ok(file.items)
}

fn collect_routes(items: &[Item]) -> syn::Result<Vec<RouteMeta>> {
    let mut impls_by_type: BTreeMap<String, (&ItemImpl, String, String, Option<String>)> =
        BTreeMap::new();
    let mut annotated: BTreeMap<String, (OxideRouteArgs, Vec<RouteFieldMeta>)> = BTreeMap::new();

    for item in items {
        match item {
            Item::Struct(item_struct) => {
                if let Some(attr) = find_oxide_route_attr(&item_struct.attrs) {
                    let args: OxideRouteArgs = attr.parse_args()?;
                    let ident = item_struct.ident.to_string();
                    let fields = extract_struct_fields(item_struct);
                    annotated.insert(ident, (args, fields));
                }
            }
            Item::Impl(item_impl) => {
                let Some((_, trait_path, _)) = &item_impl.trait_ else {
                    continue;
                };
                let trait_ident = trait_path.segments.last().map(|s| s.ident.to_string());
                if trait_ident.as_deref() != Some("Route") {
                    continue;
                }

                let Some(type_ident) = impl_self_ident(&item_impl.self_ty) else {
                    continue;
                };

                let (return_type, extra_type) = extract_associated_types(item_impl);
                let path = extract_path(item_impl);

                impls_by_type.insert(
                    type_ident.clone(),
                    (item_impl, return_type, extra_type, path),
                );
            }
            _ => {}
        }
    }

    let mut struct_fields: BTreeMap<String, Vec<RouteFieldMeta>> = BTreeMap::new();
    for item in items {
        let Item::Struct(item_struct) = item else {
            continue;
        };
        let ident = item_struct.ident.to_string();
        let fields = extract_struct_fields(item_struct);
        struct_fields.insert(ident, fields);
    }

    let mut routes = Vec::new();
    for (type_name, (args, fields)) in annotated {
        let kind = type_to_kind(&type_name);
        let return_type = args
            .return_type
            .map(|t| t.to_token_stream().to_string())
            .unwrap_or_else(|| "oxide_core::navigation::NoReturn".to_string());
        let extra_type = args
            .extra_type
            .map(|t| t.to_token_stream().to_string())
            .unwrap_or_else(|| "oxide_core::navigation::NoExtra".to_string());
        let path = args.path.as_ref().map(|s| s.value());
        routes.push(RouteMeta {
            kind,
            rust_type: type_name.clone(),
            path,
            return_type,
            extra_type,
            fields,
        });
        struct_fields.remove(&type_name);
    }

    for (type_name, (_impl_item, return_type, extra_type, path)) in impls_by_type {
        if routes.iter().any(|r| r.rust_type == type_name) {
            continue;
        }
        let kind = type_to_kind(&type_name);
        let fields = struct_fields.remove(&type_name).unwrap_or_default();
        routes.push(RouteMeta {
            kind,
            rust_type: type_name.clone(),
            path,
            return_type,
            extra_type,
            fields,
        });
    }

    Ok(routes)
}

fn find_oxide_route_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|a| {
        a.path().segments.last().map(|s| s.ident.to_string()) == Some("oxide_route".to_string())
    })
}

fn impl_self_ident(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    }
}

fn extract_associated_types(item_impl: &ItemImpl) -> (String, String) {
    let mut return_type = "oxide_core::navigation::NoReturn".to_string();
    let mut extra_type = "oxide_core::navigation::NoExtra".to_string();

    for it in &item_impl.items {
        let syn::ImplItem::Type(ty) = it else {
            continue;
        };
        let name = ty.ident.to_string();
        if name == "Return" {
            return_type = ty.ty.to_token_stream().to_string();
        } else if name == "Extra" {
            extra_type = ty.ty.to_token_stream().to_string();
        }
    }

    (return_type, extra_type)
}

fn extract_path(item_impl: &ItemImpl) -> Option<String> {
    for it in &item_impl.items {
        let syn::ImplItem::Fn(f) = it else { continue };
        if f.sig.ident != "path" {
            continue;
        }
        let block = &f.block;
        if block.stmts.len() != 1 {
            continue;
        }
        let syn::Stmt::Expr(expr, _) = &block.stmts[0] else {
            continue;
        };

        match expr {
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(p) = &*call.func {
                    if p.path.segments.last().map(|s| s.ident.to_string()) != Some("Some".into()) {
                        continue;
                    }
                    if call.args.len() != 1 {
                        continue;
                    }
                    if let Some(syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    })) = call.args.first()
                    {
                        return Some(s.value());
                    }
                }
            }
            syn::Expr::Path(p) => {
                if p.path.segments.last().map(|s| s.ident.to_string()) == Some("None".into()) {
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

fn extract_struct_fields(item_struct: &ItemStruct) -> Vec<RouteFieldMeta> {
    match &item_struct.fields {
        syn::Fields::Named(named) => named
            .named
            .iter()
            .filter_map(|f| {
                let name = f.ident.as_ref()?.to_string();
                let ty = f.ty.to_token_stream().to_string();
                Some(RouteFieldMeta { name, ty })
            })
            .collect(),
        syn::Fields::Unnamed(_) | syn::Fields::Unit => Vec::new(),
    }
}

fn type_to_kind(type_name: &str) -> String {
    type_name
        .strip_suffix("Route")
        .filter(|s| !s.is_empty())
        .unwrap_or(type_name)
        .to_string()
}

fn generate_route_kind_enum(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
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

fn generate_route_payload_enum(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
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

fn generate_payload_helpers(routes: &[RouteMeta]) -> syn::Result<TokenStream2> {
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

fn emit_metadata_json(
    crate_name: &str,
    routes: &[RouteMeta],
    manifest_dir: &Path,
) -> syn::Result<()> {
    let target_dir = manifest_dir.join("target").join("oxide_routes");
    fs::create_dir_all(&target_dir)
        .map_err(|e| syn::Error::new(manifest_dir.span(), e.to_string()))?;
    let file_path = target_dir.join(format!("{crate_name}.json"));
    let json = serde_json::to_string_pretty(&RouteMetadataFile {
        crate_name: crate_name.to_string(),
        routes: routes.to_vec(),
    })
    .map_err(|e| syn::Error::new(manifest_dir.span(), e.to_string()))?;
    fs::write(&file_path, json).map_err(|e| syn::Error::new(file_path.span(), e.to_string()))?;
    Ok(())
}

trait SpanExt {
    fn span(&self) -> Span;
}

impl SpanExt for PathBuf {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl SpanExt for &Path {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_ENV_LOCK;
    use std::fs;
    use std::path::PathBuf;

    fn temp_dir(prefix: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let stamp = format!(
            "{}_{}_{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        dir.push(stamp);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn restore_env(key: &str, previous: Option<String>) {
        match previous {
            Some(value) => unsafe { std::env::set_var(key, value) },
            None => unsafe { std::env::remove_var(key) },
        }
    }

    #[test]
    fn type_to_kind_strips_suffix() {
        assert_eq!(type_to_kind("HomeRoute"), "Home");
        assert_eq!(type_to_kind("Route"), "Route");
        assert_eq!(type_to_kind("Splash"), "Splash");
    }

    #[test]
    fn extract_path_reads_some_and_none() {
        let item: ItemImpl = syn::parse_str(
            "impl Route for HomeRoute { fn path() -> Option<&'static str> { Some(\"/home\") } type Return = oxide_core::navigation::NoReturn; type Extra = oxide_core::navigation::NoExtra; }",
        )
        .unwrap();
        assert_eq!(extract_path(&item), Some("/home".to_string()));

        let item_none: ItemImpl = syn::parse_str(
            "impl Route for HomeRoute { fn path() -> Option<&'static str> { None } type Return = oxide_core::navigation::NoReturn; type Extra = oxide_core::navigation::NoExtra; }",
        )
        .unwrap();
        assert_eq!(extract_path(&item_none), None);
    }

    #[test]
    fn collect_routes_reads_fields() {
        let src = "use oxide_core::navigation::{NoExtra, NoReturn, Route}; #[derive(Clone, serde::Serialize, serde::Deserialize)] pub struct HomeRoute { pub id: String, pub count: i32 } impl Route for HomeRoute { type Return = NoReturn; type Extra = NoExtra; fn path() -> Option<&'static str> { Some(\"/home/:id\") } }";
        let file = syn::parse_file(src).unwrap();
        let routes = collect_routes(&file.items).unwrap();
        assert_eq!(routes.len(), 1);
        let route = &routes[0];
        assert_eq!(route.kind, "Home");
        assert_eq!(route.rust_type, "HomeRoute");
        assert_eq!(route.path.as_deref(), Some("/home/:id"));
        assert!(route.return_type.contains("NoReturn"));
        assert!(route.extra_type.contains("NoExtra"));
        assert_eq!(route.fields.len(), 2);
        assert_eq!(route.fields[0].name, "id");
        assert_eq!(route.fields[1].name, "count");
    }

    #[test]
    fn generated_tokens_include_variants() {
        let routes = vec![
            RouteMeta {
                kind: "Home".to_string(),
                rust_type: "HomeRoute".to_string(),
                path: Some("/home".to_string()),
                return_type: "oxide_core::navigation::NoReturn".to_string(),
                extra_type: "oxide_core::navigation::NoExtra".to_string(),
                fields: vec![],
            },
            RouteMeta {
                kind: "Charts".to_string(),
                rust_type: "ChartsRoute".to_string(),
                path: None,
                return_type: "oxide_core::navigation::NoReturn".to_string(),
                extra_type: "oxide_core::navigation::NoExtra".to_string(),
                fields: vec![RouteFieldMeta {
                    name: "id".to_string(),
                    ty: "u64".to_string(),
                }],
            },
        ];

        let kind = generate_route_kind_enum(&routes).unwrap().to_string();
        let payload = generate_route_payload_enum(&routes).unwrap().to_string();
        let helpers = generate_payload_helpers(&routes).unwrap().to_string();

        assert!(kind.contains("RouteKind"));
        assert!(kind.contains("Home"));
        assert!(kind.contains("Charts"));
        assert!(payload.contains("RoutePayload"));
        assert!(payload.contains("Home"));
        assert!(payload.contains("Charts"));
        assert!(helpers.contains("RoutePayload"));
    }

    #[test]
    fn navigation_bridge_module_contains_bindings() {
        let tokens = generate_navigation_bridge_module().unwrap().to_string();
        assert!(tokens.contains("oxide_nav_commands_stream"));
        assert!(tokens.contains("oxide_nav_emit_result"));
        assert!(tokens.contains("oxide_nav_set_current_route"));
    }

    #[test]
    fn expand_routes_module_writes_metadata_file() {
        let _guard = TEST_ENV_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap();
        let dir = temp_dir("oxide_routes_expand");
        let prev_manifest = std::env::var("CARGO_MANIFEST_DIR").ok();
        let prev_pkg = std::env::var("CARGO_PKG_NAME").ok();
        unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };
        unsafe { std::env::set_var("CARGO_PKG_NAME", "oxide_routes_test") };

        let src_dir = dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();

        let item_mod: ItemMod = syn::parse_str(
            "pub mod routes { use oxide_core::navigation::{NoExtra, NoReturn, Route}; use serde::{Deserialize, Serialize}; #[derive(Clone, Serialize, Deserialize)] pub struct HomeRoute { pub id: u64 } impl Route for HomeRoute { type Return = NoReturn; type Extra = NoExtra; fn path() -> Option<&'static str> { Some(\"/home\") } } }",
        )
        .unwrap();

        let _ = expand_routes_module(item_mod).unwrap();

        let metadata_path = dir
            .join("target")
            .join("oxide_routes")
            .join("oxide_routes_test.json");
        let json = fs::read_to_string(&metadata_path).unwrap();
        assert!(json.contains("\"Home\""));
        assert!(json.contains("\"HomeRoute\""));

        restore_env("CARGO_MANIFEST_DIR", prev_manifest);
        restore_env("CARGO_PKG_NAME", prev_pkg);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn discover_and_parse_routes_files() {
        let dir = temp_dir("oxide_routes_scan");
        let routes_dir = dir.join("routes");
        fs::create_dir_all(&routes_dir).unwrap();
        let file_path = routes_dir.join("home.rs");
        fs::write(
            &file_path,
            "use oxide_core::navigation::{NoExtra, NoReturn, Route}; #[derive(Clone, serde::Serialize, serde::Deserialize)] pub struct HomeRoute; impl Route for HomeRoute { type Return = NoReturn; type Extra = NoExtra; }",
        )
        .unwrap();

        let files = discover_rs_files(&routes_dir).unwrap();
        assert_eq!(files.len(), 1);
        let items = parse_items_from_file(&files[0]).unwrap();
        assert!(!items.is_empty());

        fs::remove_dir_all(&dir).unwrap();
    }
}
