import 'dart:async';

import 'package:flutter/widgets.dart' show ValueNotifier;

import 'navigation_command.dart';
import 'navigation_error.dart';
import 'navigation_handler.dart';
import 'navigation_state.dart';

/// Runtime coordinator that executes Rust-emitted navigation commands.
///
/// Rust is the source of truth for navigation intent. Dart must execute those intents
/// using Flutter-native primitives and then forward route context and results back to Rust.
///
/// the runtime listens to a command stream, delegates execution to the configured
/// [OxideNavigationHandler], and invokes callbacks for result/route-context forwarding.
final class OxideNavigationRuntime<RouteT extends Object, KindT extends Object> {
  OxideNavigationRuntime({
    required this.commands,
    required this.handler,
    required this.emitResult,
    required this.setCurrentRoute,
    required this.kindOf,
    this.emitRouteUpdate,
    this.onCommandError,
    this.onStreamError,
  });

  /// Stream of commands emitted by Rust.
  final Stream<OxideNavigationCommand<RouteT, KindT>> commands;

  /// Handler that executes navigation operations.
  final OxideNavigationHandler<RouteT, KindT> handler;

  /// Callback invoked when a pushed route completes and a ticket is present.
  final Future<void> Function(String ticket, Object? result) emitResult;

  /// Callback invoked to keep Rust route context in sync.
  final Future<void> Function(RouteT? route) setCurrentRoute;

  /// Optional callback invoked with detailed route transition metadata.
  final Future<void> Function(OxideRouteUpdate<RouteT, KindT> update)? emitRouteUpdate;

  final KindT Function(RouteT route) kindOf;

  /// Callback invoked when executing a decoded navigation command fails.
  final void Function(Object error, StackTrace stackTrace, OxideNavigationCommand<RouteT, KindT> cmd)? onCommandError;

  /// Callback invoked when the command stream itself errors (decode/transport layer).
  final void Function(Object error, StackTrace stackTrace)? onStreamError;

  StreamSubscription<void>? _sub;
  final _errors = StreamController<OxideNavigationError<RouteT, KindT>>.broadcast();
  final _stack = <RouteT>[];
  late final ValueNotifier<OxideNavigationState<RouteT, KindT>> state = ValueNotifier(
    OxideNavigationState(stack: const [], current: null, kindOf: kindOf),
  );
  var _isDisposed = false;

  Stream<OxideNavigationError<RouteT, KindT>> get errors => _errors.stream;

  /// Starts processing navigation commands.
  void start() {
    if (_isDisposed) {
      throw StateError('Navigation runtime is disposed.');
    }
    _sub ??= commands
        .asyncMap(_handleSafely)
        .listen(
          (_) {},
          onError: (Object error, StackTrace stackTrace) {
            _errors.add(OxideNavigationStreamError(error, stackTrace));
            if (onStreamError != null) {
              onStreamError!(error, stackTrace);
              return;
            }
            Zone.current.handleUncaughtError(error, stackTrace);
          },
        );
  }

  /// Stops processing navigation commands.
  Future<void> stop() async {
    final sub = _sub;
    _sub = null;
    await sub?.cancel();
    _stack.clear();
    _updateState();
    await setCurrentRoute(null);
  }

  Future<void> dispose() async {
    if (_isDisposed) return;
    _isDisposed = true;
    await stop();
    await _errors.close();
    state.dispose();
  }

  Future<void> _handleSafely(OxideNavigationCommand<RouteT, KindT> cmd) async {
    try {
      await _handle(cmd);
    } catch (error, stackTrace) {
      _errors.add(OxideNavigationCommandError(error, stackTrace, cmd));
      if (onCommandError != null) {
        onCommandError!(error, stackTrace, cmd);
        return;
      }
      Zone.current.handleUncaughtError(error, stackTrace);
    }
  }

  Future<void> _handle(OxideNavigationCommand<RouteT, KindT> cmd) async {
    switch (cmd) {
      case OxideNavigationPush<RouteT, KindT>(:final route, :final ticket):
        final wasEmpty = _stack.isEmpty;
        // ignore duplicate push if the top of our stack already matches the
        // requested route. This prevents spurious startup duplicate pushes and
        // other coalesces.
        if (!(_stack.isNotEmpty && _stack.last == route)) {
          _pushRoute(route);
          await _syncCurrentRoute();
          await _emitRouteUpdate(
            OxideRouteUpdate<RouteT, KindT>(
              operation: OxideRouteOperation.push,
              route: route,
              arguments: route,
              firstPush: wasEmpty,
            ),
          );
          unawaited(_completePush(route, ticket, cmd));
        }
      case OxideNavigationPop<RouteT, KindT>(:final result):
        final popped = _stack.isNotEmpty ? _stack.last : null;
        handler.pop(result);
        _popRoute();
        await _syncCurrentRoute();
        await _emitRouteUpdate(
          OxideRouteUpdate<RouteT, KindT>(
            operation: OxideRouteOperation.pop,
            route: popped,
            result: result,
          ),
        );
      case OxideNavigationPopUntil<RouteT, KindT>(:final kind):
        handler.popUntil(kind);
        _popUntil(kind);
        await _syncCurrentRoute();
        await _emitRouteUpdate(
          OxideRouteUpdate<RouteT, KindT>(
            operation: OxideRouteOperation.popUntil,
            route: _stack.isEmpty ? null : _stack.last,
          ),
        );
      case OxideNavigationReset<RouteT, KindT>(:final routes):
        // ignore redundant resets to avoid unnecessary navigator churn and
        // flicker. equality is based on route sequence, preserving order.
        if (!_routesEqual(_stack, routes)) {
          await handler.reset(routes);
          _reset(routes);
          await _syncCurrentRoute();
          await _emitRouteUpdate(
            OxideRouteUpdate<RouteT, KindT>(
              operation: OxideRouteOperation.reset,
              route: _stack.isEmpty ? null : _stack.last,
              arguments: routes,
            ),
          );
        }
    }
  }

  void _pushRoute(RouteT route) {
    _stack.add(route);
    _updateState();
  }

  void _popRoute() {
    if (_stack.isNotEmpty) {
      _stack.removeLast();
      _updateState();
    }
  }

  void _removeRoute(RouteT route) {
    final idx = _stack.lastIndexOf(route);
    if (idx != -1) {
      _stack.removeAt(idx);
      _updateState();
    }
  }

  void _popUntil(KindT kind) {
    final targetIndex = _stack.lastIndexWhere((route) => kindOf(route) == kind);
    if (targetIndex == -1) return;
    while (_stack.length - 1 > targetIndex) {
      _stack.removeLast();
    }
    _updateState();
  }

  void _reset(List<RouteT> routes) {
    _stack
      ..clear()
      ..addAll(routes);
    _updateState();
  }

  void _updateState() {
    final current = _stack.isEmpty ? null : _stack.last;
    state.value = OxideNavigationState(stack: List.unmodifiable(_stack), current: current, kindOf: kindOf);
  }

  bool _routesEqual(List<RouteT> a, List<RouteT> b) {
    if (a.length != b.length) return false;
    for (var i = 0; i < a.length; i++) {
      if (a[i] != b[i]) return false;
    }
    return true;
  }

  Future<void> _completePush(RouteT route, String? ticket, OxideNavigationCommand<RouteT, KindT> cmd) async {
    Object? result;
    try {
      result = await handler.push(route, ticket: ticket);
    } catch (error, stackTrace) {
      _errors.add(OxideNavigationCommandError(error, stackTrace, cmd));
      if (onCommandError != null) {
        onCommandError!(error, stackTrace, cmd);
        return;
      }
      Zone.current.handleUncaughtError(error, stackTrace);
      return;
    } finally {
      _removeRoute(route);
      await _syncCurrentRoute();
    }
    if (ticket != null) {
      await emitResult(ticket, result);
    }
  }

  Future<void> _syncCurrentRoute() async {
    final current = _stack.isEmpty ? null : _stack.last;
    if (current != null) {
      handler.setCurrentRoute(current);
    }
    await setCurrentRoute(current);
  }

  Future<void> _emitRouteUpdate(OxideRouteUpdate<RouteT, KindT> update) async {
    final callback = emitRouteUpdate;
    if (callback == null) return;
    await callback(update);
  }
}
