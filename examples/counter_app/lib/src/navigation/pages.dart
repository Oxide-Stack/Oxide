import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/routes/route_kind.g.dart';
import '../../oxide_generated/routes/route_models.g.dart';
import '../features/counter/counter_home_screen.dart';
import '../features/navigation/confirm_screen.dart';
import '../features/navigation/counter_detail_screen.dart';
import '../features/splash/counter_splash_screen.dart';

@OxideRoutePage(RouteKind.splash)
final class CounterSplashPage extends ConsumerWidget {
  const CounterSplashPage({super.key, required this.route});

  final SplashRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const CounterSplashScreen();
  }
}

@OxideRoutePage(RouteKind.home)
final class CounterHomePage extends StatelessWidget {
  const CounterHomePage({super.key, required this.route});

  final HomeRoute route;

  @override
  Widget build(BuildContext context) {
    return const CounterHomeScreen();
  }
}

@OxideRoutePage(RouteKind.confirm)
final class CounterConfirmPage extends StatelessWidget {
  const CounterConfirmPage({super.key, required this.route});

  final ConfirmRoute route;

  @override
  Widget build(BuildContext context) {
    return CounterConfirmScreen(route: route);
  }
}

@OxideRoutePage(RouteKind.counterDetail)
final class CounterDetailPage extends ConsumerWidget {
  const CounterDetailPage({super.key, required this.route});

  final CounterDetailRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return CounterDetailScreen(route: route);
  }
}
