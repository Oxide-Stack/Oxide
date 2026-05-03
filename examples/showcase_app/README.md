# showcase_app

End-to-end Oxide showcase application demonstrating the full platform surface:

- Rust-driven navigation (`@OxideApp`, `@OxideRoutePage`, route payloads)
- Reducer state + sliced updates (`#[state(sliced = true)]`, `StateChange::Infer`)
- Isolated channels (event, callbacking, duplex)
- State persistence (`state-persistence` feature + reducer persistence config)
- Multiple generated Flutter backends (Inherited, Hooks, Riverpod, BLoC)

## Feature Tour

1. **Splash -> Home (Rust navigation)**
   - App starts on `SplashScreen` route and transitions using Rust-dispatched actions.
2. **Settings**
   - Change theme and main color.
   - Extract dominant color from an image path.
   - Reset/reload persisted state.
3. **Isolated Channels Demo**
   - Emit Rust -> Dart events.
   - Handle Rust -> Dart -> Rust callback confirmations.
   - Exercise duplex outgoing/incoming channels.
4. **Backend Matrix**
   - Compare the same Rust state/actions across Inherited, Hooks, Riverpod, and BLoC integrations.

## Run Locally (Windows fast gate)

From `examples/showcase_app/`:

```powershell
flutter pub get
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
Set-Location rust; cargo test; Set-Location ..
dart run build_runner build -d
flutter build windows --debug
```

## Start the App

From `examples/showcase_app/`:

```powershell
flutter run -d windows
```

## Notes

- If you change Rust API surface, rerun FRB codegen.
- If you change `@OxideStore` / route annotations, rerun `build_runner`.
- Friction points and workarounds are tracked in the repository root at [`friction.md`](../../friction.md).
