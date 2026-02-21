import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/routes/route_models.g.dart';
import '../app.dart';
import '../user_detail/user_detail_screen.dart';

@OxideRoutePage('Splash')
final class ApiBrowserSplashPage extends ConsumerWidget {
  const ApiBrowserSplashPage({super.key, required this.route});

  final SplashRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const ApiBrowserSplashScreen();
  }
}

@OxideRoutePage('Home')
final class ApiBrowserHomePage extends ConsumerWidget {
  const ApiBrowserHomePage({super.key, required this.route});

  final HomeRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const ApiBrowserHome();
  }
}

@OxideRoutePage('UserDetail')
final class ApiBrowserUserDetailPage extends StatelessWidget {
  const ApiBrowserUserDetailPage({super.key, required this.route});

  final UserDetailRoute route;

  @override
  Widget build(BuildContext context) {
    return ApiBrowserUserDetailScreen(route: route);
  }
}

