import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide_generated/navigation/navigation_runtime.g.dart';
import '../../oxide_generated/routes/route_models.g.dart';
import '../navigation/pages.dart';

@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: oxideNavigatorKey,
      theme: ThemeData(colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF3F51B5)), useMaterial3: true),
      home: CounterSplashPage(route: const SplashRoute()),
    );
  }
}

