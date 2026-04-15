use crate::TEST_ENV_LOCK;
use std::fs;
use std::path::PathBuf;
use syn::ItemImpl;

use super::{
    OxideCallbackArgs, OxideEventChannelArgs, expand_oxide_callback, expand_oxide_event_channel,
};

#[test]
fn event_channel_generates_variant_helpers() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

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
        OxideEventChannelArgs {
            orphaned: false,
            no_frb: true,
        },
        item_impl,
    )
    .unwrap();

    let out = ts.to_string();
    assert!(
        out.contains("pub fn track"),
        "expected track helper, got: {out}"
    );
}

#[test]
fn event_channel_generates_frb_stream_guard_helper() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_event_frb_guard");
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
        OxideEventChannelArgs {
            orphaned: false,
            no_frb: false,
        },
        item_impl,
    )
    .unwrap();

    let out = ts.to_string();
    assert!(
        out.contains("__oxide_require_fresh_event_stream_bindings"),
        "expected event stream FRB guard helper, got: {out}"
    );
    assert!(
        out.matches("__oxide_require_fresh_event_stream_bindings")
            .count()
            >= 2,
        "expected event stream path and helper declaration, got: {out}"
    );
}

#[test]
fn callbacking_enforces_variant_parity() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

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
        message.contains("missing Response variant")
            || message
                .contains("Response type must be an enum defined in the current crate `src/`"),
        "expected parity error, got: {message}"
    );
}

#[test]
fn callbacking_generates_methods_and_runtime_module() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_callback_ok");
    write_src_lib(
        &dir,
        r#"
        pub enum DialogRequest {
            Confirm { title: String },
            Ping,
        }

        pub enum DialogResponse {
            Confirm(bool),
            Ping,
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

    let ts = expand_oxide_callback(OxideCallbackArgs { no_frb: true }, item_impl).unwrap();
    let out = ts.to_string();
    assert!(
        out.contains("pub async fn confirm"),
        "expected confirm method, got: {out}"
    );
    assert!(
        out.contains("pub async fn ping"),
        "expected ping method, got: {out}"
    );
    assert!(out.contains("__oxide_isolated_callback_dialog_service"));
}

#[test]
fn callbacking_frb_stream_includes_sink_guard_helper() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_callback_guard");
    write_src_lib(
        &dir,
        r#"
        pub enum DialogRequest {
            Confirm { title: String },
        }

        pub enum DialogResponse {
            Confirm(bool),
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

    let ts = expand_oxide_callback(OxideCallbackArgs { no_frb: false }, item_impl).unwrap();
    let out = ts.to_string();
    let compact: String = out.chars().filter(|c| !c.is_whitespace()).collect();

    assert!(
        compact.contains("fn__oxide_callback_require_fresh_frb_bindings"),
        "expected callback FRB guard helper, got: {out}"
    );
    assert!(
        compact.contains("__oxide_callback_require_fresh_frb_bindings(&sink,envelope)"),
        "expected stream sink to use callback guard helper, got: {out}"
    );
}

#[test]
fn callbacking_rejects_unsupported_args_and_non_callback_trait() {
    let bad_args = match syn::parse_str::<OxideCallbackArgs>("other = true") {
        Ok(_) => panic!("expected parse error"),
        Err(err) => err.to_string(),
    };
    assert!(bad_args.contains("unsupported arguments"));

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl core::fmt::Display for DialogService {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "x") }
        }
        "#,
    )
    .unwrap();
    let err = expand_oxide_callback(OxideCallbackArgs { no_frb: true }, item_impl)
        .unwrap_err()
        .to_string();
    assert!(err.contains("must be applied to an impl of OxideCallbacking"));
}

#[test]
fn event_channel_duplex_generates_send_and_register_helpers() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_duplex");
    write_src_lib(
        &dir,
        r#"
        pub enum OutgoingEvent {
            Tick(u64),
        }

        pub enum IncomingEvent {
            Start,
        }
        "#,
    );

    unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl oxide_core::OxideEventDuplexChannel for DuplexChannel {
            type Outgoing = OutgoingEvent;
            type Incoming = IncomingEvent;
        }
        "#,
    )
    .unwrap();

    let ts = expand_oxide_event_channel(
        OxideEventChannelArgs {
            orphaned: true,
            no_frb: true,
        },
        item_impl,
    )
    .unwrap();

    let out = ts.to_string();
    assert!(
        out.contains("pub fn send"),
        "expected send helper, got: {out}"
    );
    assert!(
        out.contains("register_incoming"),
        "expected register helper, got: {out}"
    );
}

#[test]
fn event_channel_duplex_generates_outgoing_frb_guard_helper() {
    let _guard = TEST_ENV_LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .unwrap();

    let dir = make_temp_manifest_dir("oxide_isolated_channels_duplex_frb_guard");
    write_src_lib(
        &dir,
        r#"
        pub enum OutgoingEvent {
            Tick(u64),
        }

        pub enum IncomingEvent {
            Start,
        }
        "#,
    );

    unsafe { std::env::set_var("CARGO_MANIFEST_DIR", &dir) };

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl oxide_core::OxideEventDuplexChannel for DuplexChannel {
            type Outgoing = OutgoingEvent;
            type Incoming = IncomingEvent;
        }
        "#,
    )
    .unwrap();

    let ts = expand_oxide_event_channel(
        OxideEventChannelArgs {
            orphaned: false,
            no_frb: false,
        },
        item_impl,
    )
    .unwrap();

    let out = ts.to_string();
    assert!(
        out.contains("__oxide_require_fresh_duplex_outgoing_bindings"),
        "expected duplex outgoing FRB guard helper, got: {out}"
    );
    assert!(
        out.matches("__oxide_require_fresh_duplex_outgoing_bindings")
            .count()
            >= 2,
        "expected duplex outgoing stream path and helper declaration, got: {out}"
    );
}

#[test]
fn event_channel_rejects_invalid_args_and_non_channel_trait() {
    let bad_args = match syn::parse_str::<OxideEventChannelArgs>("orphaned = 1") {
        Ok(_) => panic!("expected parse error"),
        Err(err) => err.to_string(),
    };
    assert!(!bad_args.is_empty());

    let item_impl: ItemImpl = syn::parse_str(
        r#"
        impl core::fmt::Display for AnalyticsChannel {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "x") }
        }
        "#,
    )
    .unwrap();

    let err = expand_oxide_event_channel(
        OxideEventChannelArgs {
            orphaned: false,
            no_frb: true,
        },
        item_impl,
    )
    .unwrap_err()
    .to_string();
    assert!(
        err.contains("must be applied to an impl of OxideEventChannel or OxideEventDuplexChannel")
    );
}

fn make_temp_manifest_dir(name: &str) -> String {
    let mut dir = std::env::temp_dir();
    dir.push(format!("{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"tmp\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    dir.to_string_lossy().to_string()
}

fn write_src_lib(manifest_dir: &str, src: &str) {
    let mut lib = PathBuf::from(manifest_dir);
    lib.push("src");
    lib.push("lib.rs");
    fs::write(lib, src).unwrap();
}
