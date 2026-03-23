# QA Script Suite

This directory contains a modular QA pipeline split by test domain.

## Test Surface Covered

- Rust workspace tests (`cargo test --workspace`)
- Rust macro compile tests (`oxide_generator_rs` compile harness)
- Rust feature tests (`navigation-binding`, `isolated-channels`)
- Rust wasm compatibility checks and test compilation (`wasm32-unknown-unknown`, `wasm32-wasip1`)
- Rust coverage gate for `oxide_core` (optional, `cargo-llvm-cov`)
- Flutter package tests (`flutter/oxide_runtime`)
- Flutter package coverage gate for `oxide_runtime`
- Dart package tests (`flutter/oxide_generator`)
- Dart analysis (`flutter/oxide_annotations`)
- Example app suites for:
  - `examples/counter_app`
  - `examples/todos_app`
  - `examples/ticker_app`
  - `examples/benchmark_app`
  - `examples/api_browser_app`
- Per-example steps:
  - `flutter pub get`
  - FRB generation (if `flutter_rust_bridge.yaml` exists)
  - Example Rust tests (if `rust/Cargo.toml` exists)
  - `dart run build_runner build -d`
  - `flutter test`
  - `integration_test/*_test.dart` (optional)

## Orchestration Model

Master scripts:

- `qa.ps1`
- `qa.sh`

Stages:

1. Sequential prerequisites
   - version verification
   - toolchain setup
2. Parallel core suites
   - Rust QA
   - Flutter package QA
3. Example stage
   - runs per-example sequential steps
   - runs multiple example apps in parallel (bounded fan-out)

## Files

- `common.ps1`, `common.sh`: shared helpers
- `00-verify-versions.*`: version sync verification
- `10-setup-toolchains.*`: prerequisite setup
- `20-rust.*`: Rust test suites and coverage gate
- `30-flutter-packages.*`: Flutter/Dart package suites and coverage gate
- `40-example.*`: one example app pipeline
- `50-examples.*`: all example apps with bounded parallelism
- `qa.*`: master orchestrators

## Entry Points

Compatibility wrappers still exist:

- `tools/scripts/qa.ps1`
- `tools/scripts/qa.sh`

These invoke the new master scripts in this directory.
