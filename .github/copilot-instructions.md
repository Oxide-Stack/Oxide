# Project Guidelines

## Code Style
- Keep package code usage-agnostic; place runnable usage under `examples/`.
- Preserve generated files unless regeneration is required by source changes.
- Do not hand-edit `frb_generated.*` or `*.oxide.g.dart` files unless explicitly requested.
- Prefer minimal, targeted edits; avoid broad refactors unrelated to the task.

## Architecture
- `rust/oxide_core`: core runtime and engine semantics (transactional reducer updates, snapshots, optional persistence).
- `rust/oxide_generator_rs`: Rust proc macros (`#[state]`, `#[actions]`, `#[reducer]`, `#[routes]`).
- `flutter/oxide_annotations`: store declaration annotations.
- `flutter/oxide_generator`: build_runner code generation for store adapters.
- `flutter/oxide_runtime`: runtime lifecycle/dispatch/snapshot plumbing used by generated adapters.
- `examples/*`: canonical integration patterns and end-to-end usage.

Key invariant:
- Reducer failures must not partially mutate committed state.

## Build and Test
Run commands from the noted directories.

- Repo-level QA:
  - Windows: `tools/scripts/qa.ps1`
  - Unix: `tools/scripts/qa.sh`

- Rust workspace (`rust/`):
  - `cargo test --workspace`
  - `cargo test -p oxide_core --all-features`

- Flutter packages:
  - `flutter/oxide_runtime`: `flutter test`
  - `flutter/oxide_generator`: `dart test`
  - `flutter/oxide_annotations`: `dart analyze`

- Example app workflow (`examples/<app>/`):
  - `flutter pub get`
  - `dart run build_runner build -d`
  - `flutter test`

- FRB regeneration (when Rust API shape changes):
  - `flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml`

## Conventions
- Always initialize before using Oxide APIs:
  - `await OxideStack.init()` in app startup.
- Feature-gated behavior in Rust is intentional (`navigation-binding`, `isolated-channels`, `state-persistence`); preserve gating patterns.
- For multiple engines in one app, use explicit `@OxideStore` bindings to avoid generated API collisions.
- When changing reducer API or Rust bridge signatures, re-run FRB codegen and verify generated bindings are up to date.
- For deep runtime diagnostics, compile-time Dart flags are available:
  - `OXIDE_DEBUG_LOGS=true`
  - `ENABLE_ADVANCED_LOGS=true`

## Key References
- `README.md`
- `docs/ARCHITECTURE.md`
- `docs/usage/README.md`
- `docs/usage/reducer-pattern.md`
- `docs/usage/init-oxide.md`
- `tools/scripts/qa.ps1`
- `examples/counter_app/lib/src/oxide.dart`
- `examples/counter_app/rust/src/state/app.rs`
