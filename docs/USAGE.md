# Oxide Usage Guide

For fully working code, follow the examples and mirror their structure:

- [counter_app](../examples/counter_app)
- [todos_app](../examples/todos_app)
- [ticker_app](../examples/ticker_app)
- [benchmark_app](../examples/benchmark_app)
- Benchmarks write-up: [BENCHMARKS.md](./BENCHMARKS.md)

Important: start from an **FRB-integrated Flutter app / template** (or copy one of this repo’s examples). Oxide assumes you already have a working Flutter Rust Bridge (FRB) setup for building Rust, generating bindings, and initializing `RustLib` in Flutter.

FRB docs: https://fzyzcjy.github.io/flutter_rust_bridge/

## 1) Rust: Define State, Actions, Reducer

In your Rust crate (the one FRB will bind to), add dependencies:

```toml
[dependencies]
oxide_core = "0.1.0"
oxide_generator_rs = "0.1.0"
```

When working inside this repository, use combined version + path dependencies (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.1.0", path = "../rust/oxide_core" }
oxide_generator_rs = { version = "0.1.0", path = "../rust/oxide_generator_rs" }
```

Then define state/actions and a reducer implementation.

```rust
use oxide_generator_rs::{actions, reducer, state};

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
pub struct AppReducer {}

#[reducer(
  engine = AppEngine,
  snapshot = AppStateSnapshot,
  initial = AppState { counter: 0 },
)]
impl oxide_core::Reducer for AppReducer {
  type State = AppState;
  type Action = AppAction;
  type SideEffect = AppSideEffect;

  async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

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

## 2) Rust ↔ Dart: Generate FRB Bindings

Each example in this repo carries a working `flutter_rust_bridge.yaml`. Copy that approach into your app (or copy an example and iterate).

Typical regeneration command:

```bash
flutter_rust_bridge_codegen generate --config-file flutter_rust_bridge.yaml
```

Common error: FRB crash on unit structs

If your reducer is declared as a unit struct (e.g. `pub struct AppReducer;`), `flutter_rust_bridge_codegen` may log something like:

```text
Skip parsing enum_or_struct ... AppReducer ... struct with unit fields are not supported yet
...
panicked ... no entry found for key=MirStructIdent(... AppReducer ...)
```

Fix: change the reducer declaration to a braced struct:

```rust
#[derive(Default)]
pub struct AppReducer {}
```

Alternatively, mark it as opaque (FRB will treat it as an opaque handle):

```rust
#[flutter_rust_bridge::frb(opaque)]
pub struct AppReducer;
```

Important: make sure `OxideError` is re-exported from the Rust module tree that FRB scans (based on your `flutter_rust_bridge.yaml` `rust_input`; the examples use `rust_input: crate::api`).

At minimum, re-export it from your crate root (`src/lib.rs`):

```rust
pub use oxide_core::OxideError;
```

If your `rust_input` points at an `api` module, it’s also common to re-export `OxideError` from within that module tree (e.g. `src/api/bridge.rs`), mirroring this repo’s examples.

After generation, your Flutter code imports the FRB-generated Dart API (types like `ArcAppEngine`, `AppStateSnapshot`, and functions like `createEngine`, `dispatch`, `current`, `stateStream`, `disposeEngine`).

## 2.1) Unified Async Initialization (initOxide)

Oxide runs all background work via Flutter Rust Bridge (FRB) spawning. This requires a single Rust-side initialization call **after** `RustLib.init()` and **before** any engine is created.

### Rust: expose initOxide

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

### Dart: call initOxide in main

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initOxide();
  runApp(const MyApp());
}
```

### Best practices

- Call `initOxide()` once per app startup (calling multiple times is harmless).
- Create engines only after `initOxide()` completes.
- For persistence/file I/O on native platforms, Oxide uses FRB blocking spawning.
## 3) Flutter: Add Oxide Packages And Configure Codegen

Add the packages you need in your Flutter app:

- `oxide_annotations` (for `@OxideStore`)
- `oxide_generator` (for build_runner code generation)
- `oxide_runtime` (runtime helpers used by generated code)

In your `pubspec.yaml`:

```yaml
dependencies:
  oxide_annotations: ^0.1.0
  oxide_runtime: ^0.1.0

dev_dependencies:
  build_runner: ^2.4.0
  oxide_generator: ^0.1.0
```

Then run:

```bash
flutter pub get
dart run build_runner build -d
```

## 4) Flutter: Declare A Store With `@OxideStore`

Create a small Dart file declaring the store types and the FRB function names, then run build_runner to generate `*.oxide.g.dart`.

Example `lib/src/oxide.dart` (modeled after [benchmark_app's oxide.dart](../examples/benchmark_app/lib/src/oxide.dart)):

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

## 5) Use The Generated Adapter In UI

Oxide supports multiple generated backends (InheritedWidget, Inherited Hooks, Riverpod, BLoC). Choose the backend via `@OxideStore(backend: ...)`.

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

After codegen, all backends expose the same shape in widgets: `OxideView<State, Actions>`, which contains:

- `state` (nullable until loaded)
- `actions` (typed async methods that dispatch actions into Rust)
- `isLoading` and `error` (engine lifecycle / reducer errors)

Below are the four supported UI wiring styles (pick one).

### InheritedWidget backend (`OxideBackend.inherited`)

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

### Inherited Hooks backend (`OxideBackend.inheritedHooks`)

This backend is the InheritedWidget backend plus a generated hook helper function.

Naming note: the generated hook function appends `Oxide` to the store class name. If your store name already ends with `Oxide`, the generated hook ends up with `...OxideOxide()` (this is expected).

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
    final view = useAppHooksOxideOxide();

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

### Riverpod backend (`OxideBackend.riverpod`)

This backend generates a Riverpod provider that yields `OxideView<State, Actions>`.

Provider type depends on `@OxideStore(keepAlive: ...)`:

- `keepAlive: false` (default) → `NotifierProvider.autoDispose`
- `keepAlive: true` → `NotifierProvider`

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

### BLoC/Cubit backend (`OxideBackend.bloc`)

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

The recommended way to learn the exact UI wiring is to copy an example and adapt it.

## Persistence (Optional)

Persistence is feature-gated in Rust.

Enable it in your Rust dependency:

```toml
[dependencies]
oxide_core = { version = "0.1.0", features = ["state-persistence"] }
```

When working inside this repository, use a combined version + path dependency (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.1.0", path = "../rust/oxide_core", features = ["state-persistence"] }
```

Then use the persistence options supported by the reducer macro (see the `todos_app` example for a working configuration).
