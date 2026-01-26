# todos_app

Focused example showing CRUD operations over a Rust-owned todo list.

## What It Demonstrates

- Add, toggle, and delete todo items via typed Rust actions.
- Receive state updates as a typed snapshot stream (`AppStateSnapshot`).
- Rust-side validation and `NotFound` errors surfaced back to Flutter.

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

