import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../oxide_generated/routes/route_kind.g.dart';
import 'home.dart';
import 'isolated_channels_demo_page.dart';
import 'oxide.dart';

@OxideRoutePage(RouteKind.splash)
final class ApiBrowserSplashScreen extends ConsumerWidget {
  const ApiBrowserSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(usersRiverpodOxideProvider);
    return const Scaffold(body: Center(child: Text('Loading…')));
  }
}

@OxideRoutePage(RouteKind.home)
class ApiBrowserHome extends ConsumerWidget {
  const ApiBrowserHome({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Oxide API Browser'),
        actions: [
          IconButton(
            onPressed: () {
              Navigator.of(context).push(
                MaterialPageRoute(builder: (_) => const ApiBrowserIsolatedChannelsDemoPage()),
              );
            },
            icon: const Icon(Icons.swap_horiz),
            tooltip: 'Isolated Channels Demo',
          ),
        ],
      ),
      body: const ApiBrowserCoordinator(child: ApiBrowserView()),
    );
  }
}
