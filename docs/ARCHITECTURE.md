# Architecture

This repository is intentionally split into two usage-agnostic distribution surfaces (Rust + Flutter), plus runnable integration examples.

## Scope

- **Package/library code** lives under `rust/` and `flutter/` and must stay usage-agnostic.
- **Runnables and wiring** live under `examples/` and are intentionally excluded from core implementation structure.

## Rust

### Crates

- `oxide_core`: engine primitives (state, reducer contract, snapshot streams, error model, optional persistence).
- `oxide_generator_rs`: procedural macros that generate a binding-friendly surface for reducers/stores.

### Dependency graph

```text
oxide_generator_rs  ──(dev-dependency for trybuild)──▶  oxide_core
oxide_core          ──(no dependency)───────────────▶  oxide_generator_rs
```

### Internal boundaries (oxide_core)

- `engine/`: state ownership + transactional update semantics (dispatch, snapshot broadcast, side-effect loop).
- `runtime.rs`: spawning/initialization utilities (FRB-backed).
- `ffi/`: small adapters intended for binding layers.
- `persistence/` (feature-gated): codec + backend + debounced writer for snapshot persistence.

Maintenance guideline: keep reducer/store semantics in `engine/`; keep platform/runtime concerns in `runtime`, `ffi`, and `persistence`.

## Flutter

### Packages

- `oxide_annotations`: build-time annotations (`@OxideStore`) + small shared helper types used by generated output.
- `oxide_generator`: build_runner / source_gen package that emits `*.oxide.g.dart` glue code.
- `oxide_runtime`: runtime primitives used by generated stores (engine lifecycle, dispatch, snapshot stream coordination).

### Dependency graph

```text
oxide_generator  ───────────────▶  oxide_annotations
oxide_runtime    ───────────────▶  oxide_annotations
oxide_generator  ──(generates code that uses)──▶  oxide_runtime
```

### Internal boundaries (oxide_generator)

- `src/oxide_store_generator.dart`: analyzer-driven extraction of configuration from `@OxideStore`.
- `src/oxide_store_codegen.dart` + `src/codegen/*`: string emission for generated store glue code, split by responsibility:
  - config model, action method emission, backend templates, and orchestration.

### Internal boundaries (oxide_runtime)

- `src/types.dart`: callback contracts supplied by generated bindings (create/dispatch/current/stream/etc).
- `src/store_core.dart`: backend-agnostic core that owns engine lifecycle + snapshot forwarding.
- `src/core.dart`: barrel export to keep public imports stable.

## Testing and Validation

The primary regression signal for refactors is:

- Rust: workspace tests under `rust/` (including feature-gated persistence and wasm compatibility checks).
- Flutter/Dart: package tests under `flutter/oxide_runtime` and `flutter/oxide_generator`, plus analysis for `flutter/oxide_annotations`.

Example app tests under `examples/` are integration-focused and are intentionally treated separately from the core package/library validation.
