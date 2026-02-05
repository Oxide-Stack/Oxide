# ticker_app

Focused example showing async Rust work streaming updates into Flutter.

## What It Demonstrates

- Rust spawns a background Tokio task on `StartTicker` that periodically emits side-effects to drive state updates.
- Flutter can always dispatch `ManualTick` to increment immediately.
- Rust can apply a tick through the side-effect pipeline (`EmitSideEffectTick` → `effect()`).
- Flutter subscribes to a typed FRB stream of `AppStateSnapshot`.
- Flutter can start/stop ticking and reset ticks (the default interval is 1000ms).

## Rust Surface

- Intended FRB surface: `init_app`, `init_oxide`, the engine type, and the state/action/snapshot types.
- Not part of the FRB surface: reducer implementation structs, ticker task internals, and side-effect plumbing.

## Changes Applied

- Removed unnecessary reducer exposure (no Dart-visible reducer instance / RustOpaque type).
- Removed crate-root Rust re-exports that FRB can accidentally pick up (e.g., tokio watch types).

## Why the side-effect test was failing (earlier)

The original test flake came from storing the side-effect sender in a single global slot: concurrent tests could overwrite it with a different engine’s sender, causing the tick to apply to the wrong engine (or none) and time out.

The fix is to keep the side-effect sender on the reducer instance (per engine) and start the auto-tick thread from that same reducer, so there is no cross-engine global sender to race.



## Run

```bash
flutter pub get
dart run build_runner build -d
flutter run
```

## Regenerate FRB bindings (if Rust API changes)

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

## Key Files

- Rust ticker logic + FRB surface: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
