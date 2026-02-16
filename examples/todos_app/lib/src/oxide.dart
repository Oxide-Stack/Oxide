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
  slices: [AppStateSlice.todos],
  backend: OxideBackend.inherited,
  name: 'TodosListInheritedOxide',
)
class TodosListInheritedOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.nextId],
  backend: OxideBackend.inherited,
  name: 'TodosNextIdInheritedOxide',
)
class TodosNextIdInheritedOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.todos],
  backend: OxideBackend.inheritedHooks,
  name: 'TodosListHooksOxide',
)
class TodosListHooksOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.nextId],
  backend: OxideBackend.inheritedHooks,
  name: 'TodosNextIdHooksOxide',
)
class TodosNextIdHooksOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.todos],
  backend: OxideBackend.riverpod,
  name: 'TodosListRiverpodOxide',
)
class TodosListRiverpodOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.nextId],
  backend: OxideBackend.riverpod,
  name: 'TodosNextIdRiverpodOxide',
)
class TodosNextIdRiverpodOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.todos],
  backend: OxideBackend.bloc,
  name: 'TodosListBlocOxide',
)
class TodosListBlocOxide {}

@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  createEngine: 'createSharedEngine',
  slices: [AppStateSlice.nextId],
  backend: OxideBackend.bloc,
  name: 'TodosNextIdBlocOxide',
)
class TodosNextIdBlocOxide {}
