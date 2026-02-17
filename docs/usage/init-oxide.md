# Unified Async Initialization (`initOxide`)

Oxide runs background work via Flutter Rust Bridge (FRB) spawning. This requires a single Rust-side initialization call after `RustLib.init()` and before any engine is created.

## Rust: Expose `init_oxide`

Add an FRB-exposed function in your Rust API module (the one referenced by `flutter_rust_bridge.yaml`):

```rust
#[flutter_rust_bridge::frb]
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
  fn thread_pool() -> oxide_core::runtime::ThreadPool {
    crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
  }
  let _ = oxide_core::runtime::init(thread_pool);
  Ok(())
}
```

## Dart: Call `initOxide` In `main`

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initOxide();
  runApp(const MyApp());
}
```

## Best Practices

- Call `initOxide()` once per app startup (calling multiple times is harmless).
- Create engines only after `initOxide()` completes.
- If you enable optional feature runtimes (for example, isolated channels or navigation), initialize them from the same Rust `init_oxide()` entrypoint.
