# Declare A Store (`@OxideStore`)

Create a small Dart file declaring the store types and the FRB function names, then run build_runner to generate `*.oxide.g.dart`.

Example `lib/src/oxide.dart` (modeled after [benchmark_app's oxide.dart](../../examples/benchmark_app/lib/src/oxide.dart)):

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

