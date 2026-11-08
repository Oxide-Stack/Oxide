# {{app_name}}

A new [Oxide](https://github.com/Oxide-Stack/Oxide) + `flutter_rust_bridge`
counter app (backend: {{backend}}, Rust crate: `{{rust_crate}}`).

## Prerequisites

- Flutter SDK + Dart
- Rust toolchain (`cargo`)
- `flutter_rust_bridge_codegen` 2.12.0:
  `cargo install flutter_rust_bridge_codegen --version 2.12.0`

## Run

```sh
flutter pub get
flutter_rust_bridge_codegen generate
dart run build_runner build --delete-conflicting-outputs
flutter run
```

## Layout

- `rust/src/state/` — `AppState` (`#[state]`) and `AppAction` (`#[actions]`).
- `rust/src/api/bridge.rs` — `#[reducer]` engine/snapshot/init wiring.
- `lib/src/oxide.dart` — the single `@OxideStore` declaration.
- `lib/src/counter_page.dart` — minimal counter UI (Increment/Decrement/Reset).
- `integration_test/oxide_test.dart` — dispatch round-trip test.

## Regenerate

```sh
oxide generate
```
