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

## Best Practices

- Call `OxideStack.init()` once per app startup.
- Do not access `OxideStack.navigation` or create any Oxide engines before initialization.
