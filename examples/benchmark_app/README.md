# benchmark_app

Benchmark example that compares “Dart-only” state updates vs Rust/Oxide-backed state updates.

## What It Demonstrates

- A workload/variant dashboard that makes it explicit what is currently being benchmarked.
- Fair(er) measurement practices: warmup runs, multiple samples, and sequential execution.
- Comparing “Dart-only” state updates vs Rust/Oxide-backed state updates across Riverpod/BLoC/Hooks.
- End-to-end Rust ↔ Flutter wiring (FRB + `@OxideStore` + generated glue) in a performance-focused app.
- Multi-engine store declarations using `bindings` to avoid repeating FRB method names.
- A deterministic CPU-bound benchmark (“sieve”) implemented in both Dart-only and Rust-backed variants.
- Tabular summaries (median/mean/p95) plus a dedicated charts view for median/mean/p95 bars.
- Charts view includes run context (iterations/samples/warmup) and a quick fastest/slowest summary.

## Notes on fairness / interpretation

- Rust variants batch work inside a single action via an `iterations` parameter. This reduces FFI crossings compared to repeating single-iteration actions and will affect “end-to-end” wall times.
- The dashboard reports end-to-end wall time per action (including dispatch + state update + a frame boundary). Use this to compare how each variant behaves inside a Flutter app.

## Navigation Choice

This example uses `GoRouter` (`MaterialApp.router`) for app-level routing, and wires the generated `oxideNavigatorKey` into the router so Oxide can execute Rust-emitted navigation commands through the underlying Navigator.

## Run

```bash
flutter pub get
dart run build_runner build -d
flutter run
```

## Generate FRB bindings

This repo does not commit the Rust FRB glue (`rust/src/frb_generated.rs`). Run this on a fresh checkout and whenever the Rust API changes:

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

## Key Files

- Rust FRB init hook: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Rust engines (multi-reducer / multi-engine): [counter_bridge.rs](./rust/src/api/counter_bridge.rs), [json_bridge.rs](./rust/src/api/json_bridge.rs), [sieve_bridge.rs](./rust/src/api/sieve_bridge.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Benchmark results and interpretation: [docs/BENCHMARKS.md](../../docs/BENCHMARKS.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
