import 'dart:async';

import 'package:go_router/go_router.dart';

import 'navigation_handler.dart';

final class GoRouterNavigationHandler<RouteT extends Object, KindT extends Object>
    implements OxideNavigationHandler<RouteT, KindT> {
  GoRouterNavigationHandler({
    required GoRouter router,
    required KindT Function(RouteT route) kindOf,
    required String Function(RouteT route) locationOf,
    required String Function(KindT kind) locationOfKind,
  })  : _router = router,
        _kindOf = kindOf,
        _locationOf = locationOf,
        _locationOfKind = locationOfKind;

  final GoRouter _router;
  final KindT Function(RouteT route) _kindOf;
  final String Function(RouteT route) _locationOf;
  final String Function(KindT kind) _locationOfKind;
  String get _currentLocation => _router.routerDelegate.currentConfiguration.uri.toString();

  @override
  Future<Object?> push(RouteT route, {String? ticket}) async {
    return _router.push<Object?>(_locationOf(route));
  }

  @override
  void pop([Object? result]) {
    if (_router.canPop()) {
      _router.pop(result);
    }
  }

  @override
  void popUntil(KindT kind) {
    final target = _locationOfKind(kind);
    while (_router.canPop() && _currentLocation != target) {
      _router.pop();
    }

    if (_currentLocation != target) {
      _router.go(target);
    }
  }

  @override
  void reset(List<RouteT> routes) {
    if (routes.isEmpty) return;
    unawaited(() async {
      _router.go(_locationOf(routes.first));
      for (final r in routes.skip(1)) {
        await _router.push<void>(_locationOf(r));
      }
    }());
  }

  @override
  void setCurrentRoute(RouteT route) {
    final _ = _kindOf(route);
  }
}
