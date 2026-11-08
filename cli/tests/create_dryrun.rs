//! Golden tests for `oxide create --dry-run` file list, template rendering,
//! demo-signature detection, backend parsing, and oxide-source resolution.

use oxide_cli::commands::create::{
    Backend, OxideSource, TemplateVars, dep_snippets, is_frb_demo_dart_main,
    is_frb_demo_integration_test, is_frb_demo_rust_api, parse_backend, planned_files, render_all,
    resolve_oxide_source,
};
use oxide_cli::common::gitignore::merge as merge_gitignore;
use std::path::PathBuf;

fn test_vars(backend: Backend) -> TemplateVars {
    TemplateVars {
        app_name: "demo_app".to_string(),
        rust_crate: "rust_lib_demo_app".to_string(),
        backend,
        page_class: "CounterPage".to_string(),
        deps: dep_snippets(&OxideSource::Pub, &PathBuf::from("/tmp/demo_app")),
    }
}

#[test]
fn dry_run_file_list_golden() {
    assert_eq!(
        planned_files(),
        vec![
            ".gitignore (merge additions)".to_string(),
            "README.md".to_string(),
            "analysis_options.yaml".to_string(),
            "flutter_rust_bridge.yaml".to_string(),
            "integration_test/oxide_test.dart".to_string(),
            "lib/main.dart".to_string(),
            "lib/src/app.dart".to_string(),
            "lib/src/counter_page.dart".to_string(),
            "lib/src/navigation/pages.dart".to_string(),
            "lib/src/oxide.dart".to_string(),
            "pubspec.yaml".to_string(),
            "rust/Cargo.toml".to_string(),
            "rust/src/api/bridge.rs".to_string(),
            "rust/src/api/mod.rs".to_string(),
            "rust/src/lib.rs".to_string(),
            "rust/src/routes/home_route.rs".to_string(),
            "rust/src/routes/mod.rs".to_string(),
            "rust/src/state/app_action.rs".to_string(),
            "rust/src/state/app_state.rs".to_string(),
            "rust/src/state/mod.rs".to_string(),
            "test/widget_test.dart".to_string(),
        ]
    );
}

#[test]
fn rendered_templates_have_no_placeholders() {
    for backend in [
        Backend::Inherited,
        Backend::InheritedHooks,
        Backend::Riverpod,
        Backend::Bloc,
    ] {
        let vars = test_vars(backend);
        for (rel, content) in render_all(&vars) {
            assert!(!content.contains("{{"), "unrendered var in {rel}");
            assert!(!content.contains("}}"), "unrendered var in {rel}");
            assert!(!content.is_empty(), "empty render for {rel}");
        }
    }
}

#[test]
fn rendered_pubspec_and_manifests_use_vars() {
    let vars = test_vars(Backend::Riverpod);
    let files = render_all(&vars);
    let get = |name: &str| files.iter().find(|(r, _)| r == name).unwrap().1.clone();
    let pubspec = get("pubspec.yaml");
    assert!(pubspec.contains("name: demo_app"));
    assert!(pubspec.contains("rust_lib_demo_app:\n    path: rust_builder"));
    assert!(pubspec.contains("flutter_riverpod: ^3.2.0"));
    assert!(pubspec.contains("oxide_runtime: ^0.5.0"));
    assert!(pubspec.contains("oxide_generator: ^0.5.0"));
    let cargo = get("rust/Cargo.toml");
    assert!(cargo.contains("name = \"rust_lib_demo_app\""));
    assert!(cargo.contains("oxide_core = { version = \"0.5.0\" }"));
    let oxide = get("lib/src/oxide.dart");
    assert!(oxide.contains("backend: OxideBackend.riverpod"));
    assert!(oxide.contains("class AppStore {}"));
    let page = get("lib/src/counter_page.dart");
    assert!(page.contains("class CounterPage"));
    assert!(page.contains("state.counter.toInt()"));
    assert!(page.contains("ref.watch(appStoreProvider)"));
    let readme = get("README.md");
    assert!(readme.contains("demo_app"));
}

#[test]
fn rendered_inherited_page_uses_scope_and_animated_builder() {
    let vars = test_vars(Backend::Inherited);
    let files = render_all(&vars);
    let page = files
        .iter()
        .find(|(r, _)| r == "lib/src/counter_page.dart")
        .unwrap()
        .1
        .clone();
    assert!(page.contains("AppStoreScope.controllerOf(context)"));
    assert!(page.contains("AnimatedBuilder"));
    assert!(page.contains("view.actions.increment()"));
}

#[test]
fn demo_signature_keep_vs_delete() {
    // Unmodified FRB demo files match.
    assert!(is_frb_demo_rust_api(
        "simple.rs",
        "use flutter_rust_bridge::frb;\n#[frb(sync)]\npub fn greet(name: String) -> String { name }"
    ));
    assert!(is_frb_demo_dart_main(
        "import 'src/rust/api/simple.dart';\nString x = greet(name: 'a');"
    ));
    assert!(is_frb_demo_integration_test(
        "simple_test.dart",
        "test('greet', () async { expect(greet(name: 'x'), 'x'); });"
    ));
    // Modified files (signature gone) are kept.
    assert!(!is_frb_demo_rust_api(
        "simple.rs",
        "pub fn custom_api() -> u32 { 42 }"
    ));
    assert!(!is_frb_demo_rust_api("other.rs", "pub fn greet() {}"));
    assert!(!is_frb_demo_dart_main(
        "void main() { runApp(const MyApp()); }"
    ));
    assert!(!is_frb_demo_integration_test(
        "simple_test.dart",
        "test('counter', () { expect(1, 1); });"
    ));
    assert!(!is_frb_demo_integration_test(
        "oxide_test.dart",
        "greet(name: 'x')"
    ));
}

#[test]
fn backend_parsing() {
    assert_eq!(parse_backend("inherited").unwrap(), Backend::Inherited);
    assert_eq!(parse_backend("hooks").unwrap(), Backend::InheritedHooks);
    assert_eq!(parse_backend("riverpod").unwrap(), Backend::Riverpod);
    assert_eq!(parse_backend("bloc").unwrap(), Backend::Bloc);
    assert!(parse_backend("redux").is_err());
}

#[test]
fn oxide_source_auto_outside_checkout_is_pub() {
    let dir = tempfile::tempdir().unwrap();
    let source = resolve_oxide_source("auto", dir.path()).unwrap();
    assert_eq!(source, OxideSource::Pub);
}

#[test]
fn oxide_source_auto_inside_checkout_is_path() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("rust/oxide_core")).unwrap();
    std::fs::create_dir_all(dir.path().join("flutter/oxide_runtime")).unwrap();
    std::fs::write(dir.path().join("rust/oxide_core/Cargo.toml"), "[package]").unwrap();
    std::fs::write(
        dir.path().join("flutter/oxide_runtime/pubspec.yaml"),
        "name: x",
    )
    .unwrap();
    // Nested project dir inside the fake checkout.
    let proj = dir.path().join("my_app");
    std::fs::create_dir_all(&proj).unwrap();
    let source = resolve_oxide_source("auto", &proj).unwrap();
    match source {
        OxideSource::Path { root } => {
            assert_eq!(root, dir.path().canonicalize().unwrap());
        }
        other => panic!("expected Path, got {other:?}"),
    }
}

#[test]
fn path_source_snippets_are_relative() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("checkout");
    std::fs::create_dir_all(root.join("rust/oxide_core")).unwrap();
    std::fs::create_dir_all(root.join("flutter/oxide_runtime")).unwrap();
    let app_dir = root.join("apps").join("demo_app");
    let snippets = dep_snippets(&OxideSource::Path { root: root.clone() }, &app_dir);
    assert!(
        snippets
            .oxide_runtime_dep
            .contains("../../flutter/oxide_runtime")
    );
    assert!(snippets.oxide_core_dep.contains("../../../rust/oxide_core"));
    assert!(
        snippets
            .oxide_generator_rs_dep
            .contains("../../../rust/oxide_generator_rs")
    );
}

#[test]
fn gitignore_merge_is_additive() {
    let merged = merge_gitignore(Some("name: x\n*.log\n"), "*.log\n*.g.dart\n");
    assert!(merged.contains("*.log"));
    assert!(merged.contains("*.g.dart"));
    assert_eq!(merged.matches("*.log").count(), 1);
}
