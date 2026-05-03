import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';
import 'package:showcase_app/presintation/widgets/isolated_channels_demo_pane.dart';

class ChannelsScreen extends ConsumerWidget {
  const ChannelsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final actions = ref.watch(settingsControllerProvider).actions;
    return Scaffold(
      appBar: AppBar(
        title: const Text('Isolated Channels Demo'),
        leading: IconButton(
          onPressed: () => actions.pop(),
          icon: const Icon(Icons.arrow_back),
        ),
      ),
      body: const Padding(
        padding: EdgeInsets.all(16),
        child: IsolatedChannelsDemoPane(),
      ),
    );
  }
}
