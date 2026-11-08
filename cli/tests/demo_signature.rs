//! Demo-signature tests: unmodified FRB demo files are removed, modified ones
//! are kept, and `--keep-frb-example` keeps everything.

use oxide_cli::commands::integrate::IntegrateArgs;
use std::path::PathBuf;

const DEMO_SIMPLE_RS: &str = r#"use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
"#;

const MODIFIED_SIMPLE_RS: &str = r#"use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn my_custom_api(value: u32) -> u32 {
    value * 2
}
"#;

const DEMO_TEST: &str = r#"import 'package:demo_app/src/rust/api/simple.dart';

void main() {
  test('greet', () async {
    expect(greet(name: 'x'), 'Hello, x!');
  });
}
"#;

fn write_demo_fixture(root: &std::path::Path, simple_rs: &str) {
    std::fs::create_dir_all(root.join("lib")).unwrap();
    std::fs::create_dir_all(root.join("rust/src/api")).unwrap();
    std::fs::create_dir_all(root.join("integration_test")).unwrap();
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
    std::fs::write(root.join("rust/src/api/mod.rs"), "pub mod simple;\n").unwrap();
    std::fs::write(root.join("rust/src/api/simple.rs"), simple_rs).unwrap();
    std::fs::write(root.join("integration_test/simple_test.dart"), DEMO_TEST).unwrap();
}

fn args(path: PathBuf, keep: bool) -> IntegrateArgs {
    IntegrateArgs {
        path,
        backend: "inherited".to_string(),
        oxide_source: "pub".to_string(),
        example: "none".to_string(),
        keep_frb_example: keep,
        no_init_main: false,
        no_codegen: true,
        dry_run: false,
        yes: true,
        skip_frb: false,
        oxide_path: None,
    }
}

#[test]
fn unmodified_demo_is_deleted() {
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_demo_fixture(dir.path(), DEMO_SIMPLE_RS);

    let report =
        oxide_cli::commands::integrate::run(args(dir.path().to_path_buf(), false)).unwrap();
    assert!(!dir.path().join("rust/src/api/simple.rs").exists());
    assert!(
        !dir.path()
            .join("integration_test/simple_test.dart")
            .exists()
    );
    assert!(
        report
            .changed
            .contains(&"rust/src/api/simple.rs".to_string())
    );
    let api_mod = std::fs::read_to_string(dir.path().join("rust/src/api/mod.rs")).unwrap();
    assert!(!api_mod.contains("pub mod simple;"));
}

#[test]
fn modified_demo_is_kept() {
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_demo_fixture(dir.path(), MODIFIED_SIMPLE_RS);

    let report =
        oxide_cli::commands::integrate::run(args(dir.path().to_path_buf(), false)).unwrap();
    assert!(dir.path().join("rust/src/api/simple.rs").exists());
    assert!(
        !report
            .changed
            .contains(&"rust/src/api/simple.rs".to_string())
    );
    // The untouched-by-signature integration test is still removed (it matches).
    assert!(
        !dir.path()
            .join("integration_test/simple_test.dart")
            .exists()
    );
}

#[test]
fn extended_demo_is_kept() {
    // greet demo plus a user-added function: real code lives here now.
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_demo_fixture(
        dir.path(),
        "pub fn greet(name: String) -> String { name }\npub fn triple(value: i32) -> i32 { value * 3 }\n",
    );

    let report =
        oxide_cli::commands::integrate::run(args(dir.path().to_path_buf(), false)).unwrap();
    assert!(dir.path().join("rust/src/api/simple.rs").exists());
    assert!(
        !report
            .changed
            .contains(&"rust/src/api/simple.rs".to_string())
    );
}

#[test]
fn keep_flag_preserves_demo() {
    unsafe {
        std::env::set_var("OXIDE_CLI_NO_EXTERNAL", "1");
    }
    let dir = tempfile::tempdir().unwrap();
    write_demo_fixture(dir.path(), DEMO_SIMPLE_RS);

    let report = oxide_cli::commands::integrate::run(args(dir.path().to_path_buf(), true)).unwrap();
    assert!(dir.path().join("rust/src/api/simple.rs").exists());
    assert!(
        dir.path()
            .join("integration_test/simple_test.dart")
            .exists()
    );
    assert!(
        !report
            .changed
            .contains(&"rust/src/api/simple.rs".to_string())
    );
}
