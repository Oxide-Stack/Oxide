# counter_app

Minimal example showing basic state management with Oxide + Flutter Rust Bridge (FRB).

## What It Demonstrates

- Rust owns the state (`AppState { counter }`) and applies transitions via a reducer.
- Flutter dispatches typed actions (`Increment`, `Decrement`, `Reset`) into Rust.
- Flutter receives state updates as a typed stream of snapshots (`AppStateSnapshot`).
- Four Flutter adapters for the same store idea (Inherited, Hooks, Riverpod, BLoC) shown as tabs.

## State lifetime (tabs)

Each tab owns its own store instance. If a tab subtree is disposed and rebuilt (common in tab/page views), the store may be disposed and recreated, which resets Rust state back to its initial value.

This example opts into keeping tab state alive by using:

- `keepAlive: true` on the `@OxideStore(...)` declarations so each backend keeps its store alive when possible

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

- Rust reducer + FRB surface: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
