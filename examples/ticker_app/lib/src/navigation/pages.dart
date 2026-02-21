import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/routes/route_models.g.dart';
import '../features/home/ticker_home_screen.dart';
import '../features/navigation/confirm_screen.dart';
import '../features/splash/ticker_splash_screen.dart';

@OxideRoutePage('Splash')
final class TickerSplashPage extends ConsumerWidget {
  const TickerSplashPage({super.key, required this.route});

  final SplashRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const TickerSplashScreen();
  }
}

@OxideRoutePage('Home')
final class TickerHomePage extends StatelessWidget {
  const TickerHomePage({super.key, required this.route});

  final HomeRoute route;

  @override
  Widget build(BuildContext context) {
    return const TickerHomeScreen();
  }
}

@OxideRoutePage('Confirm')
final class TickerConfirmPage extends StatelessWidget {
  const TickerConfirmPage({super.key, required this.route});

  final ConfirmRoute route;

  @override
  Widget build(BuildContext context) {
    return TickerConfirmScreen(route: route);
  }
}
