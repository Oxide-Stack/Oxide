# Oxide

[![CI](https://github.com/Oxide-Stack/Oxide/actions/workflows/ci.yml/badge.svg)](https://github.com/Oxide-Stack/Oxide/actions/workflows/ci.yml)
[![Release](https://github.com/Oxide-Stack/Oxide/actions/workflows/release.yml/badge.svg)](https://github.com/ORG_OR_USER/Oxide/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)


Oxide is a Rust ↔ Flutter workflow for building apps where:

- Rust owns state and business logic (reducers).
- Flutter stays UI-first and consumes typed bindings plus generated adapters.

This repository is structured so **package code stays usage-agnostic**, and complete runnable usage lives under [examples/](./examples).

Status: `1.0.0`.
## Acknowledgements

Oxide is powered by [Flutter Rust Bridge (FRB)](https://github.com/fzyzcjy/flutter_rust_bridge). Huge thanks to the FRB maintainers and contributors for making Rust ↔ Dart interoperability approachable and productive.
## Why Oxide

- Keep business logic and state invariants in Rust.
- Stream revisioned snapshots to Flutter for reactive UI updates.
- Generate the boring wiring code (InheritedWidget / Riverpod / BLoC adapters) from a small annotation.
- Preserve a key invariant: failed reducer calls must not partially mutate live state.

## Mental Model

Oxide implements a Redux-like unidirectional flow:

```
UI event -> Action -> dispatch(Action) -> reducer(&mut State, Action)
        -> if success: state updated + revision++ + snapshot emitted
        -> Flutter observes snapshot stream -> rebuilds UI
```

Snapshots are revisioned:

```rust
pub struct StateSnapshot<T> {
  pub revision: u64,
  pub state: T,
}
```

## Packages

### Rust

- [oxide_core](./rust/oxide_core) — store/engine primitives, snapshot streams, error model, optional persistence
- [oxide_macros](./rust/oxide_macros) — ergonomic macros for state/actions/reducers (`#[state]`, `#[actions]`, `#[reducer]`)

### Flutter

- [oxide_annotations](./flutter/oxide_annotations) — `@OxideStore(...)` annotation + `OxideBackend` enum
- [oxide_generator](./flutter/oxide_generator) — build_runner generator that produces backend glue (`*.oxide.g.dart`)
- [oxide_runtime](./flutter/oxide_runtime) — small runtime used by generated code (includes Riverpod helpers)

## Examples (Start Here)

- [counter_app](./examples/counter_app) — smallest end-to-end store (counter reducer + snapshot stream)
- [todos_app](./examples/todos_app) — CRUD list state + errors + persistence
- [ticker_app](./examples/ticker_app) — periodic tick dispatch + snapshot stream into Flutter
- [benchmark_app](./examples/benchmark_app) — performance comparison against Dart-only approaches

## Example Demos

- Counter example GIF: (placeholder)
- Todos example GIF: (placeholder)
- Ticker example GIF: (placeholder)
- Benchmark example GIF: (placeholder)

## Benchmark Results

- Benchmark results image: (placeholder)

## Quickstart (Run An Example)

From the repo root:

```bash
cd examples/counter_app
flutter pub get
dart run build_runner build -d
flutter run
```

If you change the Rust API surface, regenerate Flutter Rust Bridge (FRB) bindings (still from the example directory):

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

## How To Implement Oxide In Your App

This section is a practical “wire it up” guide. For fully working code, follow the examples and mirror their structure.

Important: start from an **FRB-integrated Flutter app / template** (or copy one of this repo’s examples). Oxide assumes you already have a working FRB setup for building Rust, generating bindings, and initializing `RustLib` in Flutter.

FRB docs: https://fzyzcjy.github.io/flutter_rust_bridge/

### 1) Rust: Define State, Actions, Reducer

In your Rust crate (the one FRB will bind to), add dependencies:

```toml
[dependencies]
oxide_core = "1.0.0"
oxide_macros = "1.0.0"
```

Then define state/actions and a reducer implementation.

```rust
use oxide_macros::{actions, reducer, state};

#[state]
pub struct AppState {
  pub counter: u64,
}

#[actions]
pub enum AppAction {
  Increment,
}

pub enum AppSideEffect {}

#[derive(Default)]
pub struct AppReducer;

#[reducer(
  engine = AppEngine,
  snapshot = AppStateSnapshot,
  initial = AppState { counter: 0 },
)]
impl oxide_core::Reducer for AppReducer {
  type State = AppState;
  type Action = AppAction;
  type SideEffect = AppSideEffect;

  fn init(
    &mut self,
    _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
  ) {}

  fn reduce(
    &mut self,
    state: &mut Self::State,
    action: Self::Action,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    match action {
      AppAction::Increment => state.counter = state.counter.saturating_add(1),
    }
    Ok(oxide_core::StateChange::FullUpdate)
  }

  fn effect(
    &mut self,
    _state: &mut Self::State,
    _effect: Self::SideEffect,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    Ok(oxide_core::StateChange::None)
  }
}
```

What this gives you:

- A store engine type (to be held behind an `Arc` on the Dart side).
- A snapshot type for `current()` and stream updates.
- An FRB-friendly surface by default (can be disabled with `no_frb`).
- `StateChange` control: return `None` to skip committing/emitting snapshots.
- Rust-side state updates via `SideEffect` messages sent into the engine.

### 2) Rust ↔ Dart: Generate FRB Bindings

Each example in this repo carries a working `flutter_rust_bridge.yaml`. Copy that approach into your app (or copy an example and iterate).

Typical regeneration command:

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

After generation, your Flutter code imports the FRB-generated Dart API (types like `ArcAppEngine`, `AppStateSnapshot`, and functions like `createEngine`, `dispatch`, `current`, `stateStream`, `disposeEngine`).

### 3) Flutter: Add Oxide Packages And Configure Codegen

Add the packages you need in your Flutter app:

- `oxide_annotations` (for `@OxideStore`)
- `oxide_generator` (for build_runner code generation)
- `oxide_runtime` (runtime helpers used by generated code)

In your `pubspec.yaml`:

```yaml
dependencies:
  oxide_annotations: ^1.0.0
  oxide_runtime: ^1.0.0

dev_dependencies:
  build_runner: ^2.4.0
  oxide_generator: ^1.0.0
```

Then run:

```bash
flutter pub get
dart run build_runner build -d
```

### 4) Flutter: Declare A Store With `@OxideStore`

Create a small Dart file declaring the store types and the FRB function names, then run build_runner to generate `*.oxide.g.dart`.

Example `lib/src/oxide.dart` (modeled after [benchmark_app's oxide.dart](./examples/benchmark_app/lib/src/oxide.dart)):

```dart
import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'rust/api/bridge.dart'
    show ArcAppEngine, AppStateSnapshot, createEngine, current, dispatch, disposeEngine, stateStream;
import 'rust/state/app_action.dart';
import 'rust/state/app_state.dart';

part 'oxide.oxide.g.dart';

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.inherited,
  name: 'AppOxide',
)
class AppOxide {}
```

### 5) Use The Generated Adapter In UI

Oxide supports multiple generated “backends” (InheritedWidget, Inherited Hooks, Riverpod, BLoC). Choose the backend via `@OxideStore(backend: ...)`.

At app startup, initialize FRB (your `frb_generated.dart` path may differ depending on your FRB config):

```dart
import 'package:flutter/widgets.dart';

import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}
```

After codegen, all backends expose the same shape in widgets:
`OxideView<State, Actions>`, which contains:
- `state` (nullable until loaded)
- `actions` (typed async methods that dispatch actions into Rust)
- `isLoading` and `error` (engine lifecycle / reducer errors)

Below are the four supported UI wiring styles (pick one).

#### InheritedWidget backend (`OxideBackend.inherited`)

This backend generates:
- `<Name>Controller extends ChangeNotifier`
- `<Name>Scope extends StatefulWidget` (an `InheritedNotifier` provider)
- `<Name>Actions` facade

Wrap your app (or a subtree) with the generated scope:

```dart
import 'package:flutter/material.dart';
import 'package:your_app/src/oxide.dart'; // contains `part 'oxide.oxide.g.dart';`
import 'package:your_app/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(
    AppOxideScope(
      child: const MyApp(),
    ),
  );
}
```

Read state and dispatch actions from any widget:

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:your_app/src/oxide.dart';

class CounterCard extends StatelessWidget {
  const CounterCard({super.key});

  @override
  Widget build(BuildContext context) {
    final view = AppOxideScope.useOxide(context);

    if (view.isLoading) return const CircularProgressIndicator();
    if (view.error != null) return Text('Error: ${view.error}');

    return Column(
      children: [
        Text('Counter: ${view.state?.counter ?? '-'}'),
        FilledButton(
          onPressed: () => unawaited(view.actions.increment()),
          child: const Text('Increment'),
        ),
      ],
    );
  }
}
```

#### Inherited Hooks backend (`OxideBackend.inheritedHooks`)

This backend is the InheritedWidget backend plus a generated hook helper function:
`use<Name>Oxide()`. It’s useful when you want the simplest possible “read view + rebuild on change” experience.

Add dependencies in your app:
- `flutter_hooks`

Declare the store using the hooks backend:

```dart
import 'package:oxide_runtime/oxide_runtime.dart';

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.inheritedHooks,
  name: 'AppHooksOxide',
)
class AppHooksOxide {}
```

Wrap your subtree with the generated scope (same as the plain inherited backend):

```dart
runApp(
  AppHooksOxideScope(
    child: const MyApp(),
  ),
);
```

In a `HookWidget`, read state and dispatch actions via the generated hook:

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:your_app/src/oxide.dart';

class CounterCard extends HookWidget {
  const CounterCard({super.key});

  @override
  Widget build(BuildContext context) {
    final view = useAppHooksOxide();

    if (view.isLoading) return const CircularProgressIndicator();
    if (view.error != null) return Text('Error: ${view.error}');

    return Column(
      children: [
        Text('Counter: ${view.state?.counter ?? '-'}'),
        FilledButton(
          onPressed: () => unawaited(view.actions.increment()),
          child: const Text('Increment'),
        ),
      ],
    );
  }
}
```

#### Riverpod backend (`OxideBackend.riverpod`)

This backend generates an `AutoDisposeNotifierProvider` that yields `OxideView<State, Actions>`.
Add dependencies in your app:
- `flutter_riverpod`

App root:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:your_app/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(
    ProviderScope(
      child: const MyApp(),
    ),
  );
}
```

Read state and send actions from a `ConsumerWidget` / `ConsumerState`:

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:your_app/src/oxide.dart';

class CounterCard extends ConsumerWidget {
  const CounterCard({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(appOxideProvider); // generated provider name

    if (view.isLoading) return const CircularProgressIndicator();
    if (view.error != null) return Text('Error: ${view.error}');

    return Column(
      children: [
        Text('Counter: ${view.state?.counter ?? '-'}'),
        FilledButton(
          onPressed: () => unawaited(ref.read(appOxideProvider).actions.increment()),
          child: const Text('Increment'),
        ),
      ],
    );
  }
}
```

#### BLoC/Cubit backend (`OxideBackend.bloc`)

This backend generates a `Cubit<OxideView<State, Actions>>` plus an `Actions` facade.
Add dependencies in your app:
- `bloc`
- `flutter_bloc`

Provide the generated cubit:

```dart
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:your_app/src/oxide.dart';

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => AppOxideCubit(),
      child: const MaterialApp(home: CounterScreen()),
    );
  }
}
```

Build from the emitted `OxideView` and dispatch via `cubit.actions`:

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
import 'package:your_app/src/oxide.dart';
import 'package:your_app/src/rust/state/app_state.dart';

class CounterScreen extends StatelessWidget {
  const CounterScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final cubit = context.read<AppOxideCubit>();

    return BlocBuilder<AppOxideCubit, OxideView<AppState, AppOxideActions>>(
      builder: (context, view) {
        if (view.isLoading) return const Center(child: CircularProgressIndicator());
        if (view.error != null) return Center(child: Text('Error: ${view.error}'));

        return Center(
          child: FilledButton(
            onPressed: () => unawaited(cubit.actions.increment()),
            child: Text('Counter: ${view.state?.counter ?? '-'} (tap to increment)'),
          ),
        );
      },
    );
  }
}
```

The recommended way to learn the exact UI wiring is to copy an example and adapt it:

- Counter flow: [counter_app](./examples/counter_app)
- Persistence + errors: [todos_app](./examples/todos_app)
- Periodic tick dispatch: [ticker_app](./examples/ticker_app)

## Persistence (Optional)

Persistence is feature-gated in Rust.

Enable it in your Rust dependency:

```toml
[dependencies]
oxide_core = { version = "1.0.0", features = ["state-persistence"] }
```

When working inside this repository, the examples use path dependencies instead.

Then use the persistence options supported by the reducer macro (see the `todos_app` example for a working configuration).

## Development

### Run Tests

- Rust crates: `cargo test` (from `./rust`)
- Flutter runtime package: `flutter test` (from `./flutter/oxide_runtime`)

### Repo Scripts

The repo keeps a single `VERSION` file and syncs versions via scripts under [tools/scripts/](./tools/scripts).

- Version sync: `tools/scripts/version_sync.ps1` / `tools/scripts/version_sync.sh`
- Tests: `tools/scripts/qa.ps1` / `tools/scripts/qa.sh`
- Builds (platform required): `tools/scripts/build.ps1 -Platform windows` / `tools/scripts/build.sh linux`
- Git flow helpers: `tools/scripts/git_flow.ps1`, `tools/scripts/git_flow.sh`

## Versioning

- [VERSION](./VERSION) is the single source of truth.
- Scripts apply it to the Rust workspace, Flutter packages, and example apps.

## Non-Goals (Deferred)

- Offline-first behavior
- State replay/time-travel debugging

## Migration Notes

- Example apps previously named/split differently; use the current `examples/*` apps as the canonical references.
- `@OxideStore.actions` supports both enum actions and union-class actions (depending on your FRB mapping).
- Rust-side persistence is feature-gated; enable `state-persistence` on the relevant crates to use it.

## Contributing

Issues and PRs are welcome. Please:

- Keep package code usage-agnostic; put runnable usage under `examples/`.
- Add or update tests when you change core behavior.
- Run the repo scripts/tests before submitting a PR.

## License

This repo does not currently ship a single root `LICENSE` file.

Some packages include their own license file (for example, `flutter/oxide_runtime/LICENSE`). If you plan to redistribute or publish additional parts of this repo, add a top-level `LICENSE` covering the repository.
