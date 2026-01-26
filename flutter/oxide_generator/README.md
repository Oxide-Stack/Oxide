# oxide_generator

`oxide_generator` is a dev-only build_runner generator that reads `@OxideStore(...)` declarations and emits backend glue as `*.oxide.g.dart`.

This package is intentionally usage-agnostic. For runnable usage, see the example apps in the repository (examples/*) and the root README.

## Add It To Your App

In your Flutter app:

```yaml
dependencies:
  oxide_annotations: ^1.0.0
  oxide_runtime: ^1.0.0

dev_dependencies:
  build_runner: ^2.0.0
  oxide_generator: ^1.0.0
```

Then fetch packages:

```bash
flutter pub get
```

## Declare A Store

Create a Dart file with:

- a `part 'your_file.oxide.g.dart';`
- an empty class annotated with `@OxideStore(...)`

Example (from `counter_app`):

```dart
part 'oxide.oxide.g.dart';

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
)
class StateBridgeOxide {}
```

## Run Code Generation

```bash
dart run build_runner build -d
```

This writes `*.oxide.g.dart` next to the source file.

## What Gets Generated

The generator emits backend-specific glue. Depending on `backend: OxideBackend...`, this typically includes:

- A controller that manages engine lifecycle + snapshot stream consumption
- An actions facade that calls into the FRB-generated `dispatch(...)`
- A UI adapter (scope/widget/provider) that exposes an `OxideView<State, Actions>` or equivalent access pattern

Exact output shapes are easiest to understand by inspecting an example’s generated file, e.g.:

- `examples/counter_app/lib/src/oxide.oxide.g.dart`

## License

MIT. See [LICENSE](./LICENSE).
