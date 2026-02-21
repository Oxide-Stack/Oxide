import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../oxide.dart';

final class TodosSplashScreen extends ConsumerWidget {
  const TodosSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(todosListRiverpodOxideProvider);
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
