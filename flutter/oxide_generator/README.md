# oxide_generator

`oxide_generator` is a dev-only build_runner generator that reads `@OxideStore(...)` declarations and emits backend glue as `*.oxide.g.dart`.

This package is intentionally usage-agnostic. For runnable usage, see the example apps in the repository (examples/*) and the root README.

## Add It To Your App

In your Flutter app:

```yaml
dependencies:
  oxide_annotations: ^0.1.0
  oxide_runtime: ^0.1.0

dev_dependencies:
  build_runner: ^2.0.0
  oxide_generator: ^0.1.0
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

## Lifetime / Keep-Alive

`@OxideStore` supports a single lifetime knob: `keepAlive` (default: `false`).

When `keepAlive: true`, the generator keeps the store alive when possible:

- Inherited / Hooks / BLoC: the generated scope widget requests keep-alive so tab/page views do not dispose it when off-screen
- Riverpod: the generator emits a non-autoDispose provider so the store is not disposed when un-watched

### BLoC scopes

For `OxideBackend.bloc`, the generator emits a `...Cubit` and a `...Scope` widget that provides the cubit. The scope uses `flutter_bloc` (`BlocProvider`), so the source file containing your `part '...oxide.g.dart';` must import `package:flutter_bloc/flutter_bloc.dart`.

## Multi-engine bindings

For multi-engine apps, `@OxideStore` supports `bindings` (an import alias string). When provided, default binding method names are qualified automatically so you don’t need to repeat `createEngine/dispatch/current/stateStream/...` for every engine.

Exact output shapes are easiest to understand by inspecting an example’s generated file, e.g.:

- `examples/counter_app/lib/src/oxide.oxide.g.dart`

## License

MIT. See [LICENSE](./LICENSE).
