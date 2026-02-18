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

  final List<KindT> _stack = <KindT>[];

  @override
  Future<Object?> push(RouteT route, {String? ticket}) async {
    final kind = _kindOf(route);
    _stack.add(kind);
    return _router.push<Object?>(_locationOf(route));
  }

  @override
  void pop([Object? result]) {
    if (_router.canPop()) {
      if (_stack.isNotEmpty) _stack.removeLast();
      _router.pop(result);
    }
  }

  @override
  void popUntil(KindT kind) {
    while (_stack.isNotEmpty && _stack.last != kind && _router.canPop()) {
      _stack.removeLast();
      _router.pop();
    }

    if (_stack.isEmpty || _stack.last != kind) {
      _stack
        ..clear()
        ..add(kind);
      _router.go(_locationOfKind(kind));
    }
  }

  @override
  void reset(List<RouteT> routes) {
    if (routes.isEmpty) return;
    final first = routes.first;
    final firstKind = _kindOf(first);
    _stack
      ..clear()
      ..add(firstKind);

    _router.go(_locationOf(first));

    for (final r in routes.skip(1)) {
      _stack.add(_kindOf(r));
      _router.push<void>(_locationOf(r));
    }
  }

  @override
  void setCurrentRoute(RouteT route) {
    final kind = _kindOf(route);
    if (_stack.isEmpty) {
      _stack.add(kind);
    } else {
      _stack[_stack.length - 1] = kind;
    }
  }
}

