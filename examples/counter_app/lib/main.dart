import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'oxide_generated/navigation/navigation_runtime.g.dart';
import 'oxide_generated/routes/route_kind.g.dart';
import 'src/oxide.dart';
import 'src/rust/api/bridge.dart' show initOxide;
import 'src/rust/frb_generated.dart';
import 'src/rust/state/app_state.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initOxide();
  oxideNavStart();
  runApp(const ProviderScope(child: MyApp()));
}

@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(navigatorKey: oxideNavigatorKey, home: const CounterSplashScreen());
  }
}

@OxideRoutePage(RouteKind.splash)
final class CounterSplashScreen extends ConsumerWidget {
  const CounterSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(stateBridgeRiverpodOxideProvider);
    return const Scaffold(body: Center(child: Text('Loading…')));
  }
}

@OxideRoutePage(RouteKind.home)
final class CounterHomeScreen extends StatelessWidget {
  const CounterHomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 4,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Counter (4 Backends)'),
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
            StateBridgeOxideScope(child: _InheritedPane()),
            StateBridgeHooksOxideScope(child: _HooksPane()),
            _RiverpodPane(),
            _BlocPane(),
          ],
        ),
      ),
    );
  }
}

class _Controls extends StatelessWidget {
  const _Controls({required this.counter, required this.onInc, required this.onDec, required this.onReset});

  final int counter;
  final Future<void> Function() onInc;
  final Future<void> Function() onDec;
  final Future<void> Function() onReset;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Counter: $counter', style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: FilledButton(onPressed: () => onDec(), child: const Text('Decrement')),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: FilledButton(onPressed: () => onInc(), child: const Text('Increment')),
              ),
            ],
          ),
          const SizedBox(height: 8),
          FilledButton(onPressed: () => onReset(), child: const Text('Reset')),
        ],
      ),
    );
  }
}

class _InheritedPane extends StatelessWidget {
  const _InheritedPane();

  @override
  Widget build(BuildContext context) {
    final controller = StateBridgeOxideScope.controllerOf(context);
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        final view = controller.oxide;
        final state = view.state;
        if (view.isLoading) return const Center(child: Text('Loading…'));
        if (view.error != null) return Center(child: Text('Error: ${view.error}'));
        if (state == null) return const Center(child: Text('No state'));
        return _Controls(
          counter: state.counter.toInt(),
          onInc: () => view.actions.increment(),
          onDec: () => view.actions.decrement(),
          onReset: () => view.actions.reset(),
        );
      },
    );
  }
}

class _HooksPane extends HookWidget {
  const _HooksPane();

  @override
  Widget build(BuildContext context) {
    final view = useStateBridgeHooksOxideOxide();
    final state = view.state;
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    return _Controls(
      counter: state.counter.toInt(),
      onInc: () => view.actions.increment(),
      onDec: () => view.actions.decrement(),
      onReset: () => view.actions.reset(),
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(stateBridgeRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    final actions = ref.read(stateBridgeRiverpodOxideProvider).actions;
    return _Controls(
      counter: state.counter.toInt(),
      onInc: () => actions.increment(),
      onDec: () => actions.decrement(),
      onReset: () => actions.reset(),
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return StateBridgeBlocOxideScope(
      child: BlocBuilder<StateBridgeBlocOxideCubit, OxideView<AppState, StateBridgeBlocOxideActions>>(
        builder: (context, view) {
          final state = view.state;
          if (view.isLoading) return const Center(child: Text('Loading…'));
          if (view.error != null) return Center(child: Text('Error: ${view.error}'));
          if (state == null) return const Center(child: Text('No state'));
          final actions = context.read<StateBridgeBlocOxideCubit>().actions;
          return _Controls(
            counter: state.counter.toInt(),
            onInc: () => actions.increment(),
            onDec: () => actions.decrement(),
            onReset: () => actions.reset(),
          );
        },
      ),
    );
  }
}
