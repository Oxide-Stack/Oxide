# Validation Report

## 2026-02-08 (Windows + Web/WASM)

### Summary

- Ran Windows tests and builds across the Rust workspace, Flutter packages, and all example apps.
- Verified `oxide_core` compiles for `wasm32-unknown-unknown` and `wasm32-wasip1`, including WASM compatibility test compilation.
- Verified Flutter Web WASM builds for all example apps via FRB `build-web` + wasm-pack.
- Hardened repo scripts so build/test failures fail fast and are less flaky on Windows.

### Commands Executed

- Windows tests (repo script):
  - `pwsh -File .\tools\scripts\qa.ps1 -SkipIntegration`
- Windows builds (repo script):
  - `pwsh -File .\tools\scripts\build.ps1 -Platform windows`
- Rust WASM compilation checks:
  - From `rust/`:
    - `cargo check -p oxide_core --target wasm32-unknown-unknown --all-features`
    - `cargo check -p oxide_core --target wasm32-wasip1 --all-features`
    - `cargo test -p oxide_core --target wasm32-unknown-unknown --all-features --no-run --test wasm_web_compat`
    - `cargo test -p oxide_core --target wasm32-wasip1 --all-features --no-run --test wasm_wasi_compat`
- Flutter Web WASM builds (FRB build-web + wasm-pack):
  - `pwsh -File .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\counter_app`
  - `pwsh -File .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\todos_app`
  - `pwsh -File .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\ticker_app`
  - `pwsh -File .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\benchmark_app`
  - `pwsh -File .\tools\scripts\web_wasm.ps1 -Action build -Path .\examples\api_browser_app`

### Issues Found

#### Windows integration tests exceeded default timeouts

- Symptom: `examples/todos_app/integration_test/persistence_test.dart` could time out during the Windows integration test build/run phase.
- Fix applied:
  - Increased the test timeout to 30 minutes.

#### ticker_app Web WASM build failed with non-Send future

- Symptom: `examples/ticker_app` Rust failed to compile for web WASM due to a non-`Send` future passed to `oxide_core::runtime::spawn`.
- Fix applied:
  - Use `oxide_core::runtime::safe_spawn` on `wasm32-unknown-unknown` (spawn_local) and keep `spawn` only on native targets.

#### oxide_core WASM compilation failures (thread pool + WASI vs web)

- Symptom: `oxide_core` failed to compile for WASM targets due to thread pool trait bounds and web-specific spawn implementation being compiled for WASI.
- Fix applied:
  - Split web-specific spawning (`spawn_local`) to `wasm32-unknown-unknown` only.
  - Adjusted `spawn_blocking` to pass the thread pool in the form expected by FRB for WASM.

#### Repo scripts did not fail fast on command errors

- Symptom: PowerShell scripts could continue after external commands failed, making validation unreliable.
- Fix applied:
  - Added a checked command runner to `qa.ps1` and `build.ps1` so non-zero exit codes stop the script.
  - Updated `build.ps1` to match QA semantics for Flutter packages (runtime: `flutter test`, generator: `dart test`, annotations: `dart analyze`).

### Notes / Warnings

- Rust warnings:
  - `async_fn_in_trait` warning in `oxide_core` (lint-level warning, not a test/build failure).
- wasm-pack warnings:
  - Some runs logged Windows `Access is denied (os error 5)` while finalizing incremental compilation directories; builds still completed successfully.

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
