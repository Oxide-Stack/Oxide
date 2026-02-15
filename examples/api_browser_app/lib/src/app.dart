import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../oxide_generated/routes/route_kind.g.dart';
import 'home.dart';
import 'oxide.dart';

@OxideRoutePage(RouteKind.splash)
final class ApiBrowserSplashScreen extends ConsumerWidget {
  const ApiBrowserSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(usersRiverpodOxideProvider);
    return const Scaffold(body: Center(child: Text('Loading…')));
  }
}

@OxideRoutePage(RouteKind.home)
class ApiBrowserHome extends ConsumerWidget {
  const ApiBrowserHome({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const Scaffold(body: ApiBrowserCoordinator(child: ApiBrowserView()));
  }
}
