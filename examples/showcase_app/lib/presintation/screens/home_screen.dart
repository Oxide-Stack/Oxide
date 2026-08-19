import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/presintation/color_utils.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';
import 'package:showcase_app/src/rust/config/enums/theme_type.dart'
    as rust_theme;

class HomeScreen extends ConsumerWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsControllerProvider);
    final themeSlice = ref.watch(settingsThemeSliceControllerProvider);
    final colorSlice = ref.watch(settingsColorSliceControllerProvider);
    final state = settings.state;
    final currentTheme = state?.theme ?? rust_theme.Theme.light;
    final currentColor =
        parseHexColor(state?.mainColor) ??
        Theme.of(context).colorScheme.primary;

    return Scaffold(
      appBar: AppBar(title: const Text('Oxide Showcase')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Text(
            'Welcome',
            style: Theme.of(context).textTheme.headlineMedium,
          ),
          const SizedBox(height: 12),
          const Text(
            'This screen is route-driven by Oxide navigation. Open settings to '
            'dispatch reducer actions and watch Rust-owned state update the app theme.',
          ),
          const SizedBox(height: 24),
          Card(
            child: ListTile(
              leading: CircleAvatar(backgroundColor: currentColor),
              title: Text(
                currentTheme == rust_theme.Theme.dark
                    ? 'Theme: Dark'
                    : 'Theme: Light',
              ),
              subtitle: Text(
                state?.mainColor.isNotEmpty == true
                    ? 'Main color: ${state!.mainColor}'
                    : 'Main color: Unset',
              ),
            ),
          ),
          const SizedBox(height: 12),
          Card(
            child: ListTile(
              title: const Text('Theme slice listener'),
              subtitle: Text(
                themeSlice.state?.theme == rust_theme.Theme.dark
                    ? 'Observed theme slice: Dark'
                    : 'Observed theme slice: Light',
              ),
            ),
          ),
          const SizedBox(height: 12),
          Card(
            child: ListTile(
              title: const Text('Color slice listener'),
              subtitle: Text(
                colorSlice.state?.mainColor.isNotEmpty == true
                    ? 'Observed color slice: ${colorSlice.state!.mainColor}'
                    : 'Observed color slice: Unset',
              ),
            ),
          ),
          const SizedBox(height: 16),
          FilledButton.icon(
            onPressed: () => settings.actions.openSettings(),
            icon: const Icon(Icons.settings),
            label: const Text('Open settings (Rust-driven)'),
          ),
          const SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () => settings.actions.openChannels(),
            icon: const Icon(Icons.sync_alt),
            label: const Text('Open isolated channels demo'),
          ),
          const SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () => settings.actions.openBackendMatrix(),
            icon: const Icon(Icons.dashboard_customize),
            label: const Text('Open backend matrix demo'),
          ),
        ],
      ),
    );
  }
}
