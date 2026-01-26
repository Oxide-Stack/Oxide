import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'src/oxide.dart';
import 'src/rust/frb_generated.dart';
import 'src/rust/state/app_state.dart';

Future<void> main() async {
  // Initialize Flutter bindings and the Rust dynamic library before building UI.
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: DefaultTabController(
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
              StateBridgeOxideScope(child: _InheritedPane()),
              StateBridgeHooksOxideScope(child: _HooksPane()),
              _RiverpodPane(),
              _BlocPane(),
            ],
          ),
        ),
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
        return _TickerPane(
          ticks: state.ticks.toInt(),
          isRunning: state.isRunning,
          intervalMs: state.intervalMs.toInt(),
          lastTickSource: state.lastTickSource,
          onStart: (ms) => view.actions.startTicker(intervalMs: BigInt.from(ms)),
          onStop: () => view.actions.stopTicker(),
          onReset: () => view.actions.reset(),
          onManualTick: () => view.actions.manualTick(),
          onSideEffectTick: () => view.actions.emitSideEffectTick(),
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
    return _TickerPane(
      ticks: state.ticks.toInt(),
      isRunning: state.isRunning,
      intervalMs: state.intervalMs.toInt(),
      lastTickSource: state.lastTickSource,
      onStart: (ms) => view.actions.startTicker(intervalMs: BigInt.from(ms)),
      onStop: () => view.actions.stopTicker(),
      onReset: () => view.actions.reset(),
      onManualTick: () => view.actions.manualTick(),
      onSideEffectTick: () => view.actions.emitSideEffectTick(),
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
    return _TickerPane(
      ticks: state.ticks.toInt(),
      isRunning: state.isRunning,
      intervalMs: state.intervalMs.toInt(),
      lastTickSource: state.lastTickSource,
      onStart: (ms) => actions.startTicker(intervalMs: BigInt.from(ms)),
      onStop: () => actions.stopTicker(),
      onReset: () => actions.reset(),
      onManualTick: () => actions.manualTick(),
      onSideEffectTick: () => actions.emitSideEffectTick(),
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => StateBridgeBlocOxideCubit(),
      child: BlocBuilder<StateBridgeBlocOxideCubit, OxideView<AppState, StateBridgeBlocOxideActions>>(
        builder: (context, view) {
          final state = view.state;
          if (view.isLoading) return const Center(child: Text('Loading…'));
          if (view.error != null) return Center(child: Text('Error: ${view.error}'));
          if (state == null) return const Center(child: Text('No state'));
          final actions = context.read<StateBridgeBlocOxideCubit>().actions;
          return _TickerPane(
            ticks: state.ticks.toInt(),
            isRunning: state.isRunning,
            intervalMs: state.intervalMs.toInt(),
            lastTickSource: state.lastTickSource,
            onStart: (ms) => actions.startTicker(intervalMs: BigInt.from(ms)),
            onStop: () => actions.stopTicker(),
            onReset: () => actions.reset(),
            onManualTick: () => actions.manualTick(),
            onSideEffectTick: () => actions.emitSideEffectTick(),
          );
        },
      ),
    );
  }
}

class _TickerPane extends StatefulWidget {
  const _TickerPane({
    required this.ticks,
    required this.isRunning,
    required this.intervalMs,
    required this.lastTickSource,
    required this.onStart,
    required this.onStop,
    required this.onReset,
    required this.onManualTick,
    required this.onSideEffectTick,
  });

  final int ticks;
  final bool isRunning;
  final int intervalMs;
  final String lastTickSource;
  final void Function(int intervalMs) onStart;
  final void Function() onStop;
  final void Function() onReset;
  final Future<void> Function() onManualTick;
  final Future<void> Function() onSideEffectTick;

  @override
  State<_TickerPane> createState() => _TickerPaneState();
}

class _TickerPaneState extends State<_TickerPane> {
  late final TextEditingController _intervalController;

  @override
  void initState() {
    super.initState();
    _intervalController = TextEditingController(text: widget.intervalMs.toString());
  }

  @override
  void didUpdateWidget(covariant _TickerPane oldWidget) {
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
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Ticks: ${widget.ticks}', style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 4),
          Text('Last tick: ${widget.lastTickSource}', style: Theme.of(context).textTheme.bodySmall),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _intervalController,
                  keyboardType: TextInputType.number,
                  decoration: const InputDecoration(labelText: 'Interval (ms)', border: OutlineInputBorder()),
                ),
              ),
              const SizedBox(width: 12),
              FilledButton(
                onPressed: widget.isRunning ? null : () => widget.onStart(_readIntervalMs()),
                child: Text(widget.isRunning ? 'Running' : 'Start'),
              ),
              const SizedBox(width: 8),
              OutlinedButton(onPressed: widget.isRunning ? widget.onStop : null, child: const Text('Stop')),
            ],
          ),
          const SizedBox(height: 12),
          Wrap(
            spacing: 8,
            runSpacing: 8,
            children: [
              FilledButton(onPressed: () => unawaited(widget.onManualTick()), child: const Text('Manual tick')),
              OutlinedButton(onPressed: () => unawaited(widget.onSideEffectTick()), child: const Text('Side-effect tick')),
              OutlinedButton(onPressed: widget.onReset, child: const Text('Reset')),
            ],
          ),
          const SizedBox(height: 12),
          Text(
            widget.isRunning ? 'Auto ticks are generated by a Rust background thread.' : 'Auto ticks are paused.',
            style: Theme.of(context).textTheme.bodySmall,
          ),
        ],
      ),
    );
  }
}
