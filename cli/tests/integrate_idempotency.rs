//! Idempotency tests for `oxide integrate`: running twice must be a no-op the
//! second time, and marker patches must never duplicate.

use oxide_cli::commands::integrate::{
    IntegrateArgs, MainPatch, ProjectShape, detect_shape, has_marker, marker_block, patch_api_mod,
    patch_frb_rust_input, patch_lib_rs, patch_main_dart,
};
use std::path::PathBuf;

fn default_args(path: PathBuf) -> IntegrateArgs {
    IntegrateArgs {
        path,
        backend: "inherited".to_string(),
        oxide_source: "pub".to_string(),
        example: "counter".to_string(),
        keep_frb_example: false,
        no_init_main: false,
        no_codegen: true,
        dry_run: false,
        yes: true,
        skip_frb: false,
        oxide_path: None,
    }
}

/// Minimal FRB app fixture (no Oxide yet).
fn write_frb_fixture(root: &std::path::Path) {
    std::fs::create_dir_all(root.join("lib")).unwrap();
    std::fs::create_dir_all(root.join("rust/src/api")).unwrap();
    std::fs::write(
        root.join("pubspec.yaml"),
        "name: demo_app\nenvironment:\n  sdk: ^3.10.4\ndependencies:\n  flutter:\n    sdk: flutter\ndev_dependencies:\n  flutter_test:\n    sdk: flutter\n",
    )
    .unwrap();
    std::fs::write(
        root.join("flutter_rust_bridge.yaml"),
        "rust_input: crate\nrust_root: rust/\ndart_output: lib/src/rust\n",
    )
    .unwrap();
    std::fs::write(
        root.join("lib/main.dart"),
        "import 'package:flutter/material.dart';\n\nvoid main() {\n  runApp(const MyApp());\n}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("rust/Cargo.toml"),
        "[package]\nname = \"rust_lib_demo_app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"cdylib\", \"staticlib\"]\n\n[dependencies]\nflutter_rust_bridge = \"=2.12.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("rust/src/lib.rs"), "pub mod api;\n").unwrap();
    std::fs::write(root.join("rust/src/api/mod.rs"), "// api\n").unwrap();
}

#[test]
fn integrate_twice_second_is_noop() {
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_frb_fixture(dir.path());
    assert_eq!(detect_shape(dir.path()), ProjectShape::FrbApp);

    let first =
        oxide_cli::commands::integrate::run(default_args(dir.path().to_path_buf())).unwrap();
    assert_eq!(first.shape, "FrbApp");
    assert!(!first.noop);
    assert!(first.changed.contains(&"lib/src/oxide.dart".to_string()));
    assert!(
        first
            .changed
            .contains(&"rust/src/api/oxide_bridge.rs".to_string())
    );
    assert!(first.changed.contains(&"lib/main.dart".to_string()));
    assert!(first.changed.contains(&"pubspec.yaml".to_string()));
    assert!(first.changed.contains(&"rust/Cargo.toml".to_string()));
    // Backup manifest written for patched pre-existing files.
    assert!(dir.path().join(".oxide/backups").is_dir());

    let lib_rs = std::fs::read_to_string(dir.path().join("rust/src/lib.rs")).unwrap();
    assert!(has_marker(&lib_rs, "oxide-lib"));
    let main_dart = std::fs::read_to_string(dir.path().join("lib/main.dart")).unwrap();
    assert!(main_dart.contains("runOxideApp"));

    let second =
        oxide_cli::commands::integrate::run(default_args(dir.path().to_path_buf())).unwrap();
    assert_eq!(second.shape, "OxideApp");
    assert!(second.noop, "second run changed: {:?}", second.changed);
    assert!(second.changed.is_empty());

    // Markers appear exactly once.
    let lib_rs2 = std::fs::read_to_string(dir.path().join("rust/src/lib.rs")).unwrap();
    assert_eq!(lib_rs2.matches("// oxide:begin oxide-lib").count(), 1);
    assert_eq!(lib_rs, lib_rs2);
}

#[test]
fn marker_patches_are_idempotent() {
    let once = patch_lib_rs("pub mod api;\n").unwrap();
    assert!(has_marker(&once, "oxide-lib"));
    assert!(once.contains("mod state;"));
    assert!(patch_lib_rs(&once).is_none());

    let api_once = patch_api_mod("// api\n", "oxide_bridge").unwrap();
    assert!(has_marker(&api_once, "oxide-api"));
    assert!(patch_api_mod(&api_once, "oxide_bridge").is_none());
    // Existing entry without marker is also a no-op.
    assert!(patch_api_mod("pub mod oxide_bridge;\n", "oxide_bridge").is_none());

    match patch_main_dart("void main() {\n  runApp(const MyApp());\n}\n") {
        MainPatch::Patched(new) => {
            assert!(new.contains("await runOxideApp("));
            assert!(new.contains("Future<void> main() async"));
            assert!(has_marker(&new, "oxide-main-import"));
            assert_eq!(patch_main_dart(&new), MainPatch::AlreadyPatched);
        }
        other => panic!("expected Patched, got {other:?}"),
    }
    assert_eq!(
        patch_main_dart("await runOxideApp(const MyApp());"),
        MainPatch::AlreadyPatched
    );
    assert_eq!(
        patch_main_dart("void custom() {}\n"),
        MainPatch::PatternMiss
    );
}

/// Regression: `await` was inserted unconditionally while `main` was only made
/// async when the literal `"void main() {"` appeared. Every other legal spelling
/// produced `await` inside a non-async `main`, which is an analyzer error
/// reported as success.
#[test]
fn main_dart_makes_main_async_for_every_spelling() {
    let spellings = [
        "void main() {\n  runApp(const MyApp());\n}\n",
        "main() {\n  runApp(const MyApp());\n}\n",
        "void  main() {\n  runApp(const MyApp());\n}\n",
        "void main()\n{\n  runApp(const MyApp());\n}\n",
        "  void main() {\n  runApp(const MyApp());\n}\n",
        "Future<void> main() {\n  runApp(const MyApp());\n}\n",
    ];
    for src in spellings {
        let MainPatch::Patched(new) = patch_main_dart(src) else {
            panic!("expected Patched for {src:?}");
        };
        assert!(
            new.contains("Future<void> main() async"),
            "main was not made async for {src:?}\ngot:\n{new}"
        );
        assert!(
            new.contains("await runOxideApp("),
            "runApp was not wrapped for {src:?}\ngot:\n{new}"
        );
    }
}

/// Regression: `runApp(` was replaced globally, so a second call in another
/// function or a comment picked up an `await` that function cannot contain.
#[test]
fn main_dart_only_rewrites_runapp_inside_main() {
    let src = concat!(
        "void main() {\n",
        "  runApp(const MyApp());\n",
        "}\n",
        "\n",
        "void launchPreview() {\n",
        "  runApp(const Preview());\n",
        "}\n",
    );
    let MainPatch::Patched(new) = patch_main_dart(src) else {
        panic!("expected Patched");
    };
    assert!(
        new.contains("await runOxideApp(const MyApp())"),
        "main's runApp should be wrapped:\n{new}"
    );
    assert!(
        new.contains("  runApp(const Preview());"),
        "runApp outside main must be left alone:\n{new}"
    );
}

#[test]
fn main_dart_does_not_double_async_an_async_main() {
    let src = "Future<void> main() async {\n  runApp(const MyApp());\n}\n";
    let MainPatch::Patched(new) = patch_main_dart(src) else {
        panic!("expected Patched");
    };
    assert_eq!(
        new.matches("main() async").count(),
        1,
        "main should not become doubly async:\n{new}"
    );
}

#[test]
fn main_dart_ignores_runapp_inside_string_literals() {
    let src = "void main() {\n  runApp(const MyApp());\n  print('a { b }');\n}\n";
    let MainPatch::Patched(new) = patch_main_dart(src) else {
        panic!("expected Patched");
    };
    // The brace inside the string must not have terminated the body scan early.
    assert!(new.contains("Future<void> main() async"), "got:\n{new}");
}

#[test]
fn frb_rust_input_widened_to_crate() {
    assert_eq!(
        patch_frb_rust_input("rust_input: crate::api\nrust_root: rust/\n").as_deref(),
        Some("rust_input: crate\nrust_root: rust/\n")
    );
    assert!(patch_frb_rust_input("rust_input: crate\n").is_none());
    assert!(patch_frb_rust_input("rust_root: rust/\n").is_none());
}

#[test]
fn marker_block_format() {
    assert_eq!(
        marker_block("x", "body"),
        "// oxide:begin x\nbody\n// oxide:end x"
    );
}

#[test]
fn starter_routes_and_orphan_cleanup() {
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_frb_fixture(dir.path());
    // Unmodified FRB demo (Rust + its generated Dart orphan).
    std::fs::write(
        dir.path().join("rust/src/api/simple.rs"),
        "pub fn greet(name: String) -> String { name }\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join("lib/src/rust/api")).unwrap();
    std::fs::write(
        dir.path().join("lib/src/rust/api/simple.dart"),
        "// generated\nString greet({required String name}) => throw '';\n",
    )
    .unwrap();

    let first =
        oxide_cli::commands::integrate::run(default_args(dir.path().to_path_buf())).unwrap();
    // Starter route scaffolded for routeless apps.
    assert!(
        first
            .changed
            .contains(&"rust/src/routes/mod.rs".to_string())
    );
    assert!(
        first
            .changed
            .contains(&"rust/src/routes/home_route.rs".to_string())
    );
    let home = std::fs::read_to_string(dir.path().join("rust/src/routes/home_route.rs")).unwrap();
    assert!(home.contains("pub struct HomeRoute"));
    assert!(home.contains("demo_app"));
    // Demo Rust source and its generated Dart orphan are both gone.
    assert!(!dir.path().join("rust/src/api/simple.rs").exists());
    assert!(!dir.path().join("lib/src/rust/api/simple.dart").exists());

    // An app with its own routes keeps them: no starter files, no clobber.
    let dir2 = tempfile::tempdir().unwrap();
    write_frb_fixture(dir2.path());
    std::fs::create_dir_all(dir2.path().join("rust/src/routes")).unwrap();
    std::fs::write(
        dir2.path().join("rust/src/routes/mod.rs"),
        "pub mod detail_route;\n",
    )
    .unwrap();
    std::fs::write(
        dir2.path().join("rust/src/routes/detail_route.rs"),
        "#[oxide_generator_rs::oxide_route()]\npub struct DetailRoute {}\n",
    )
    .unwrap();
    let run = oxide_cli::commands::integrate::run(default_args(dir2.path().to_path_buf())).unwrap();
    assert!(!run.changed.contains(&"rust/src/routes/mod.rs".to_string()));
    assert!(
        !run.changed
            .contains(&"rust/src/routes/home_route.rs".to_string())
    );
    let kept =
        std::fs::read_to_string(dir2.path().join("rust/src/routes/detail_route.rs")).unwrap();
    assert!(kept.contains("DetailRoute"));
}
