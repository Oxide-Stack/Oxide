# counter_app

Minimal example showing basic state management with Oxide + Flutter Rust Bridge (FRB).

## What It Demonstrates

- Rust owns the state (`AppState { counter }`) and applies transitions via a reducer.
- Flutter dispatches typed actions (`Increment`, `Decrement`, `Reset`) into Rust.
- Flutter receives state updates as a typed stream of snapshots (`AppStateSnapshot`).
- Four Flutter adapters for the same store idea (Inherited, Hooks, Riverpod, BLoC) shown as tabs.

## Rust Surface

- Intended FRB surface: `init_app`, `init_oxide`, the engine type, and the state/action/snapshot types.
- Not part of the FRB surface: reducer implementation structs and any internal side-effect wiring.

## Isolated Channels Demo (Additive)

This example also ships an isolated-channels demo API surface (feature-gated on the Rust side and exercised by the Flutter UI via the “Channels” tab). It demonstrates:

- Rust → Dart event streaming
- Rust → Dart → Rust callbacks (request/response)
- Simple duplex messaging

To try it:

- Run the app, open the “Channels” tab, and press “Start Demo”.

Generated Dart wrappers live at:

- `lib/src/rust/api/isolated_channels_bridge.dart`

Example usage (from Flutter code):

```dart
import 'package:counter_app/src/rust/api/isolated_channels_bridge.dart' as ch;
import 'package:counter_app/src/rust/isolated_channels_demo/channels.dart';

Future<void> startDemo() async {
  await ch.initIsolatedChannelsDemo();

  ch.counterDemoEventsStream().listen((event) {
    event.when(notify: (message) => print('notify: $message'));
  });

  ch.counterDemoDialogRequestsStream().listen((pending) async {
    await ch.counterDemoDialogRespond(
      id: pending.id,
      response: CounterDemoDialogResponse.confirm(true),
    );
  });
}
```

## Changes Applied

- Hid reducer implementation types from FRB to avoid redundant Dart-visible “reducer” artifacts.
- Removed crate-root Rust re-exports (e.g., tokio watch types) that FRB can accidentally pick up.

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

- Rust FRB surface + engine macro: [rust/src/api/bridge.rs](./rust/src/api/bridge.rs)
- Flutter store declaration (annotation): [lib/src/oxide.dart](./lib/src/oxide.dart)
- Generated glue (do not edit): [lib/src/oxide.oxide.g.dart](./lib/src/oxide.oxide.g.dart)

## Docs

- Root project overview: [README.md](../../README.md)
- Generator docs: [flutter/oxide_generator](../../flutter/oxide_generator)
