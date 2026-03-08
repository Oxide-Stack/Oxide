# api_browser_app

Real-world example that integrates Oxide with a free public API (JSONPlaceholder) and demonstrates multiple interconnected reducers running in Rust.

## What It Demonstrates

- Multiple Rust reducers (Users → Posts → Comments) with async side-effects and snapshot streaming.
- Cross-store coordination in Flutter (select a user → load posts; select a post → load comments).
- Deterministic integration testing using a local HTTP server (no external network dependency).
- End-to-end Rust ↔ Flutter wiring (FRB + `@OxideStore` + generated glue).

## Navigation Choice

This example uses Oxide's Navigator 1.0 integration for Rust-driven navigation, and also demonstrates plain Flutter `Navigator.push` for an auxiliary “Isolated Channels Demo” page.

## Rust Surface

- Intended FRB surface: no manual init functions (`init_app`/`init_oxide` are now generated). the crate simply exposes its engine and state/action/snapshot types.
- Not part of the FRB surface: reducer implementation structs and side-effect sender plumbing.

## Isolated Channels Demo (Additive)

This example also ships an isolated-channels demo API surface (exercised by the Flutter UI via the “Isolated Channels Demo” toolbar action). It demonstrates:

- Rust → Dart event streaming
- Rust → Dart → Rust callbacks (request/response)
- Simple duplex messaging

To try it:

- Run the app and tap the “Isolated Channels Demo” icon in the top app bar.

Generated Dart wrappers live at:

- `lib/src/rust/api/isolated_channels_bridge.dart`

Example usage (from Flutter code):

```dart
import 'package:api_browser_app/src/oxide.dart';

Future<void> startDemo() async {
  // channel initialization is now automatic; only `OxideStack.init()` is
  // required to boot the runtime.

  // use the unified OxideStack surface for events/callbacks
  OxideStack.events.apiBrowserDemoEvents.listen((event) {
    event.when(notify: (message) => print('notify: $message'));
  });

  OxideStack.callbacks.apiBrowserDemoDialogRequests.listen((pending) async {
    await OxideStack.callbacks.apiBrowserDemoDialogRespond(
      id: pending.id,
      response: ApiBrowserDemoDialogResponse.confirm(true),
    );
  });
}
```

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

## Generate FRB bindings

This repo does not commit the Rust FRB glue (`rust/src/frb_generated.rs`). Run this on a fresh checkout and whenever the Rust API changes:

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
