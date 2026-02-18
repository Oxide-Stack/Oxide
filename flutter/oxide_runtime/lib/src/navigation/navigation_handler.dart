/// Interface implemented by a Dart-side navigation backend.
///
/// Why: Oxide navigation is driven by Rust, but needs to execute real Flutter navigation
/// primitives. The handler isolates this Flutter-specific logic behind a small API.
abstract class OxideNavigationHandler<RouteT extends Object, KindT extends Object> {
  /// Pushes a route and optionally returns a result when the pushed page pops.
  Future<Object?> push(RouteT route, {String? ticket});

  /// Pops the current route.
  void pop([Object? result]);

  /// Pops routes until the first route with the given kind becomes active.
  void popUntil(KindT kind);

  /// Resets the stack to the given routes.
  void reset(List<RouteT> routes);

  /// Notifies the handler that the current route changed.
  void setCurrentRoute(RouteT route);
}

