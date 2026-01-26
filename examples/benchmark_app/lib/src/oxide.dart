import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'rust/api/counter_bridge.dart' as counter_api;
import 'rust/api/counter_bridge.dart' show ArcCounterEngine, CounterStateSnapshot;
import 'rust/api/json_bridge.dart' as json_api;
import 'rust/api/json_bridge.dart' show ArcJsonEngine, JsonStateSnapshot;
import 'rust/api/sieve_bridge.dart' as sieve_api;
import 'rust/api/sieve_bridge.dart' show ArcSieveEngine, SieveStateSnapshot;
import 'rust/state/counter_action.dart';
import 'rust/state/counter_state.dart';
import 'rust/state/json_action.dart';
import 'rust/state/json_state.dart';
import 'rust/state/sieve_action.dart';
import 'rust/state/sieve_state.dart';

part 'oxide.oxide.g.dart';

/// Oxide setup for this example:
/// - Declares the Rust engine + state/actions/snapshot types.
/// - Enables dev-only codegen (run build_runner) to generate:
///   - a ChangeNotifier controller
///   - an Actions facade
///   - a widget scope with a `useOxide(context)` accessor
@OxideStore(
  state: CounterState,
  snapshot: counter_api.CounterStateSnapshot,
  actions: CounterAction,
  engine: counter_api.ArcCounterEngine,
  backend: OxideBackend.riverpod,
  name: 'BenchCounterRiverpodOxide',
  createEngine: 'counter_api.createEngine',
  disposeEngine: 'counter_api.disposeEngine',
  dispatch: 'counter_api.dispatch',
  current: 'counter_api.current',
  stateStream: 'counter_api.stateStream',
)
class BenchCounterRiverpodOxide {}

@OxideStore(
  state: CounterState,
  snapshot: counter_api.CounterStateSnapshot,
  actions: CounterAction,
  engine: counter_api.ArcCounterEngine,
  backend: OxideBackend.bloc,
  name: 'BenchCounterBlocOxide',
  createEngine: 'counter_api.createEngine',
  disposeEngine: 'counter_api.disposeEngine',
  dispatch: 'counter_api.dispatch',
  current: 'counter_api.current',
  stateStream: 'counter_api.stateStream',
)
class BenchCounterBlocOxide {}

@OxideStore(
  state: CounterState,
  snapshot: counter_api.CounterStateSnapshot,
  actions: CounterAction,
  engine: counter_api.ArcCounterEngine,
  backend: OxideBackend.inheritedHooks,
  name: 'BenchCounterHooksOxide',
  createEngine: 'counter_api.createEngine',
  disposeEngine: 'counter_api.disposeEngine',
  dispatch: 'counter_api.dispatch',
  current: 'counter_api.current',
  stateStream: 'counter_api.stateStream',
)
class BenchCounterHooksOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.riverpod,
  name: 'BenchJsonRiverpodOxide',
  createEngine: 'json_api.createEngine',
  disposeEngine: 'json_api.disposeEngine',
  dispatch: 'json_api.dispatch',
  current: 'json_api.current',
  stateStream: 'json_api.stateStream',
)
class BenchJsonRiverpodOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.bloc,
  name: 'BenchJsonBlocOxide',
  createEngine: 'json_api.createEngine',
  disposeEngine: 'json_api.disposeEngine',
  dispatch: 'json_api.dispatch',
  current: 'json_api.current',
  stateStream: 'json_api.stateStream',
)
class BenchJsonBlocOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.inheritedHooks,
  name: 'BenchJsonHooksOxide',
  createEngine: 'json_api.createEngine',
  disposeEngine: 'json_api.disposeEngine',
  dispatch: 'json_api.dispatch',
  current: 'json_api.current',
  stateStream: 'json_api.stateStream',
)
class BenchJsonHooksOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.riverpod,
  name: 'BenchSieveRiverpodOxide',
  createEngine: 'sieve_api.createEngine',
  disposeEngine: 'sieve_api.disposeEngine',
  dispatch: 'sieve_api.dispatch',
  current: 'sieve_api.current',
  stateStream: 'sieve_api.stateStream',
)
class BenchSieveRiverpodOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.bloc,
  name: 'BenchSieveBlocOxide',
  createEngine: 'sieve_api.createEngine',
  disposeEngine: 'sieve_api.disposeEngine',
  dispatch: 'sieve_api.dispatch',
  current: 'sieve_api.current',
  stateStream: 'sieve_api.stateStream',
)
class BenchSieveBlocOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.inheritedHooks,
  name: 'BenchSieveHooksOxide',
  createEngine: 'sieve_api.createEngine',
  disposeEngine: 'sieve_api.disposeEngine',
  dispatch: 'sieve_api.dispatch',
  current: 'sieve_api.current',
  stateStream: 'sieve_api.stateStream',
)
class BenchSieveHooksOxide {}
