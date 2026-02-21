import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/navigation/navigation_runtime.g.dart';
import '../../oxide_generated/routes/route_models.g.dart';
import 'bench_detail.dart';
import 'bench_screen.dart';
import 'routing_bench_screen.dart';

@OxideApp(navigation: OxideNavigation.navigator())
final class BenchApp extends StatelessWidget {
  const BenchApp({super.key});

  @override
  Widget build(BuildContext context) {
    final router = GoRouter(
      navigatorKey: oxideNavigatorKey,
      routes: <RouteBase>[
        GoRoute(path: '/', builder: (context, state) => BenchHomeScreen(route: const HomeRoute())),
        GoRoute(path: '/splash', builder: (context, state) => BenchSplashScreen(route: const SplashRoute())),
        GoRoute(path: '/charts', builder: (context, state) => BenchChartsScreen(route: const ChartsRoute())),
        GoRoute(path: '/routing', builder: (context, state) => const RoutingBenchScreen()),
        GoRoute(
          path: '/bench/:id',
          builder: (context, state) {
            final idStr = state.pathParameters['id'] ?? '0';
            final id = int.tryParse(idStr) ?? 0;
            return BenchDetailScreen(id: id);
          },
        ),
      ],
    );

    return MaterialApp.router(routerConfig: router);
  }
}
