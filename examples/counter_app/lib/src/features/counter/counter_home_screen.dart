import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../isolated_channels_demo_pane.dart';
import '../../oxide.dart';
import '../../rust/state/app_state.dart';

final class CounterHomeScreen extends StatelessWidget {
  const CounterHomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 5,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Counter (+ Channels Demo)'),
          bottom: const TabBar(
            tabs: [
              Tab(text: 'Inherited'),
              Tab(text: 'Hooks'),
              Tab(text: 'Riverpod'),
              Tab(text: 'BLoC'),
              Tab(text: 'Channels'),
            ],
          ),
        ),
        body: const TabBarView(
          children: [
            StateBridgeOxideScope(child: _InheritedPane()),
            StateBridgeHooksOxideScope(child: _HooksPane()),
            _RiverpodPane(),
            _BlocPane(),
            IsolatedChannelsDemoPane(),
          ],
        ),
      ),
    );
  }
}

class _Controls extends StatefulWidget {
  const _Controls({
    required this.counter,
    required this.lastConfirmed,
    required this.onInc,
    required this.onDec,
    required this.onReset,
    required this.onOpenDetail,
    required this.onOpenConfirm,
  });

  final int counter;
  final bool? lastConfirmed;
  final Future<void> Function() onInc;
  final Future<void> Function() onDec;
  final Future<void> Function() onReset;
  final Future<void> Function() onOpenDetail;
  final Future<void> Function() onOpenConfirm;

  @override
  State<_Controls> createState() => _ControlsState();
}

class _ControlsState extends State<_Controls> {
  bool _inFlight = false;

  Future<void> _run(Future<void> Function() op) async {
    if (_inFlight) return;
    setState(() => _inFlight = true);
    try {
      await op();
    } finally {
      if (mounted) setState(() => _inFlight = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Counter: ${widget.counter}', style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 8),
          Text('Last confirm: ${widget.lastConfirmed ?? '-'}'),
          const SizedBox(height: 16),
          if (_inFlight) const LinearProgressIndicator(),
          if (_inFlight) const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: FilledButton(
                  onPressed: _inFlight ? null : () => _run(widget.onDec),
                  child: const Text('Decrement'),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: FilledButton(
                  onPressed: _inFlight ? null : () => _run(widget.onInc),
                  child: const Text('Increment'),
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          FilledButton(onPressed: _inFlight ? null : () => _run(widget.onReset), child: const Text('Reset')),
          const SizedBox(height: 16),
          Text('Navigation', style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 8),
          Row(
            children: [
              Expanded(
                child: OutlinedButton(
                  onPressed: _inFlight ? null : () => _run(widget.onOpenDetail),
                  child: const Text('Push detail (args)'),
                ),
              ),
              const SizedBox(width: 8),
              Expanded(
                child: FilledButton(
                  onPressed: _inFlight ? null : () => _run(widget.onOpenConfirm),
                  child: const Text('Push confirm (ticket)'),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class _LoadingView extends StatelessWidget {
  const _LoadingView();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Semantics(
        label: 'Loading',
        child: CircularProgressIndicator(),
      ),
    );
  }
}

class _ErrorView extends StatelessWidget {
  const _ErrorView(this.error);

  final Object? error;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Something went wrong', style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 8),
            Text('$error', textAlign: TextAlign.center),
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
    final controller = StateBridgeOxideScope.controllerOf(context);
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        final view = controller.oxide;
        final state = view.state;
        if (view.isLoading) return const _LoadingView();
        if (view.error != null) return _ErrorView(view.error);
        if (state == null) return const Center(child: Text('No state'));
        return _Controls(
          counter: state.counter.toInt(),
          lastConfirmed: state.lastConfirmed,
          onInc: () => view.actions.increment(),
          onDec: () => view.actions.decrement(),
          onReset: () => view.actions.reset(),
          onOpenDetail: () => view.actions.openDetail(),
          onOpenConfirm: () => view.actions.openConfirm(title: 'Confirm from Rust navigation?'),
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
    if (view.isLoading) return const _LoadingView();
    if (view.error != null) return _ErrorView(view.error);
    if (state == null) return const Center(child: Text('No state'));
    return _Controls(
      counter: state.counter.toInt(),
      lastConfirmed: state.lastConfirmed,
      onInc: () => view.actions.increment(),
      onDec: () => view.actions.decrement(),
      onReset: () => view.actions.reset(),
      onOpenDetail: () => view.actions.openDetail(),
      onOpenConfirm: () => view.actions.openConfirm(title: 'Confirm from Rust navigation?'),
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(stateBridgeRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const _LoadingView();
    if (view.error != null) return _ErrorView(view.error);
    if (state == null) return const Center(child: Text('No state'));
    final actions = ref.read(stateBridgeRiverpodOxideProvider).actions;
    return _Controls(
      counter: state.counter.toInt(),
      lastConfirmed: state.lastConfirmed,
      onInc: () => actions.increment(),
      onDec: () => actions.decrement(),
      onReset: () => actions.reset(),
      onOpenDetail: () => actions.openDetail(),
      onOpenConfirm: () => actions.openConfirm(title: 'Confirm from Rust navigation?'),
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
          if (view.isLoading) return const _LoadingView();
          if (view.error != null) return _ErrorView(view.error);
          if (state == null) return const Center(child: Text('No state'));
          final actions = context.read<StateBridgeBlocOxideCubit>().actions;
          return _Controls(
            counter: state.counter.toInt(),
            lastConfirmed: state.lastConfirmed,
            onInc: () => actions.increment(),
            onDec: () => actions.decrement(),
            onReset: () => actions.reset(),
            onOpenDetail: () => actions.openDetail(),
            onOpenConfirm: () => actions.openConfirm(title: 'Confirm from Rust navigation?'),
          );
        },
      ),
    );
  }
}
