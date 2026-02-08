# todos_app

Focused example showing CRUD operations over a Rust-owned todo list.

## What It Demonstrates

- Add, toggle, and delete todo items via typed Rust actions.
- Receive state updates as a typed snapshot stream (`AppStateSnapshot`).
- Rust-side validation and `NotFound` errors surfaced back to Flutter.
- Rust emits sliced updates so Flutter can efficiently react to partial state changes.

## Rust Surface

- Intended FRB surface: `init_app`, `init_oxide`, the engine type, and the state/action/snapshot types.
- Not part of the FRB surface: reducer implementation structs and internal persistence/side-effect wiring details.

## Sliced Updates

`AppState` is declared with `#[state(sliced = true)]` and exposes two top-level slices:

- `Todos`: the todo list
- `NextId`: the ID counter

The reducer uses `StateChange::Infer` so snapshots carry `snapshot.slices` describing which slice(s) changed without manually listing them.

## Persistence

This example enables `state-persistence` on the Rust side and demonstrates restoring state across engine instances.

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

- Rust FRB surface + reducer composition: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Explicit slice boundaries: [rust/src/state/app_state.rs](./rust/src/state/app_state.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
