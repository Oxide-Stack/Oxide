import 'dart:async';

import 'package:flutter/widgets.dart' show ValueNotifier;

import 'navigation_command.dart';
import 'navigation_error.dart';
import 'navigation_handler.dart';
import 'navigation_state.dart';

/// Runtime coordinator that executes Rust-emitted navigation commands.
///
/// Why: Rust is the source of truth for navigation intent. Dart must execute those intents
/// using Flutter-native primitives and then forward route context and results back to Rust.
///
/// How: the runtime listens to a command stream, delegates execution to the configured
/// [OxideNavigationHandler], and invokes callbacks for result/route-context forwarding.
final class OxideNavigationRuntime<RouteT extends Object, KindT extends Object> {
  OxideNavigationRuntime({
    required this.commands,
    required this.handler,
    required this.emitResult,
    required this.setCurrentRoute,
    required this.kindOf,
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
        _pushRoute(route);
        await _syncCurrentRoute();
        Object? result;
        try {
          result = await handler.push(route, ticket: ticket);
        } finally {
          _removeRoute(route);
          await _syncCurrentRoute();
        }
        if (ticket != null) await emitResult(ticket, result);
      case OxideNavigationPop<RouteT, KindT>(:final result):
        handler.pop(result);
        _popRoute();
        await _syncCurrentRoute();
      case OxideNavigationPopUntil<RouteT, KindT>(:final kind):
        handler.popUntil(kind);
        _popUntil(kind);
        await _syncCurrentRoute();
      case OxideNavigationReset<RouteT, KindT>(:final routes):
        handler.reset(routes);
        _reset(routes);
        await _syncCurrentRoute();
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
    while (_stack.length > 1 && kindOf(_stack.last) != kind) {
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

  Future<void> _syncCurrentRoute() async {
    final current = _stack.isEmpty ? null : _stack.last;
    if (current != null) {
      handler.setCurrentRoute(current);
    }
    await setCurrentRoute(current);
  }
}
