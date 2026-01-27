import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'rust/api/bridge.dart' show ArcAppEngine, AppStateSnapshot, createEngine, current, dispatch, disposeEngine, stateStream;
import 'rust/state/app_action.dart';
import 'rust/state/app_state.dart';

part 'oxide.oxide.g.dart';

/// Oxide setup for this example:
/// - Declares the Rust engine + state/actions/snapshot types.
/// - Enables dev-only codegen (run build_runner) to generate:
///   - a ChangeNotifier controller
///   - an Actions facade
///   - a widget scope with a `useOxide(context)` accessor
@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.inherited,
  keepAlive: true,
  name: 'StateBridgeOxide',
)
class StateBridgeOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.inheritedHooks,
  keepAlive: true,
  name: 'StateBridgeHooksOxide',
)
class StateBridgeHooksOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.riverpod,
  keepAlive: true,
  name: 'StateBridgeRiverpodOxide',
)
class StateBridgeRiverpodOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  backend: OxideBackend.bloc,
  keepAlive: true,
  name: 'StateBridgeBlocOxide',
)
class StateBridgeBlocOxide {}
