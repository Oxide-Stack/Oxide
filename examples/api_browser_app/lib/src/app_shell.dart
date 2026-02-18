import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'app.dart';

@OxideApp(navigation: OxideNavigation.navigator())
final class ApiBrowserApp extends StatelessWidget {
  const ApiBrowserApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Oxide API Browser',
      theme: ThemeData(colorSchemeSeed: Colors.indigo, useMaterial3: true),
      home: const ApiBrowserHome(),
    );
  }
}
