import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/routes/route_models.g.dart';
import '../features/home/todos_home_screen.dart';
import '../features/navigation/confirm_screen.dart';
import '../features/splash/todos_splash_screen.dart';

@OxideRoutePage('Splash')
final class TodosSplashPage extends ConsumerWidget {
  const TodosSplashPage({super.key, required this.route});

  final SplashRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const TodosSplashScreen();
  }
}

@OxideRoutePage('Home')
final class TodosHomePage extends StatelessWidget {
  const TodosHomePage({super.key, required this.route});

  final HomeRoute route;

  @override
  Widget build(BuildContext context) {
    return const TodosHomeScreen();
  }
}

@OxideRoutePage('Confirm')
final class TodosConfirmPage extends StatelessWidget {
  const TodosConfirmPage({super.key, required this.route});

  final ConfirmRoute route;

  @override
  Widget build(BuildContext context) {
    return TodosConfirmScreen(route: route);
  }
}
