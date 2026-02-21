import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../oxide_generated/navigation/navigation_runtime.g.dart';
import '../oxide_generated/routes/route_models.g.dart';
import 'navigation/pages.dart';

@OxideApp(navigation: OxideNavigation.navigator())
final class ApiBrowserApp extends StatelessWidget {
  const ApiBrowserApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Oxide API Browser',
      theme: ThemeData(colorSchemeSeed: Colors.indigo, useMaterial3: true),
      navigatorKey: oxideNavigatorKey,
      home: ApiBrowserSplashPage(route: const SplashRoute()),
    );
  }
}
