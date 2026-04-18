use crate::TEST_ENV_LOCK;
use std::fs;
use std::path::PathBuf;
use syn::{Attribute, ItemImpl, ItemMod, ItemStruct, Token};

use super::codegen::{
    generate_navigation_bridge_module, generate_payload_helpers, generate_route_kind_enum,
    generate_route_payload_enum,
};
use super::discovery::{
    collect_routes, discover_rs_files, extract_path, find_oxide_route_attr, parse_items_from_file,
    parse_oxide_route_args, type_to_kind,
};
use super::expand_routes_module;
use super::model::{RouteFieldMeta, RouteMeta};
use super::route_struct::ensure_route_struct_derives;

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
fn parse_oxide_route_args_accepts_no_parentheses() {
    let item_struct: ItemStruct =
        syn::parse_str("#[oxide_route] pub struct SplashRoute {}").unwrap();
    let attr = find_oxide_route_attr(&item_struct.attrs).expect("oxide_route attr");
    let args = parse_oxide_route_args(attr).unwrap();
    assert!(args.path.is_none());
    assert!(args.return_type.is_none());
    assert!(args.extra_type.is_none());
}

fn derive_occurrences(attrs: &[Attribute], needle: &str) -> usize {
    let mut count = 0usize;
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        let Ok(list) = attr
            .parse_args_with(syn::punctuated::Punctuated::<syn::Path, Token![,]>::parse_terminated)
        else {
            continue;
        };
        for p in list {
            if p.segments.last().map(|s| s.ident.to_string()) == Some(needle.to_string()) {
                count += 1;
            }
        }
    }
    count
}

#[test]
fn ensure_route_struct_derives_adds_missing_and_keeps_existing_once() {
    let mut item_struct: ItemStruct =
        syn::parse_str("#[derive(Clone, serde::Serialize)] pub struct HomeRoute {}").unwrap();
    ensure_route_struct_derives(&mut item_struct);

    assert_eq!(derive_occurrences(&item_struct.attrs, "Clone"), 1);
    assert_eq!(derive_occurrences(&item_struct.attrs, "Debug"), 1);
    assert_eq!(derive_occurrences(&item_struct.attrs, "Serialize"), 1);
    assert_eq!(derive_occurrences(&item_struct.attrs, "Deserialize"), 1);
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
    assert!(tokens.contains("__oxide_nav_require_fresh_frb_bindings"));
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
fn expand_routes_module_supports_empty_inline_module_without_include() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();
    let dir = temp_dir("oxide_routes_inline_empty");
    let prev_manifest = std::env::var("CARGO_MANIFEST_DIR").ok();
    let prev_pkg = std::env::var("CARGO_PKG_NAME").ok();
    unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };
    unsafe { std::env::set_var("CARGO_PKG_NAME", "oxide_routes_inline_empty_test") };

    let src_dir = dir.join("src");
    let routes_dir = src_dir.join("routes");
    fs::create_dir_all(&routes_dir).unwrap();
    fs::write(
        routes_dir.join("mod.rs"),
        "pub mod splash_route; pub use splash_route::SplashRoute;",
    )
    .unwrap();
    fs::write(
        routes_dir.join("splash_route.rs"),
        "use oxide_generator_rs::oxide_route; #[oxide_route] pub struct SplashRoute {}",
    )
    .unwrap();

    let item_mod: ItemMod = syn::parse_str("pub mod routes {}").unwrap();
    let out = expand_routes_module(item_mod).unwrap().to_string();
    assert!(out.contains("pub mod routes"));
    assert!(out.contains("pub mod splash_route"));
    assert!(out.contains("RouteKind"));
    assert!(out.contains("RoutePayload"));

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
