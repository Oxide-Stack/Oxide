import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../oxide_generated/routes/route_models.g.dart';
import '../../oxide.dart';

final class CounterDetailScreen extends ConsumerWidget {
  const CounterDetailScreen({super.key, required this.route});

  final CounterDetailRoute route;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(stateBridgeRiverpodOxideProvider);
    final state = view.state;
    final actions = ref.read(stateBridgeRiverpodOxideProvider).actions;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Counter Detail'),
        leading: IconButton(
          onPressed: () => Navigator.of(context).maybePop(),
          icon: const Icon(Icons.arrow_back),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Text('Route arg start=${route.start}', style: Theme.of(context).textTheme.titleLarge),
          const SizedBox(height: 12),
          Text('Current state counter=${state?.counter ?? 'null'}'),
          const SizedBox(height: 24),
          FilledButton(
            onPressed: () => actions.pop(),
            child: const Text('Pop (Rust-driven)'),
          ),
          const SizedBox(height: 8),
          OutlinedButton(
            onPressed: () => actions.popUntilHome(),
            child: const Text('PopUntil Home (Rust-driven)'),
          ),
          const SizedBox(height: 8),
          TextButton(
            onPressed: () => actions.resetStack(),
            child: const Text('Reset stack (Rust-driven)'),
          ),
        ],
      ),
    );
  }
}

