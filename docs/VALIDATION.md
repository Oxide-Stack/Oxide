# Validation Report

## 2026-02-05 (Windows)

### Summary

- Regenerated FRB bindings after Rust↔Flutter API boundary fixes.
- Ran Rust tests for the workspace and affected example crates.
- Ran Flutter unit tests and Windows integration tests for `api_browser_app` and `ticker_app`.
- Verified generated Flutter bindings no longer expose Rust channel endpoints (e.g., `UnboundedSender`).

### Commands Executed

- Rust:
  - `cargo test --workspace` (from `rust/`)
  - `cargo test` (from `examples/api_browser_app/rust`)
  - `cargo test` (from `examples/ticker_app/rust`)
- FRB regeneration:
  - `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml` (from `examples/api_browser_app`)
  - `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml` (from `examples/ticker_app`)
- Flutter unit tests:
  - `flutter test` (from `examples/api_browser_app`)
  - `flutter test` (from `examples/ticker_app`)
- Integration tests (Windows):
  - `flutter test integration_test/simple_test.dart -d windows` (from `examples/api_browser_app`)
  - `flutter test integration_test/simple_test.dart -d windows` (from `examples/ticker_app`)

### Issues Found

#### Rust channel sender surfaced in generated Flutter bindings

- Symptom: FRB generated Dart-visible artifacts exposing `UnboundedSender<...>` (and previously, reducer models containing sender handles).
- Root cause: Reducer internals (side-effect dispatch wiring) were located in the FRB input module and were being reflected into generated bindings.
- Fix applied:
  - Made reducer internals non-exported and removed the crate-local `UnboundedSender<T>` workaround.
  - Regenerated FRB bindings and removed stale generated Dart artifacts that exposed sender handles.
- Verification:
  - `cargo test` passed for the Rust workspace and affected examples.
  - `flutter test` + Windows integration tests passed for `api_browser_app` and `ticker_app`.

#### api_browser_app Windows integration build PDB lock

- Symptom: Windows build failed with `error C1041: cannot open program database ... flutter_wrapper_app.pdb`.
- Fix applied:
  - Re-ran the integration test after `flutter clean`.

### Notes / Warnings

- Rust warnings:
  - `async_fn_in_trait` warning in `oxide_core` (lint-level warning, not a test failure).
- `dart pub publish --dry-run` / `flutter pub publish --dry-run` exits with a non-zero code when warnings are present (e.g., dirty git state, dependency overrides). Archives were generated and validated, but publishing from a clean git state is recommended.
