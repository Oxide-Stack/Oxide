{{page_imports}}import 'package:flutter/material.dart';

import 'oxide.dart';

{{page_body}}

class _CounterView extends StatelessWidget {
  const _CounterView({
    required this.counter,
    required this.onIncrement,
    required this.onDecrement,
    required this.onReset,
  });

  final int counter;
  final Future<void> Function() onIncrement;
  final Future<void> Function() onDecrement;
  final Future<void> Function() onReset;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Oxide Counter')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text('You have pushed the button this many times:'),
            Text(
              'Counter: $counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            const SizedBox(height: 16),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                FilledButton(onPressed: () => onDecrement(), child: const Text('Decrement')),
                const SizedBox(width: 8),
                FilledButton(onPressed: () => onIncrement(), child: const Text('Increment')),
                const SizedBox(width: 8),
                FilledButton(onPressed: () => onReset(), child: const Text('Reset')),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
