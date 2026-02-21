import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../oxide.dart';

final class TickerSplashScreen extends ConsumerWidget {
  const TickerSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(tickerControlRiverpodOxideProvider);
    return Scaffold(
      body: Center(
        child: Semantics(
          label: 'Loading',
          child: CircularProgressIndicator(),
        ),
      ),
    );
  }
}
