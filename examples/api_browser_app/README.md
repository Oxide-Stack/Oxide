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
import 'package:api_browser_app/src/rust/api/isolated_channels_bridge.dart' as ch;
import 'package:api_browser_app/src/rust/isolated_channels_demo/channels.dart';

Future<void> startDemo() async {
  await ch.initIsolatedChannelsDemo();

  ch.apiBrowserDemoEventsStream().listen((event) {
    event.when(notify: (message) => print('notify: $message'));
  });

  ch.apiBrowserDemoDialogRequestsStream().listen((pending) async {
    await ch.apiBrowserDemoDialogRespond(
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
