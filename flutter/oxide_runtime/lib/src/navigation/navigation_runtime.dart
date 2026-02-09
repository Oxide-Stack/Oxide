import 'dart:async';

import 'navigation_command.dart';
import 'navigation_handler.dart';

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
  });

  /// Stream of commands emitted by Rust.
  final Stream<OxideNavigationCommand<RouteT, KindT>> commands;

  /// Handler that executes navigation operations.
  final OxideNavigationHandler<RouteT, KindT> handler;

  /// Callback invoked when a pushed route completes and a ticket is present.
  final Future<void> Function(String ticket, Object? result) emitResult;

  /// Callback invoked to keep Rust route context in sync.
  final Future<void> Function(RouteT route) setCurrentRoute;

  StreamSubscription<OxideNavigationCommand<RouteT, KindT>>? _sub;

  /// Starts processing navigation commands.
  void start() {
    _sub ??= commands.listen(_handle);
  }

  /// Stops processing navigation commands.
  Future<void> stop() async {
    final sub = _sub;
    _sub = null;
    await sub?.cancel();
  }

  Future<void> _handle(OxideNavigationCommand<RouteT, KindT> cmd) async {
    switch (cmd) {
      case OxideNavigationPush<RouteT, KindT>(:final route, :final ticket):
        final result = await handler.push(route, ticket: ticket);
        if (ticket != null) {
          await emitResult(ticket, result);
        }
        handler.setCurrentRoute(route);
        await setCurrentRoute(route);
      case OxideNavigationPop<RouteT, KindT>(:final result):
        handler.pop(result);
      case OxideNavigationPopUntil<RouteT, KindT>(:final kind):
        handler.popUntil(kind);
      case OxideNavigationReset<RouteT, KindT>(:final routes):
        handler.reset(routes);
        if (routes.isNotEmpty) {
          final current = routes.last;
          handler.setCurrentRoute(current);
          await setCurrentRoute(current);
        }
    }
  }
}

