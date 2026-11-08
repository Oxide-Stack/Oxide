import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../oxide.dart';
import 'counter_page.dart';
import 'navigation/pages.dart';

@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      // Required by the generated navigation handler. Without it,
      // `initNavigation` waits 10s for a NavigatorState that is never
      // attached and then throws.
      navigatorKey: OxideStack.navigatorKey,
      title: '{{app_name}}',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF3F51B5)),
        useMaterial3: true,
      ),
      home: const HomePage(route: HomeRoute()),
    );
  }
}
