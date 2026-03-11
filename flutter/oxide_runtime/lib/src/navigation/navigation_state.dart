final class OxideNavigationState<RouteT extends Object, KindT extends Object> {
  const OxideNavigationState({
    required this.stack,
    required this.current,
    required this.kindOf,
  });

  final List<RouteT> stack;
  final RouteT? current;
  final KindT Function(RouteT route) kindOf;

  List<KindT> get kindStack => stack.map(kindOf).toList(growable: false);
  KindT? get currentKind => current == null ? null : kindOf(current as RouteT);
}
