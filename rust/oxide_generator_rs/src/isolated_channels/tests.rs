use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::ItemImpl;

use super::{OxideCallbackArgs, OxideEventChannelArgs, expand_oxide_callback, expand_oxide_event_channel};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn event_channel_generates_variant_helpers() {
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_event");
    write_src_lib(
        &dir,
        r#"
        #[derive(Clone)]
        pub enum AnalyticsEvent {
            Track { name: String },
        }
        "#,
    );

    unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl oxide_core::OxideEventChannel for AnalyticsChannel {
            type Events = AnalyticsEvent;
        }
        "#,
    )
    .unwrap();

    let ts = expand_oxide_event_channel(
        OxideEventChannelArgs { orphaned: false, no_frb: true },
        item_impl,
    )
    .unwrap();

    let out = ts.to_string();
    assert!(out.contains("pub fn track"), "expected track helper, got: {out}");
}

#[test]
fn callbacking_enforces_variant_parity() {
    let _guard = ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_callback");
    write_src_lib(
        &dir,
        r#"
        pub enum DialogRequest {
            Confirm { title: String },
        }

        pub enum DialogResponse {
            SomethingElse(bool),
        }
        "#,
    );

    unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl oxide_core::OxideCallbacking for DialogService {
            type Request = DialogRequest;
            type Response = DialogResponse;
        }
        "#,
    )
    .unwrap();

    let err = expand_oxide_callback(OxideCallbackArgs { no_frb: true }, item_impl).unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("missing Response variant"),
        "expected parity error, got: {message}"
    );
}

fn make_temp_manifest_dir(name: &str) -> String {
    let mut dir = std::env::temp_dir();
    dir.push(format!("{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"tmp\"\nversion = \"0.0.0\"\nedition = \"2024\"\n").unwrap();
    dir.to_string_lossy().to_string()
}

fn write_src_lib(manifest_dir: &str, src: &str) {
    let mut lib = PathBuf::from(manifest_dir);
    lib.push("src");
    lib.push("lib.rs");
    fs::write(lib, src).unwrap();
}
