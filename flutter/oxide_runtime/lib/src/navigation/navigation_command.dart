/// A navigation command consumed by an [OxideNavigationHandler].
///
/// Why: Oxide navigation is Rust-driven. Rust emits commands, and Dart executes them using
/// a Flutter-native handler (Navigator 1.0, GoRouter, or custom).
///
/// How: Apps typically decode incoming commands from the Rust binding layer into this model.
sealed class OxideNavigationCommand<RouteT extends Object, KindT extends Object> {
  const OxideNavigationCommand();

  /// Pushes a new route.
  const factory OxideNavigationCommand.push({
    required RouteT route,
    String? ticket,
  }) = OxideNavigationPush<RouteT, KindT>;

  /// Pops the current route.
  const factory OxideNavigationCommand.pop({Object? result}) = OxideNavigationPop<RouteT, KindT>;

  /// Pops until the given kind becomes active.
  const factory OxideNavigationCommand.popUntil({required KindT kind}) = OxideNavigationPopUntil<RouteT, KindT>;

  /// Resets the stack to the given routes.
  const factory OxideNavigationCommand.reset({required List<RouteT> routes}) = OxideNavigationReset<RouteT, KindT>;
}

/// Push command.
final class OxideNavigationPush<RouteT extends Object, KindT extends Object> extends OxideNavigationCommand<RouteT, KindT> {
  const OxideNavigationPush({required this.route, this.ticket});

  final RouteT route;
  final String? ticket;
}

/// Pop command.
final class OxideNavigationPop<RouteT extends Object, KindT extends Object> extends OxideNavigationCommand<RouteT, KindT> {
  const OxideNavigationPop({this.result});

  final Object? result;
}

/// PopUntil command.
final class OxideNavigationPopUntil<RouteT extends Object, KindT extends Object> extends OxideNavigationCommand<RouteT, KindT> {
  const OxideNavigationPopUntil({required this.kind});

  final KindT kind;
}

/// Reset command.
final class OxideNavigationReset<RouteT extends Object, KindT extends Object> extends OxideNavigationCommand<RouteT, KindT> {
  const OxideNavigationReset({required this.routes});

  final List<RouteT> routes;
}
