# Initialization (`OxideStack.init`)

Oxide runs background work through Flutter Rust Bridge (FRB) spawning. In navigation-enabled apps, the generated Rust init hook runs as part of `RustLib.init()`, and Dart calls a single entrypoint, `OxideStack.init`, from `main()` before using Oxide APIs.

## Rust: Use the Macro-Generated Init Hook

If your crate uses the `#[oxide_generator_rs::routes]` macro, Oxide emits an FRB init hook at build time. You do not need to hand-write `init_app` or `init_oxide`.

## Dart: Call `OxideStack.init()` In `main`

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  runApp(const MyApp());
}
```

`OxideStack.init()` calls `RustLib.init()` and starts the generated navigation runtime by default.

## Dart: Or Use `runOxideApp`

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await runOxideApp(const MyApp());
}
```

`runOxideApp` wraps `OxideStack.init()` and `runApp(...)`, and takes the same `startNavigation` flag.

## Rules

- Call `OxideStack.init()` once per app startup.
- Do not access `OxideStack.navigation` or create any Oxide engines before initialization.

## Logging Usage

Oxide emits structured runtime logs for startup, dispatch, and error paths. Use the compile-time Dart flags below when you need extra detail or want to inspect state transitions.

### Debug Logging Flags

Oxide exposes these compile-time Dart flags:

- `OXIDE_DEBUG_LOGS=true`: enables explicit debug logs in release builds.
- `ENABLE_ADVANCED_LOGS=true`: enables verbose transition payload logs (for example `before -> action -> after`).

Example:

```bash
flutter run --dart-define=OXIDE_DEBUG_LOGS=true --dart-define=ENABLE_ADVANCED_LOGS=true
```

Notes:

- In debug/profile builds, baseline Oxide logs are already available.
- `ENABLE_ADVANCED_LOGS` is intended for deep diagnostics and can produce large logs because it includes state snapshots and action payloads.

### Persistence Debug JSON

Oxide can mirror persisted snapshots to a validated debug JSON copy. The default policy is `auto`, which enables the copy in debug/profile builds and disables it in release builds.

Supported compile-time flag:

- `OXIDE_DEBUG_JSON=auto` (default): follow the build mode.
- `OXIDE_DEBUG_JSON=on`: force persistence debug JSON on in any build.
- `OXIDE_DEBUG_JSON=off`: force persistence debug JSON off in any build.

Example:

```bash
flutter run --dart-define=OXIDE_DEBUG_JSON=off
```

`OxideStack.init()` applies this policy automatically during startup.
