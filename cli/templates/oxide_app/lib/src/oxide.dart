import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
{{oxide_extra_imports}}
import 'rust/api/bridge.dart'
    show ArcAppEngine, AppStateSnapshot, createEngine, current, dispatch, disposeEngine, stateStream;

import 'rust/state/app_action.dart';
import 'rust/state/app_state.dart';

part 'oxide.oxide.g.dart';

/// Oxide setup for this app (backend: {{backend}}):
/// - Declares the Rust engine + state/actions/snapshot types.
/// - Enables dev-only codegen (run build_runner) to generate:
///   - a ChangeNotifier controller
///   - an Actions facade
///   - a widget scope with generated accessors
@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: {{oxide_backend}},
  keepAlive: true,
  name: 'AppStore',
)
class AppStore {}
