import 'navigation_command.dart';

sealed class OxideNavigationError<RouteT extends Object, KindT extends Object> {
  const OxideNavigationError();
}

final class OxideNavigationStreamError<RouteT extends Object, KindT extends Object>
    extends OxideNavigationError<RouteT, KindT> {
  const OxideNavigationStreamError(this.error, this.stackTrace);

  final Object error;
  final StackTrace stackTrace;
}

final class OxideNavigationCommandError<RouteT extends Object, KindT extends Object>
    extends OxideNavigationError<RouteT, KindT> {
  const OxideNavigationCommandError(this.error, this.stackTrace, this.command);

  final Object error;
  final StackTrace stackTrace;
  final OxideNavigationCommand<RouteT, KindT> command;
}
