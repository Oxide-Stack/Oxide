import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'home.dart';

class ApiBrowserApp extends StatelessWidget {
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

class ApiBrowserHome extends ConsumerWidget {
  const ApiBrowserHome({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return const Scaffold(body: ApiBrowserCoordinator(child: ApiBrowserView()));
  }
}
