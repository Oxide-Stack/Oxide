import 'dart:async';

import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'rust/api/bridge.dart'
    show ArcAppEngine, AppStateSnapshot, createSharedEngine, current, dispatch, disposeEngine, stateStream;
import 'rust/state/app_action.dart';
import 'rust/state/app_state.dart';

part 'oxide.oxide.g.dart';

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.control],
  backend: OxideBackend.inherited,
  name: 'TickerControlInheritedOxide',
)
class TickerControlInheritedOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.tick],
  backend: OxideBackend.inherited,
  name: 'TickerTickInheritedOxide',
)
class TickerTickInheritedOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.control],
  backend: OxideBackend.inheritedHooks,
  name: 'TickerControlHooksOxide',
)
class TickerControlHooksOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.tick],
  backend: OxideBackend.inheritedHooks,
  name: 'TickerTickHooksOxide',
)
class TickerTickHooksOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.control],
  backend: OxideBackend.riverpod,
  name: 'TickerControlRiverpodOxide',
)
class TickerControlRiverpodOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.tick],
  backend: OxideBackend.riverpod,
  name: 'TickerTickRiverpodOxide',
)
class TickerTickRiverpodOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.control],
  backend: OxideBackend.bloc,
  name: 'TickerControlBlocOxide',
)
class TickerControlBlocOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.tick],
  backend: OxideBackend.bloc,
  name: 'TickerTickBlocOxide',
)
class TickerTickBlocOxide {}
