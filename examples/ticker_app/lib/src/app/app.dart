import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide.dart';
import '../navigation/pages.dart';

@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      navigatorKey: OxideStack.navigatorKey,
      theme: ThemeData(colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF512DA8)), useMaterial3: true),
      home: TickerSplashPage(route: const SplashRoute()),
    );
  }
}
