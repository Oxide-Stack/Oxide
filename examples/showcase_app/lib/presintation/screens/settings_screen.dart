import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';

class SettingsWidget extends HookConsumerWidget {
  const SettingsWidget({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsControllerProvider);

    final colors = <Color>[
      Colors.red,
      Colors.pink,
      Colors.purple,
      Colors.deepPurple,
      Colors.indigo,
      Colors.blue,
      Colors.teal,
      Colors.green,
      Colors.orange,
      Colors.amber,
      Colors.brown,
      Colors.grey,
      Colors.black,
    ];

    final languages = <Map<String, String>>[
      {'code': 'en', 'label': 'English'},
      {'code': 'es', 'label': 'Español'},
      {'code': 'fr', 'label': 'Français'},
      {'code': 'de', 'label': 'Deutsch'},
      {'code': 'ar', 'label': 'العربية'},
    ];

    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        const Text(
          'Settings',
          style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
        ),
        const SizedBox(height: 24),
        SwitchListTile(
          contentPadding: EdgeInsets.zero,
          title: const Text('Infer color from image'),
          subtitle: const Text(
            'Automatically use a color extracted from the selected image',
          ),
          value: settings.inferColorFromImage,
          onChanged: (value) {
            ref
                .read(settingsControllerProvider.notifier)
                .setInferColorFromImage(value);
          },
        ),
        const SizedBox(height: 16),
        Text(
          settings.inferColorFromImage
              ? 'Color selection disabled while inferring from image'
              : 'Choose app color',
          style: Theme.of(context).textTheme.titleMedium,
        ),
        const SizedBox(height: 12),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: colors.map((color) {
            final selected = settings.color == color;
            return GestureDetector(
              onTap: settings.inferColorFromImage
                  ? null
                  : () => ref
                        .read(settingsControllerProvider.notifier)
                        .setColor(color),
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
                    ? const Icon(Icons.check, color: Colors.white, size: 20)
                    : null,
              ),
            );
          }).toList(),
        ),
        const SizedBox(height: 24),
        DropdownButtonFormField<String>(
          value: settings.languageCode,
          decoration: const InputDecoration(
            labelText: 'Language',
            border: OutlineInputBorder(),
          ),
          items: languages
              .map(
                (language) => DropdownMenuItem<String>(
                  value: language['code'],
                  child: Text(language['label']!),
                ),
              )
              .toList(),
          onChanged: (value) {
            if (value != null) {
              ref.read(settingsControllerProvider.notifier).setLanguage(value);
            }
          },
        ),
        const SizedBox(height: 24),
        Card(
          child: ListTile(
            leading: CircleAvatar(backgroundColor: settings.color),
            title: const Text('Preview'),
            subtitle: Text(
              'Color: ${settings.inferColorFromImage ? 'From image' : 'Selected'} • Language: ${settings.languageCode}',
            ),
          ),
        ),
      ],
    );
  }
}
