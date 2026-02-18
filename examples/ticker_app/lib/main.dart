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
  // Initialize Flutter bindings and the Rust dynamic library before building UI.
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
    return MaterialApp(navigatorKey: oxideNavigatorKey, home: const TickerSplashScreen());
  }
}

@OxideRoutePage(RouteKind.splash)
final class TickerSplashScreen extends ConsumerWidget {
  const TickerSplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.watch(tickerControlRiverpodOxideProvider);
    return const Scaffold(body: Center(child: Text('Loading…')));
  }
}

@OxideRoutePage(RouteKind.home)
final class TickerHomeScreen extends StatelessWidget {
  const TickerHomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 4,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Ticker (Rust auto-tick thread)'),
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
            TickerControlInheritedOxideScope(
              child: TickerTickInheritedOxideScope(child: _InheritedPane()),
            ),
            TickerControlHooksOxideScope(
              child: TickerTickHooksOxideScope(child: _HooksPane()),
            ),
            _RiverpodPane(),
            _BlocPane(),
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
    final controlController = TickerControlInheritedOxideScope.controllerOf(context);
    final tickController = TickerTickInheritedOxideScope.controllerOf(context);
    return Column(
      children: [
        Expanded(
          child: AnimatedBuilder(
            animation: controlController,
            builder: (context, _) {
              final view = controlController.oxide;
              final state = view.state;
              if (view.isLoading) return const Center(child: Text('Loading…'));
              if (view.error != null) return Center(child: Text('Error: ${view.error}'));
              if (state == null) return const Center(child: Text('No state'));
              return _ControlSection(
                title: 'Control (slice: control)',
                isRunning: state.control.isRunning,
                intervalMs: state.control.intervalMs.toInt(),
                onStart: (ms) => view.actions.startTicker(intervalMs: BigInt.from(ms)),
                onStop: () => view.actions.stopTicker(),
              );
            },
          ),
        ),
        Expanded(
          child: AnimatedBuilder(
            animation: tickController,
            builder: (context, _) {
              final view = tickController.oxide;
              final state = view.state;
              if (view.isLoading) return const Center(child: Text('Loading…'));
              if (view.error != null) return Center(child: Text('Error: ${view.error}'));
              if (state == null) return const Center(child: Text('No state'));
              return _TickSection(
                title: 'Ticks (slice: tick)',
                ticks: state.tick.ticks.toInt(),
                lastTickSource: state.tick.lastTickSource,
                onReset: () => view.actions.reset(),
                onManualTick: () => view.actions.manualTick(),
                onSideEffectTick: () => view.actions.emitSideEffectTick(),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _HooksPane extends HookWidget {
  const _HooksPane();

  @override
  Widget build(BuildContext context) {
    final controlView = useTickerControlHooksOxideOxide();
    final tickView = useTickerTickHooksOxideOxide();
    return Column(
      children: [
        Expanded(
          child: Builder(
            builder: (context) {
              final state = controlView.state;
              if (controlView.isLoading) return const Center(child: Text('Loading…'));
              if (controlView.error != null) return Center(child: Text('Error: ${controlView.error}'));
              if (state == null) return const Center(child: Text('No state'));
              return _ControlSection(
                title: 'Control (slice: control)',
                isRunning: state.control.isRunning,
                intervalMs: state.control.intervalMs.toInt(),
                onStart: (ms) => controlView.actions.startTicker(intervalMs: BigInt.from(ms)),
                onStop: () => controlView.actions.stopTicker(),
              );
            },
          ),
        ),
        Expanded(
          child: Builder(
            builder: (context) {
              final state = tickView.state;
              if (tickView.isLoading) return const Center(child: Text('Loading…'));
              if (tickView.error != null) return Center(child: Text('Error: ${tickView.error}'));
              if (state == null) return const Center(child: Text('No state'));
              return _TickSection(
                title: 'Ticks (slice: tick)',
                ticks: state.tick.ticks.toInt(),
                lastTickSource: state.tick.lastTickSource,
                onReset: () => tickView.actions.reset(),
                onManualTick: () => tickView.actions.manualTick(),
                onSideEffectTick: () => tickView.actions.emitSideEffectTick(),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      children: const [
        Expanded(child: _RiverpodControlSection()),
        Expanded(child: _RiverpodTickSection()),
      ],
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider(create: (_) => TickerControlBlocOxideCubit()),
        BlocProvider(create: (_) => TickerTickBlocOxideCubit()),
      ],
      child: Column(
        children: const [
          Expanded(child: _BlocControlSection()),
          Expanded(child: _BlocTickSection()),
        ],
      ),
    );
  }
}

class _ControlSection extends StatefulWidget {
  const _ControlSection({
    required this.title,
    required this.isRunning,
    required this.intervalMs,
    required this.onStart,
    required this.onStop,
  });

  final String title;
  final bool isRunning;
  final int intervalMs;
  final void Function(int intervalMs) onStart;
  final void Function() onStop;

  @override
  State<_ControlSection> createState() => _ControlSectionState();
}

class _ControlSectionState extends State<_ControlSection> {
  late final TextEditingController _intervalController;

  @override
  void initState() {
    super.initState();
    _intervalController = TextEditingController(text: widget.intervalMs.toString());
  }

  @override
  void didUpdateWidget(covariant _ControlSection oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.intervalMs != oldWidget.intervalMs && !_intervalController.text.contains(RegExp(r'\D'))) {
      _intervalController.text = widget.intervalMs.toString();
    }
  }

  @override
  void dispose() {
    _intervalController.dispose();
    super.dispose();
  }

  int _readIntervalMs() {
    final parsed = int.tryParse(_intervalController.text.trim());
    return (parsed ?? widget.intervalMs).clamp(1, 60 * 1000);
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(widget.title, style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 12),
          Text('Running: ${widget.isRunning}'),
          const SizedBox(height: 8),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _intervalController,
                  keyboardType: TextInputType.number,
                  decoration: const InputDecoration(labelText: 'Interval (ms)'),
                ),
              ),
              const SizedBox(width: 12),
              FilledButton(
                onPressed: () => widget.onStart(_readIntervalMs()),
                child: const Text('Start'),
              ),
              const SizedBox(width: 8),
              OutlinedButton(
                onPressed: widget.onStop,
                child: const Text('Stop'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _TickSection extends StatelessWidget {
  const _TickSection({
    required this.title,
    required this.ticks,
    required this.lastTickSource,
    required this.onReset,
    required this.onManualTick,
    required this.onSideEffectTick,
  });

  final String title;
  final int ticks;
  final String lastTickSource;
  final void Function() onReset;
  final Future<void> Function() onManualTick;
  final Future<void> Function() onSideEffectTick;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 12),
          Text('Ticks: $ticks'),
          const SizedBox(height: 8),
          Text('Last tick source: $lastTickSource'),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            children: [
              FilledButton(
                onPressed: onManualTick,
                child: const Text('Manual tick'),
              ),
              OutlinedButton(
                onPressed: onSideEffectTick,
                child: const Text('Side-effect tick'),
              ),
              TextButton(
                onPressed: onReset,
                child: const Text('Reset'),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _RiverpodControlSection extends ConsumerWidget {
  const _RiverpodControlSection();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(tickerControlRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    final actions = ref.read(tickerControlRiverpodOxideProvider).actions;
    return _ControlSection(
      title: 'Control (slice: control)',
      isRunning: state.control.isRunning,
      intervalMs: state.control.intervalMs.toInt(),
      onStart: (ms) => actions.startTicker(intervalMs: BigInt.from(ms)),
      onStop: () => actions.stopTicker(),
    );
  }
}

class _RiverpodTickSection extends ConsumerWidget {
  const _RiverpodTickSection();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(tickerTickRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    final actions = ref.read(tickerTickRiverpodOxideProvider).actions;
    return _TickSection(
      title: 'Ticks (slice: tick)',
      ticks: state.tick.ticks.toInt(),
      lastTickSource: state.tick.lastTickSource,
      onReset: () => actions.reset(),
      onManualTick: () => actions.manualTick(),
      onSideEffectTick: () => actions.emitSideEffectTick(),
    );
  }
}

class _BlocControlSection extends StatelessWidget {
  const _BlocControlSection();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TickerControlBlocOxideCubit, OxideView<AppState, TickerControlBlocOxideActions>>(
      builder: (context, view) {
        final state = view.state;
        if (view.isLoading) return const Center(child: Text('Loading…'));
        if (view.error != null) return Center(child: Text('Error: ${view.error}'));
        if (state == null) return const Center(child: Text('No state'));
        final actions = context.read<TickerControlBlocOxideCubit>().actions;
        return _ControlSection(
          title: 'Control (slice: control)',
          isRunning: state.control.isRunning,
          intervalMs: state.control.intervalMs.toInt(),
          onStart: (ms) => actions.startTicker(intervalMs: BigInt.from(ms)),
          onStop: () => actions.stopTicker(),
        );
      },
    );
  }
}

class _BlocTickSection extends StatelessWidget {
  const _BlocTickSection();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TickerTickBlocOxideCubit, OxideView<AppState, TickerTickBlocOxideActions>>(
      builder: (context, view) {
        final state = view.state;
        if (view.isLoading) return const Center(child: Text('Loading…'));
        if (view.error != null) return Center(child: Text('Error: ${view.error}'));
        if (state == null) return const Center(child: Text('No state'));
        final actions = context.read<TickerTickBlocOxideCubit>().actions;
        return _TickSection(
          title: 'Ticks (slice: tick)',
          ticks: state.tick.ticks.toInt(),
          lastTickSource: state.tick.lastTickSource,
          onReset: () => actions.reset(),
          onManualTick: () => actions.manualTick(),
          onSideEffectTick: () => actions.emitSideEffectTick(),
        );
      },
    );
  }
}
