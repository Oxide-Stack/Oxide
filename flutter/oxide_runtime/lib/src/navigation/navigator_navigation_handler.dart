import 'dart:async';

import 'package:flutter/widgets.dart';

import 'navigation_handler.dart';

/// Default Navigator 1.0 based navigation handler.
///
/// Why: most Flutter apps already depend on Navigator. This handler provides a minimal
/// implementation that works for apps that want an imperative route stack.
///
/// How: the handler uses a [GlobalKey] to access the active [NavigatorState], and a route
/// builder map keyed by a generated route kind enum.
final class NavigatorNavigationHandler<RouteT extends Object, KindT extends Object>
    implements OxideNavigationHandler<RouteT, KindT> {
  NavigatorNavigationHandler({
    required this.navigatorKey,
    required this.kindOf,
    required this.routeBuilders,
  });

  final GlobalKey<NavigatorState> navigatorKey;
  final KindT Function(RouteT route) kindOf;
  final Map<KindT, Widget Function(BuildContext context, RouteT route)> routeBuilders;

  @override
  Future<Object?> push(RouteT route, {String? ticket}) async {
    final navigator = navigatorKey.currentState;
    if (navigator == null) {
      throw StateError('Navigator is not ready. Ensure MaterialApp/WidgetsApp is built.');
    }

    final kind = kindOf(route);
    final builder = routeBuilders[kind];
    if (builder == null) {
      throw StateError('Missing route builder for kind $kind');
    }

    return navigator.push<Object?>(
      PageRouteBuilder<Object?>(
        settings: RouteSettings(name: kind.toString()),
        pageBuilder: (context, animation, secondaryAnimation) => builder(context, route),
      ),
    );
  }

  @override
  void pop([Object? result]) {
    final navigator = navigatorKey.currentState;
    if (navigator == null) return;
    navigator.pop<Object?>(result);
  }

  @override
  void popUntil(KindT kind) {
    final navigator = navigatorKey.currentState;
    if (navigator == null) return;
    final targetName = kind.toString();
    navigator.popUntil((route) => route.settings.name == targetName || route.isFirst);
  }

  @override
  void reset(List<RouteT> routes) {
    final navigator = navigatorKey.currentState;
    if (navigator == null) return;
    if (routes.isEmpty) {
      navigator.popUntil((r) => r.isFirst);
      return;
    }

    unawaited(_resetAsync(navigator, routes));
  }

  @override
  void setCurrentRoute(RouteT route) {}

  Future<void> _resetAsync(NavigatorState navigator, List<RouteT> routes) async {
    final first = routes.first;
    final firstKind = kindOf(first);
    final firstBuilder = routeBuilders[firstKind];
    if (firstBuilder == null) {
      throw StateError('Missing route builder for kind $firstKind');
    }

    await navigator.pushAndRemoveUntil<Object?>(
      PageRouteBuilder<Object?>(
        settings: RouteSettings(name: firstKind.toString()),
        pageBuilder: (context, animation, secondaryAnimation) => firstBuilder(context, first),
      ),
      (route) => false,
    );

    for (final r in routes.skip(1)) {
      await push(r);
    }
  }
}
