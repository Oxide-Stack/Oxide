# Use the Generated Adapter in UI

Oxide supports multiple generated UI wiring styles, or backends. Choose one with `@OxideStore(backend: ...)`.

At app startup, initialize Oxide through the generated entrypoint:

> **Note:** if your Rust code defines isolated channels, the generated Dart entrypoints (`OxideStack.init` and `runOxideApp`) initialize the channel runtime for you. You do not need to call `initIsolatedChannels...` helpers manually.


```dart
import 'package:flutter/widgets.dart';

import 'package:your_app/oxide.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  runApp(const MyApp());
}
```

After codegen, all backends expose the same widget-facing shape: `OxideView<State, Actions>`, which contains:

- `state` (nullable until loaded)
- `actions` (typed async methods that dispatch actions into Rust)
- `isLoading` and `error` (engine lifecycle / reducer errors)

## InheritedWidget Backend (`OxideBackend.inherited`)

This backend generates:

- `<Name>Controller extends ChangeNotifier`
- `<Name>Scope extends StatefulWidget` (an `InheritedNotifier` provider)
- `<Name>Actions` facade

Wrap your app, or a subtree, with the generated scope:

```dart
import 'package:flutter/widgets.dart';

import 'package:your_app/oxide.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
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

## Inherited Hooks Backend (`OxideBackend.inheritedHooks`)

This backend extends the InheritedWidget backend with a generated hook function.

- Add dependency: `flutter_hooks`
- Wrap your widget tree with the generated scope, same as the inherited backend.
- Use the generated hook in a `HookWidget` (the exact function name depends on your store name).

```yaml
dependencies:
  flutter_hooks: ^0.21.0
```

```dart
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';

import 'package:your_app/src/oxide.dart';

class CounterHooksCard extends HookWidget {
  const CounterHooksCard({super.key});

  @override
  Widget build(BuildContext context) {
    final view = useAppOxide(context);

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

## Riverpod Backend (`OxideBackend.riverpod`)

This backend generates a Riverpod provider that yields `OxideView<State, Actions>`.

- Add dependencies: `flutter_riverpod`
- Read state via `ref.watch(...)`
- Dispatch via `ref.read(...).actions.<method>()`

## BLoC/Cubit Backend (`OxideBackend.bloc`)

This backend generates a `Cubit<OxideView<State, Actions>>` plus an actions facade.

- Add dependencies: `bloc`, `flutter_bloc`
- Provide the generated cubit with `BlocProvider`
- Build from emitted `OxideView` and dispatch via `cubit.actions`

Copy an example and adapt it.
