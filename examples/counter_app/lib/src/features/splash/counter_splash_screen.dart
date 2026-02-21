import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../oxide.dart';

final class CounterSplashScreen extends ConsumerWidget {
  const CounterSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(stateBridgeRiverpodOxideProvider);
    return Scaffold(
      body: Center(
        child: Semantics(
          label: 'Loading',
          child: const CircularProgressIndicator(),
        ),
      ),
    );
  }
}
