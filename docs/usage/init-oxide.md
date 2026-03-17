# Initialization (`OxideStack.init`)

Oxide runs background work using Flutter Rust Bridge (FRB) spawning. The recommended setup is to let a macro-generated Rust init hook run as part of `RustLib.init()`, and expose a single Dart entrypoint (`OxideStack.init`) that must be called from `main()` before using Oxide APIs.

## Rust: Use the Macro-Generated Init Hook

If your crate uses the `#[oxide_generator_rs::routes]` macro (navigation-enabled apps), Oxide emits an FRB init hook at build time. You do not need to hand-write `init_app` / `init_oxide` functions in your crate.

## Dart: Call `OxideStack.init()` In `main`

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  runApp(const MyApp());
}
```

`OxideStack.init()` calls `RustLib.init()` and (by default) starts the generated navigation runtime.

## Dart: Or Use `runOxideApp`

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await runOxideApp(const MyApp());
}
```

`runOxideApp` wraps `OxideStack.init()` and `runApp(...)`, and supports the same `startNavigation` flag.

## Best Practices

- Call `OxideStack.init()` once per app startup.
- Do not access `OxideStack.navigation` or create any Oxide engines before initialization.

## Debug Logging Flags

Oxide exposes compile-time Dart flags for structured runtime logging:

- `OXIDE_DEBUG_LOGS=true`: enables explicit debug logs in release builds.
- `ENABLE_ADVANCED_LOGS=true`: enables verbose transition payload logs (for example `before -> action -> after`).

Common usage:

```bash
flutter run --dart-define=OXIDE_DEBUG_LOGS=true --dart-define=ENABLE_ADVANCED_LOGS=true
```

Notes:

- In debug/profile builds, baseline Oxide logs are already available.
- `ENABLE_ADVANCED_LOGS` is intended for deep diagnostics and can produce large logs because it includes state snapshots and action payloads.
