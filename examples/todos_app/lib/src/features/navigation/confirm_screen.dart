import 'package:flutter/material.dart';

import '../../../oxide_generated/routes/route_models.g.dart';

final class TodosConfirmScreen extends StatelessWidget {
  const TodosConfirmScreen({super.key, required this.route});

  final ConfirmRoute route;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Confirm')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(route.title.toString(), style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 16),
            const Text('Return a bool result back to Rust via navigation ticket.'),
            const Spacer(),
            Row(
              children: [
                Expanded(
                  child: OutlinedButton(
                    onPressed: () => Navigator.of(context).pop(false),
                    child: const Text('Cancel'),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: FilledButton(
                    onPressed: () => Navigator.of(context).pop(true),
                    child: const Text('Confirm'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
