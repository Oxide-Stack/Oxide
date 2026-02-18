use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use serde::Serialize;
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
    let navigation_module = generate_navigation_module()?;

    if item_mod.content.is_none() {
        return Ok(quote! {
            #item_mod
            #kind_enum
            #payload_enum
            #payload_helpers
            #navigation_module
        });
    }

    let mut out_mod = item_mod;
    let Some((_brace, mod_items)) = &mut out_mod.content else {
        unreachable!("checked content is_some above")
    };

    mod_items.push(Item::Verbatim(kind_enum));
    mod_items.push(Item::Verbatim(payload_enum));
    mod_items.push(Item::Verbatim(payload_helpers));

    Ok(quote! {
        #out_mod
        #navigation_module
    })
}

fn generate_navigation_module() -> syn::Result<TokenStream2> {
    Ok(quote! {
        pub mod navigation {
            pub mod runtime {
                /// Initializes the Oxide navigation runtime.
                ///
                /// Why: reducers/effects may emit navigation intents, and the Dart runtime
                /// must be able to subscribe to those commands.
                ///
                /// How: this sets up the global navigation runtime singleton used by Oxide.
                pub fn init() -> ::oxide_core::CoreResult<()> {
                    ::oxide_core::init_navigation()?;
                    Ok(())
                }
            }

            pub mod frb {
                #[flutter_rust_bridge::frb]
                pub async fn init_navigation() -> ::oxide_core::CoreResult<()> {
                    super::runtime::init()
                }

                /// Stream of navigation commands emitted by Rust reducers/effects.
                #[flutter_rust_bridge::frb]
                pub async fn oxide_nav_commands_stream(
                    sink: crate::frb_generated::StreamSink<String>,
                ) {
                    let _ = super::runtime::init();
                    let runtime =
                        ::oxide_core::navigation_runtime().expect("navigation initialized");
                    let mut rx = runtime.subscribe_commands();

                    loop {
                        match rx.recv().await {
                            Ok(cmd) => {
                                if let Ok(json) = ::serde_json::to_string(&cmd) {
                                    let _ = sink.add(json);
                                }
                            }
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Lagged(
                                _,
                            )) => continue,
                            Err(::oxide_core::tokio::sync::broadcast::error::RecvError::Closed) => {
                                break
                            }
                        }
                    }
                }

                /// Emits a result payload for a previously-issued ticket.
                #[flutter_rust_bridge::frb]
                pub async fn oxide_nav_emit_result(
                    ticket: String,
                    result_json: String,
                ) -> ::oxide_core::CoreResult<()> {
                    super::runtime::init()?;
                    let runtime = ::oxide_core::navigation_runtime()?;
                    let value: ::serde_json::Value =
                        ::serde_json::from_str(&result_json).unwrap_or(::serde_json::Value::Null);
                    let _ = runtime.emit_result(&ticket, value).await;
                    Ok(())
                }

                /// Updates the current route snapshot reported by Dart.
                #[flutter_rust_bridge::frb]
                pub fn oxide_nav_set_current_route(
                    kind: String,
                    payload_json: String,
                ) -> ::oxide_core::CoreResult<()> {
                    super::runtime::init()?;
                    let runtime = ::oxide_core::navigation_runtime()?;
                    let payload: ::serde_json::Value =
                        ::serde_json::from_str(&payload_json).unwrap_or(::serde_json::Value::Null);
                    runtime.set_current_route(Some(::oxide_core::navigation::NavRoute {
                        kind,
                        payload,
                        extras: None,
                    }));
                    Ok(())
                }
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

    for item in items {
        let Item::Impl(item_impl) = item else { continue };
        let Some((_, trait_path, _)) = &item_impl.trait_ else { continue };
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

    let mut struct_fields: BTreeMap<String, Vec<RouteFieldMeta>> = BTreeMap::new();
    for item in items {
        let Item::Struct(item_struct) = item else { continue };
        let ident = item_struct.ident.to_string();
        let fields = extract_struct_fields(item_struct);
        struct_fields.insert(ident, fields);
    }

    let mut routes = Vec::new();
    for (type_name, (_impl_item, return_type, extra_type, path)) in impls_by_type {
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
        let syn::ImplItem::Type(ty) = it else { continue };
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
        let syn::Stmt::Expr(expr, _) = &block.stmts[0] else { continue };

        match expr {
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(p) = &*call.func {
                    if p.path.segments.last().map(|s| s.ident.to_string()) != Some("Some".into()) {
                        continue;
                    }
                    if call.args.len() != 1 {
                        continue;
                    }
                    if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. })) =
                        call.args.first()
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
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        #[serde(tag = "kind", content = "payload")]
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

    Ok(quote! {
        impl RoutePayload {
            pub fn kind(&self) -> RouteKind {
                match self {
                    #( Self::#variants(_) => RouteKind::#variants, )*
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

fn emit_metadata_json(crate_name: &str, routes: &[RouteMeta], manifest_dir: &Path) -> syn::Result<()> {
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
    fn navigation_module_contains_bindings() {
        let tokens = generate_navigation_module().unwrap().to_string();
        assert!(tokens.contains("oxide_nav_commands_stream"));
        assert!(tokens.contains("oxide_nav_emit_result"));
        assert!(tokens.contains("oxide_nav_set_current_route"));
    }

    #[test]
    fn expand_routes_module_writes_metadata_file() {
        let _guard = TEST_ENV_LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap();
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
