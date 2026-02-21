# ticker_app

Focused example showing async Rust work streaming updates into Flutter.

## What It Demonstrates

- Rust spawns a background Tokio task on `StartTicker` that periodically emits side-effects to drive state updates.
- Flutter can always dispatch `ManualTick` to increment immediately.
- Rust can apply a tick through the side-effect pipeline (`EmitSideEffectTick` → `effect()`).
- Flutter subscribes to a typed FRB stream of `AppStateSnapshot`.
- Flutter can start/stop ticking and reset ticks (the default interval is 1000ms).
- Rust emits sliced updates so Flutter can efficiently react to partial state changes.

## Navigation Choice

This example uses Oxide's Navigator 1.0 integration by wiring the generated `oxideNavigatorKey` into `MaterialApp`. Rust emits navigation commands, and Flutter executes them through the Navigator-backed handler.

## Rust Surface

- Intended FRB surface: `init_app`, `init_oxide`, the engine type, and the state/action/snapshot types.
- Not part of the FRB surface: reducer implementation structs, ticker task internals, and side-effect plumbing.

## Sliced Updates

`AppState` is declared with `#[state(sliced = true)]` and uses two explicit top-level slices:

- `Control`: ticker runtime control (`is_running`, `interval_ms`)
- `Tick`: tick counter + metadata (`ticks`, `last_tick_source`)

The reducer uses `StateChange::Infer` so snapshots carry `snapshot.slices` describing which slice(s) changed without manually listing them.

## Notes

- The auto-tick background task and side-effect sender are per-engine (no shared global sender), avoiding cross-engine test flakes.

## Why the side-effect test was failing (earlier)

The original test flake came from storing the side-effect sender in a single global slot: concurrent tests could overwrite it with a different engine’s sender, causing the tick to apply to the wrong engine (or none) and time out.

The fix is to keep the side-effect sender on the reducer instance (per engine) and start the auto-tick thread from that same reducer, so there is no cross-engine global sender to race.



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

- Rust FRB surface (thin glue): [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Explicit slice boundaries: [rust/src/state/app_state.rs](./rust/src/state/app_state.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
