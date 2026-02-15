# api_browser_app

Real-world example that integrates Oxide with a free public API (JSONPlaceholder) and demonstrates multiple interconnected reducers running in Rust.

## What It Demonstrates

- Multiple Rust reducers (Users → Posts → Comments) with async side-effects and snapshot streaming.
- Cross-store coordination in Flutter (select a user → load posts; select a post → load comments).
- Deterministic integration testing using a local HTTP server (no external network dependency).
- End-to-end Rust ↔ Flutter wiring (FRB + `@OxideStore` + generated glue).

## Rust Surface

- Intended FRB surface: `init_app`, `init_oxide`, and the three engine + state/action/snapshot type sets.
- Not part of the FRB surface: reducer implementation structs and side-effect sender plumbing.

## Changes Applied

- Fixed broken Rust imports and removed unnecessary “sink” indirection in reducer side-effect wiring.
- Implemented Rust-side HTTP for browser WebAssembly builds and validated `build-web` output.
- Simplified Flutter store declarations by keeping only the Riverpod backend where applicable.

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

- Rust FRB init hook: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Rust engines: [users_bridge.rs](./rust/src/api/users_bridge.rs), [posts_bridge.rs](./rust/src/api/posts_bridge.rs), [comments_bridge.rs](./rust/src/api/comments_bridge.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart) (created by `build_runner`)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
