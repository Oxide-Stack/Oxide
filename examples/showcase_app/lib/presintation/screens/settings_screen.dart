import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/presintation/color_utils.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';
import 'package:showcase_app/src/rust/config/enums/theme_type.dart'
    as rust_theme;

class SettingsScreen extends HookConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsControllerProvider);
    final state = settings.state;
    final colorController = useTextEditingController(
      text: state?.mainColor ?? '',
    );
    final imagePathController = useTextEditingController();

    useEffect(() {
      final value = state?.mainColor ?? '';
      if (colorController.text != value) {
        colorController.text = value;
        colorController.selection = TextSelection.collapsed(
          offset: value.length,
        );
      }
      return null;
    }, [state?.mainColor]);

    final currentTheme = state?.theme ?? rust_theme.Theme.light;
    final currentColor =
        parseHexColor(state?.mainColor) ??
        Theme.of(context).colorScheme.primary;
    final colorLabel = state != null && state.mainColor.isNotEmpty
        ? state.mainColor
        : 'Unset';

    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
        leading: IconButton(
          onPressed: () => settings.actions.pop(),
          icon: const Icon(Icons.arrow_back),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          DropdownButtonFormField<rust_theme.Theme>(
            key: ValueKey(currentTheme),
            initialValue: currentTheme,
            decoration: const InputDecoration(
              labelText: 'Theme',
              border: OutlineInputBorder(),
            ),
            items: rust_theme.Theme.values
                .map(
                  (theme) => DropdownMenuItem<rust_theme.Theme>(
                    value: theme,
                    child: Text(_themeLabel(theme)),
                  ),
                )
                .toList(),
            onChanged: (value) {
              if (value != null) {
                settings.actions.setTheme(value);
              }
            },
          ),
          const SizedBox(height: 24),
          Text('Main color', style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 12),
          TextField(
            controller: colorController,
            decoration: const InputDecoration(
              labelText: 'Hex color',
              border: OutlineInputBorder(),
              hintText: '#336699',
            ),
            onSubmitted: (value) =>
                _applyMainColor(context, settings.actions, value),
          ),
          const SizedBox(height: 12),
          FilledButton(
            onPressed: () =>
                _applyMainColor(context, settings.actions, colorController.text),
            child: const Text('Apply color'),
          ),
          const SizedBox(height: 16),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children:
                <Color>[
                  Colors.red,
                  Colors.pink,
                  Colors.purple,
                  Colors.indigo,
                  Colors.blue,
                  Colors.teal,
                  Colors.green,
                  Colors.orange,
                  Colors.amber,
                  Colors.brown,
                  Colors.grey,
                  Colors.black,
                ].map((color) {
                  final selected = currentColor.toARGB32() == color.toARGB32();
                  return GestureDetector(
                    onTap: () => settings.actions.setMainColor(colorToHex(color)),
                    child: Container(
                      width: 44,
                      height: 44,
                      decoration: BoxDecoration(
                        color: color,
                        shape: BoxShape.circle,
                        border: Border.all(
                          color: selected
                              ? Theme.of(context).colorScheme.onSurface
                              : Colors.transparent,
                          width: 3,
                        ),
                      ),
                      child: selected
                          ? const Icon(
                              Icons.check,
                              color: Colors.white,
                              size: 20,
                            )
                          : null,
                    ),
                  );
                }).toList(),
          ),
          const SizedBox(height: 24),
          Text(
            'Extract color from an image',
            style: Theme.of(context).textTheme.titleMedium,
          ),
          const SizedBox(height: 12),
          TextField(
            controller: imagePathController,
            decoration: const InputDecoration(
              labelText: 'Image path',
              border: OutlineInputBorder(),
              hintText: r'C:\images\showcase.png',
            ),
          ),
          const SizedBox(height: 12),
          FilledButton.tonal(
            onPressed: () => _extractMainColor(
              context,
              settings.actions,
              imagePathController.text,
            ),
            child: const Text('Extract main color'),
          ),
          const SizedBox(height: 24),
          Text(
            'Persistence demo',
            style: Theme.of(context).textTheme.titleMedium,
          ),
          const SizedBox(height: 12),
          const Text(
            'Changes are persisted by Rust. Press reload to recreate the engine '
            'and restore the latest snapshot from disk.',
          ),
          const SizedBox(height: 12),
          FilledButton.tonalIcon(
            onPressed: () => settings.actions.resetState(),
            icon: const Icon(Icons.restart_alt),
            label: const Text('Reset state'),
          ),
          const SizedBox(height: 8),
          OutlinedButton.icon(
            onPressed: () {
              ref.invalidate(settingsControllerProvider);
              ref.invalidate(settingsThemeSliceControllerProvider);
              ref.invalidate(settingsColorSliceControllerProvider);
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text(
                    'Engine reloaded. Restored state is read from persistence.',
                  ),
                ),
              );
            },
            icon: const Icon(Icons.refresh),
            label: const Text('Reload from persisted snapshot'),
          ),
          const SizedBox(height: 24),
          Card(
            child: ListTile(
              leading: CircleAvatar(backgroundColor: currentColor),
              title: const Text('Preview'),
              subtitle: Text(
                'Theme: ${_themeLabel(currentTheme)} • Color: $colorLabel',
              ),
            ),
          ),
        ],
      ),
    );
  }
}

void _applyMainColor(
  BuildContext context,
  SettingsControllerActions actions,
  String value,
) {
  final parsed = parseHexColor(value);
  if (parsed == null) {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Enter a valid hex color, like #336699')),
    );
    return;
  }

  actions.setMainColor(colorToHex(parsed));
}

void _extractMainColor(
  BuildContext context,
  SettingsControllerActions actions,
  String value,
) {
  final path = value.trim();
  if (path.isEmpty) {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Enter an image path to extract from')),
    );
    return;
  }

  actions.extractMainColor(path);
}

String _themeLabel(rust_theme.Theme theme) => switch (theme) {
  rust_theme.Theme.light => 'Light',
  rust_theme.Theme.dark => 'Dark',
};
