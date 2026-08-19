import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';
import 'package:showcase_app/src/rust/api/settings/state.dart';
import 'package:showcase_app/src/rust/config/enums/theme_type.dart' as rust_theme;

class BackendMatrixScreen extends ConsumerWidget {
  const BackendMatrixScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final actions = ref.watch(settingsControllerProvider).actions;
    return DefaultTabController(
      length: 4,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Backend Matrix'),
          leading: IconButton(
            onPressed: () => actions.pop(),
            icon: const Icon(Icons.arrow_back),
          ),
          bottom: const TabBar(
            tabs: [
              Tab(text: 'Inherited'),
              Tab(text: 'Hooks'),
              Tab(text: 'Riverpod'),
              Tab(text: 'BLoC'),
            ],
          ),
        ),
        body: const TabBarView(
          children: [
            SettingsInheritedBridgeScope(child: _InheritedPane()),
            SettingsHooksBridgeScope(child: _HooksPane()),
            _RiverpodPane(),
            SettingsBlocBridgeScope(child: _BlocPane()),
          ],
        ),
      ),
    );
  }
}

class _InheritedPane extends StatelessWidget {
  const _InheritedPane();

  @override
  Widget build(BuildContext context) {
    final controller = SettingsInheritedBridgeScope.controllerOf(context);
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        final view = controller.oxide;
        final state = view.state;
        if (view.isLoading || state == null) return const _LoadingPane();
        return _BackendControls(
          backendLabel: 'InheritedWidget',
          state: state,
          onToggleTheme: () {
            final next = state.theme == rust_theme.Theme.dark
                ? rust_theme.Theme.light
                : rust_theme.Theme.dark;
            unawaited(view.actions.setTheme(next));
          },
          onSetColor: () => unawaited(view.actions.setMainColor('#6750A4')),
          onReset: () => unawaited(view.actions.resetState()),
        );
      },
    );
  }
}

class _HooksPane extends HookWidget {
  const _HooksPane();

  @override
  Widget build(BuildContext context) {
    final view = useSettingsHooksBridgeOxide();
    final state = view.state;
    if (view.isLoading || state == null) return const _LoadingPane();
    return _BackendControls(
      backendLabel: 'Inherited Hooks',
      state: state,
      onToggleTheme: () {
        final next = state.theme == rust_theme.Theme.dark
            ? rust_theme.Theme.light
            : rust_theme.Theme.dark;
        unawaited(view.actions.setTheme(next));
      },
      onSetColor: () => unawaited(view.actions.setMainColor('#009688')),
      onReset: () => unawaited(view.actions.resetState()),
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(settingsControllerProvider);
    final state = view.state;
    if (view.isLoading || state == null) return const _LoadingPane();
    final actions = ref.read(settingsControllerProvider).actions;
    return _BackendControls(
      backendLabel: 'Riverpod',
      state: state,
      onToggleTheme: () {
        final next = state.theme == rust_theme.Theme.dark
            ? rust_theme.Theme.light
            : rust_theme.Theme.dark;
        unawaited(actions.setTheme(next));
      },
      onSetColor: () => unawaited(actions.setMainColor('#E91E63')),
      onReset: () => unawaited(actions.resetState()),
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<
      SettingsBlocBridgeCubit,
      OxideView<SettingsState, SettingsBlocBridgeActions>
    >(
      builder: (context, view) {
        final state = view.state;
        if (view.isLoading || state == null) return const _LoadingPane();
        final actions = context.read<SettingsBlocBridgeCubit>().actions;
        return _BackendControls(
          backendLabel: 'BLoC/Cubit',
          state: state,
          onToggleTheme: () {
            final next = state.theme == rust_theme.Theme.dark
                ? rust_theme.Theme.light
                : rust_theme.Theme.dark;
            unawaited(actions.setTheme(next));
          },
          onSetColor: () => unawaited(actions.setMainColor('#3F51B5')),
          onReset: () => unawaited(actions.resetState()),
        );
      },
    );
  }
}

class _BackendControls extends StatelessWidget {
  const _BackendControls({
    required this.backendLabel,
    required this.state,
    required this.onToggleTheme,
    required this.onSetColor,
    required this.onReset,
  });

  final String backendLabel;
  final SettingsState state;
  final VoidCallback onToggleTheme;
  final VoidCallback onSetColor;
  final VoidCallback onReset;

  @override
  Widget build(BuildContext context) {
    final themeLabel = state.theme == rust_theme.Theme.dark ? 'Dark' : 'Light';
    final colorLabel = state.mainColor.isEmpty ? 'Unset' : state.mainColor;

    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        Text(backendLabel, style: Theme.of(context).textTheme.headlineSmall),
        const SizedBox(height: 8),
        Text('Theme: $themeLabel'),
        Text('Main color: $colorLabel'),
        const SizedBox(height: 16),
        FilledButton(
          onPressed: onToggleTheme,
          child: const Text('Toggle theme'),
        ),
        const SizedBox(height: 8),
        OutlinedButton(
          onPressed: onSetColor,
          child: const Text('Set demo color'),
        ),
        const SizedBox(height: 8),
        TextButton(
          onPressed: onReset,
          child: const Text('Reset state'),
        ),
      ],
    );
  }
}

class _LoadingPane extends StatelessWidget {
  const _LoadingPane();

  @override
  Widget build(BuildContext context) {
    return const Center(child: CircularProgressIndicator());
  }
}
