# Generate FRB Bindings (Rust ↔ Dart)

Each example in this repo carries a working `flutter_rust_bridge.yaml`. Copy that approach into your app (or copy an example and iterate).

Typical regeneration command (run from your Flutter app directory):

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

## Troubleshooting: `StreamSink<...>::add` Trait-Bound Errors

If Rust compilation fails with trait-bound errors around generated stream sinks, for example:

- `StreamSink<OxideNavCommand>::add(...)`
- `StreamSink<YourEvent>::add(...)` from `#[oxide_event_channel]`
- `IntoIntoDart` for generated isolated-channel payload types

your FRB bindings are usually stale or out of sync with generated Oxide APIs (navigation or isolated channels).

Regenerate FRB bindings from your Flutter app root, then rebuild:

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

If you just changed isolated-channel declarations (`#[oxide_event_channel]` / `#[oxide_callback]`), always run regeneration before `cargo check`/`cargo test` in the Rust crate.

For callback channels, prefer `#[oxide_callback(no_frb)]` plus explicit bridge functions in your API module for a stable FRB surface.

## Reducer Struct Shape: Avoid Unit Structs

If your reducer is declared as a unit struct (for example `pub struct AppReducer;`), `flutter_rust_bridge_codegen` may crash or skip it.

Symptom:

```text
Skip parsing enum_or_struct ... AppReducer ... struct with unit fields are not supported yet
...
panicked ... no entry found for key=MirStructIdent(... AppReducer ...)
```

Fix: change the reducer declaration to a braced struct:

```rust
#[derive(Default)]
pub struct AppReducer {}
```

Alternative: mark it as opaque (FRB will treat it as an opaque handle):

```rust
#[flutter_rust_bridge::frb(opaque)]
pub struct AppReducer;
```

## Re-export `OxideError` For FRB

Make sure `OxideError` is re-exported from the Rust module tree that FRB scans (based on your `flutter_rust_bridge.yaml` `rust_input`; the examples use `rust_input: crate`).

At minimum, re-export it from your crate root (`src/lib.rs`):

```rust
pub use oxide_core::OxideError;
```

If your `rust_input` points at an `api` module, it’s also common to re-export `OxideError` from within that module tree (for example `src/api/bridge.rs`), mirroring this repo’s examples.

Isolated-channel errors now use `oxide_core::OxideChannelError` in generated signatures, so they do **not** require a crate-root re-export.

## What Dart Gets

After generation, your Flutter code imports the FRB-generated Dart API. For a typical store you’ll see:

- types like `ArcAppEngine`, `AppStateSnapshot`
- functions like `createEngine`, `dispatch`, `current`, `stateStream`, `disposeEngine`
