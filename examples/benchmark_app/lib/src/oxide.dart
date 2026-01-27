import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
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
  bindings: 'counter_api',
)
class BenchCounterRiverpodOxide {}

@OxideStore(
  state: CounterState,
  snapshot: counter_api.CounterStateSnapshot,
  actions: CounterAction,
  engine: counter_api.ArcCounterEngine,
  backend: OxideBackend.bloc,
  bindings: 'counter_api',
)
class BenchCounterBlocOxide {}

@OxideStore(
  state: CounterState,
  snapshot: counter_api.CounterStateSnapshot,
  actions: CounterAction,
  engine: counter_api.ArcCounterEngine,
  backend: OxideBackend.inheritedHooks,
  bindings: 'counter_api',
)
class BenchCounterHooksOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.riverpod,
  bindings: 'json_api',
)
class BenchJsonRiverpodOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.bloc,
  bindings: 'json_api',
)
class BenchJsonBlocOxide {}

@OxideStore(
  state: JsonState,
  snapshot: json_api.JsonStateSnapshot,
  actions: JsonAction,
  engine: json_api.ArcJsonEngine,
  backend: OxideBackend.inheritedHooks,
  bindings: 'json_api',
)
class BenchJsonHooksOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.riverpod,
  bindings: 'sieve_api',
)
class BenchSieveRiverpodOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.bloc,
  bindings: 'sieve_api',
)
class BenchSieveBlocOxide {}

@OxideStore(
  state: SieveState,
  snapshot: sieve_api.SieveStateSnapshot,
  actions: SieveAction,
  engine: sieve_api.ArcSieveEngine,
  backend: OxideBackend.inheritedHooks,
  bindings: 'sieve_api',
)
class BenchSieveHooksOxide {}
