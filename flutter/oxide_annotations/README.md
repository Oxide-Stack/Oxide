# oxide_annotations

`oxide_annotations` defines the `@OxideStore(...)` annotation consumed by `oxide_generator`.

This package is intentionally minimal and usage-agnostic. For runnable usage, see the example apps in the repository (examples/*) and the root README.

## What It Provides

- `@OxideStore(...)`: declares the Rust-backed store types and FRB function names
- `OxideBackend`: selects which Flutter adapter to generate (`inherited`, `inheritedHooks`, `riverpod`, `bloc`)
- `OxideView<S, A>`: a small UI-facing container type used by generated backends

## Usage

In a Flutter app that also includes FRB-generated Dart bindings for your Rust engine:

```dart
import 'rust/api/bridge.dart' show ArcAppEngine, AppStateSnapshot;
import 'rust/state/app_action.dart';
import 'rust/state/app_state.dart';

part 'oxide.oxide.g.dart';

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.inherited,
)
class AppOxide {}
```

Then run build_runner (with `oxide_generator` configured) to generate `oxide.oxide.g.dart`.

For a complete working example, see:

- `examples/counter_app/lib/src/oxide.dart`

## Annotation Fields

### Types

- `state`: the FRB-mapped Rust state type
- `snapshot`: the FRB-mapped snapshot type (must contain `revision` and `state`)
- `actions`: the FRB-mapped Rust actions type (enum or union class, depending on your FRB mapping)
- `engine`: the FRB-mapped engine handle type (typically `Arc...`)

### Backend

- `backend`: which adapter to generate (`OxideBackend.inherited` by default)

### FRB Function Names

These default to the common Oxide macro-generated surface:

- `createEngine` (default: `createEngine`)
- `disposeEngine` (default: `disposeEngine`)
- `dispatch` (default: `dispatch`)
- `stateStream` (default: `stateStream`)
- `current` (default: `current`)
- `initApp` (optional)

### Persistence Hooks (Optional)

If your Rust surface exposes persistence functions, you can provide:

- `encodeCurrentState`
- `encodeState`
- `decodeState`

### Naming (Optional)

- `name`: allows overriding the generated symbol prefixes when you want multiple stores in one app.

## License

MIT. See [LICENSE](./LICENSE).
