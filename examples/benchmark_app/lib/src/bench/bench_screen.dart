import 'dart:async';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../oxide.dart';
import 'bench_charts.dart';
import 'bench_models.dart';
import 'workloads.dart';

final class _Inputs {
  const _Inputs({required this.lightJson, required this.heavyJson});

  final String lightJson;
  final String heavyJson;
}

Future<_Inputs> _loadInputs() async {
  final lightJson = await rootBundle.loadString('assets/light.json');
  final heavyJson = await rootBundle.loadString('assets/heavy.json');
  return _Inputs(lightJson: lightJson, heavyJson: heavyJson);
}

final class BenchDartState {
  const BenchDartState({required this.counter, required this.checksum});

  factory BenchDartState.initial() => const BenchDartState(counter: 0, checksum: fnvOffsetBasis);

  final int counter;
  final int checksum;
}

final benchRiverpodDartProvider = NotifierProvider<BenchRiverpodDartNotifier, BenchDartState>(BenchRiverpodDartNotifier.new);

class BenchRiverpodDartNotifier extends Notifier<BenchDartState> {
  @override
  BenchDartState build() => BenchDartState.initial();

  Future<void> runSmall({required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runCounterWorkload(counter: state.counter, checksum: state.checksum, iterations: iterations);
    state = BenchDartState(counter: counter, checksum: checksum);
  }

  Future<void> runJson({required String json, required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runJsonWorkload(counter: state.counter, checksum: state.checksum, json: json, iterations: iterations);
    state = BenchDartState(counter: counter, checksum: checksum);
  }

  Future<void> runSieve({required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runSieveWorkload(counter: state.counter, checksum: state.checksum, iterations: iterations);
    state = BenchDartState(counter: counter, checksum: checksum);
  }

  Future<void> reset() async {
    state = BenchDartState.initial();
  }
}

class BenchDartCubit extends Cubit<BenchDartState> {
  BenchDartCubit() : super(BenchDartState.initial());

  Future<void> runSmall({required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runCounterWorkload(counter: state.counter, checksum: state.checksum, iterations: iterations);
    emit(BenchDartState(counter: counter, checksum: checksum));
  }

  Future<void> runJson({required String json, required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runJsonWorkload(counter: state.counter, checksum: state.checksum, json: json, iterations: iterations);
    emit(BenchDartState(counter: counter, checksum: checksum));
  }

  Future<void> runSieve({required int iterations}) async {
    if (iterations <= 0) return;
    final (counter, checksum) = runSieveWorkload(counter: state.counter, checksum: state.checksum, iterations: iterations);
    emit(BenchDartState(counter: counter, checksum: checksum));
  }

  Future<void> reset() async {
    emit(BenchDartState.initial());
  }
}

final class BenchApp extends StatelessWidget {
  const BenchApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(home: _BenchScreen());
  }
}

final class _BenchScreen extends StatefulWidget {
  const _BenchScreen();

  @override
  State<_BenchScreen> createState() => _BenchScreenState();
}

final class _BenchScreenState extends State<_BenchScreen> {
  late final BenchDartCubit _dartBloc = BenchDartCubit();
  late final BenchCounterBlocOxideCubit _rustCounterBloc = BenchCounterBlocOxideCubit();
  late final BenchJsonBlocOxideCubit _rustJsonBloc = BenchJsonBlocOxideCubit();
  late final BenchSieveBlocOxideCubit _rustSieveBloc = BenchSieveBlocOxideCubit();
  late final Future<_Inputs> _inputs = _loadInputs();

  @override
  void dispose() {
    _dartBloc.close();
    _rustCounterBloc.close();
    _rustJsonBloc.close();
    _rustSieveBloc.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider.value(value: _dartBloc),
        BlocProvider.value(value: _rustCounterBloc),
        BlocProvider.value(value: _rustJsonBloc),
        BlocProvider.value(value: _rustSieveBloc),
      ],
      child: Scaffold(
        appBar: AppBar(title: const Text('Benchmark Dashboard')),
        body: FutureBuilder(
          future: _inputs,
          builder: (context, snap) {
            if (!snap.hasData) {
              if (snap.hasError) return Center(child: Text('Error loading assets: ${snap.error}'));
              return const Center(child: CircularProgressIndicator());
            }
            return _BenchDashboard(inputs: snap.data!);
          },
        ),
      ),
    );
  }
}

final class _BenchDashboard extends ConsumerStatefulWidget {
  const _BenchDashboard({required this.inputs});

  final _Inputs inputs;

  @override
  ConsumerState<_BenchDashboard> createState() => _BenchDashboardState();
}

final class _BenchDashboardState extends ConsumerState<_BenchDashboard> {
  BenchWorkload _workload = BenchWorkload.small;
  int _iterations = 1;
  int _samples = 20;
  int _warmup = 3;
  bool _running = false;
  String? _status;

  final Set<BenchVariant> _enabled = {
    BenchVariant.dartRiverpod,
    BenchVariant.dartBloc,
    BenchVariant.rustRiverpod,
    BenchVariant.rustBloc,
    BenchVariant.rustHooks,
  };

  final Map<BenchVariant, List<Duration>> _samplesByVariant = {};

  Future<Duration> _measure(Future<void> Function() op) async {
    final sw = Stopwatch()..start();
    await op();
    await WidgetsBinding.instance.endOfFrame;
    sw.stop();
    return sw.elapsed;
  }

  Future<void> _runAll() async {
    if (_running) return;
    setState(() {
      _running = true;
      _status = null;
      _samplesByVariant.clear();
    });

    try {
      final dartRiverpod = ref.read(benchRiverpodDartProvider.notifier);
      final dartBloc = context.read<BenchDartCubit>();

      final rustCounterRiverpodView = ref.read(benchCounterRiverpodOxideProvider);
      final rustJsonRiverpodView = ref.read(benchJsonRiverpodOxideProvider);
      final rustSieveRiverpodView = ref.read(benchSieveRiverpodOxideProvider);

      final rustCounterHooksController = BenchCounterHooksOxideScope.controllerOf(context);
      final rustJsonHooksController = BenchJsonHooksOxideScope.controllerOf(context);
      final rustSieveHooksController = BenchSieveHooksOxideScope.controllerOf(context);

      final rustCounterBloc = context.read<BenchCounterBlocOxideCubit>();
      final rustJsonBloc = context.read<BenchJsonBlocOxideCubit>();
      final rustSieveBloc = context.read<BenchSieveBlocOxideCubit>();

      Future<void> runVariant(BenchVariant v) async {
        final jsonLight = widget.inputs.lightJson;
        final jsonHeavy = widget.inputs.heavyJson;
        switch (v) {
          case BenchVariant.dartRiverpod:
            switch (_workload) {
              case BenchWorkload.small:
                await dartRiverpod.runSmall(iterations: _iterations);
              case BenchWorkload.sieve:
                await dartRiverpod.runSieve(iterations: _iterations);
              case BenchWorkload.jsonLight:
                await dartRiverpod.runJson(json: jsonLight, iterations: _iterations);
              case BenchWorkload.jsonHeavy:
                await dartRiverpod.runJson(json: jsonHeavy, iterations: _iterations);
            }
          case BenchVariant.dartBloc:
            switch (_workload) {
              case BenchWorkload.small:
                await dartBloc.runSmall(iterations: _iterations);
              case BenchWorkload.sieve:
                await dartBloc.runSieve(iterations: _iterations);
              case BenchWorkload.jsonLight:
                await dartBloc.runJson(json: jsonLight, iterations: _iterations);
              case BenchWorkload.jsonHeavy:
                await dartBloc.runJson(json: jsonHeavy, iterations: _iterations);
            }
          case BenchVariant.rustRiverpod:
            switch (_workload) {
              case BenchWorkload.small:
                if (rustCounterRiverpodView.isLoading || rustCounterRiverpodView.error != null) return;
                await rustCounterRiverpodView.actions.run(iterations: _iterations);
              case BenchWorkload.sieve:
                if (rustSieveRiverpodView.isLoading || rustSieveRiverpodView.error != null) return;
                await rustSieveRiverpodView.actions.run(iterations: _iterations);
              case BenchWorkload.jsonLight:
                if (rustJsonRiverpodView.isLoading || rustJsonRiverpodView.error != null) return;
                await rustJsonRiverpodView.actions.runLight(iterations: _iterations);
              case BenchWorkload.jsonHeavy:
                if (rustJsonRiverpodView.isLoading || rustJsonRiverpodView.error != null) return;
                await rustJsonRiverpodView.actions.runHeavy(iterations: _iterations);
            }
          case BenchVariant.rustBloc:
            switch (_workload) {
              case BenchWorkload.small:
                await rustCounterBloc.actions.run(iterations: _iterations);
              case BenchWorkload.sieve:
                await rustSieveBloc.actions.run(iterations: _iterations);
              case BenchWorkload.jsonLight:
                await rustJsonBloc.actions.runLight(iterations: _iterations);
              case BenchWorkload.jsonHeavy:
                await rustJsonBloc.actions.runHeavy(iterations: _iterations);
            }
          case BenchVariant.rustHooks:
            switch (_workload) {
              case BenchWorkload.small:
                if (rustCounterHooksController.isLoading || rustCounterHooksController.error != null) return;
                await rustCounterHooksController.actions.run(iterations: _iterations);
              case BenchWorkload.sieve:
                if (rustSieveHooksController.isLoading || rustSieveHooksController.error != null) return;
                await rustSieveHooksController.actions.run(iterations: _iterations);
              case BenchWorkload.jsonLight:
                if (rustJsonHooksController.isLoading || rustJsonHooksController.error != null) return;
                await rustJsonHooksController.actions.runLight(iterations: _iterations);
              case BenchWorkload.jsonHeavy:
                if (rustJsonHooksController.isLoading || rustJsonHooksController.error != null) return;
                await rustJsonHooksController.actions.runHeavy(iterations: _iterations);
            }
        }
      }

      final variants = BenchVariant.values.where(_enabled.contains).toList();

      for (var i = 0; i < _warmup + _samples; i++) {
        for (final v in variants) {
          if (!mounted) return;
          setState(() => _status = 'Running ${_workload.label}: ${i + 1}/${_warmup + _samples} • ${v.label}');
          final elapsed = await _measure(() => runVariant(v));
          if (i >= _warmup) {
            (_samplesByVariant[v] ??= []).add(elapsed);
          }
        }
      }
    } finally {
      if (mounted) {
        setState(() {
          _running = false;
          _status = null;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final rustCounterRiverpodView = ref.watch(benchCounterRiverpodOxideProvider);
    final rustJsonRiverpodView = ref.watch(benchJsonRiverpodOxideProvider);
    final rustSieveRiverpodView = ref.watch(benchSieveRiverpodOxideProvider);

    final rustCounterHooksController = BenchCounterHooksOxideScope.controllerOf(context);
    final rustJsonHooksController = BenchJsonHooksOxideScope.controllerOf(context);
    final rustSieveHooksController = BenchSieveHooksOxideScope.controllerOf(context);

    final rustRiverpodReady = switch (_workload) {
      BenchWorkload.small => !rustCounterRiverpodView.isLoading && rustCounterRiverpodView.error == null,
      BenchWorkload.sieve => !rustSieveRiverpodView.isLoading && rustSieveRiverpodView.error == null,
      BenchWorkload.jsonLight => !rustJsonRiverpodView.isLoading && rustJsonRiverpodView.error == null,
      BenchWorkload.jsonHeavy => !rustJsonRiverpodView.isLoading && rustJsonRiverpodView.error == null,
    };
    final rustHooksReady = switch (_workload) {
      BenchWorkload.small => !rustCounterHooksController.isLoading && rustCounterHooksController.error == null,
      BenchWorkload.sieve => !rustSieveHooksController.isLoading && rustSieveHooksController.error == null,
      BenchWorkload.jsonLight => !rustJsonHooksController.isLoading && rustJsonHooksController.error == null,
      BenchWorkload.jsonHeavy => !rustJsonHooksController.isLoading && rustJsonHooksController.error == null,
    };
    final hasResults = _samplesByVariant.values.any((samples) => samples.isNotEmpty);

    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        _Controls(
          workload: _workload,
          iterations: _iterations,
          samples: _samples,
          warmup: _warmup,
          enabled: _enabled,
          running: _running,
          rustRiverpodReady: rustRiverpodReady,
          rustHooksReady: rustHooksReady,
          onWorkloadChanged: (w) => setState(() => _workload = w),
          onIterationsChanged: (v) => setState(() => _iterations = v),
          onSamplesChanged: (v) => setState(() => _samples = v),
          onWarmupChanged: (v) => setState(() => _warmup = v),
          onToggleVariant: (v, enabled) {
            setState(() {
              if (enabled) {
                _enabled.add(v);
              } else {
                _enabled.remove(v);
              }
            });
          },
          onRun: _runAll,
        ),
        if (_status != null) Padding(padding: const EdgeInsets.only(top: 12), child: Text(_status!)),
        const SizedBox(height: 16),
        _ResultTable(samplesByVariant: _samplesByVariant),
        if (hasResults && !_running) ...[
          const SizedBox(height: 12),
          Align(
            alignment: Alignment.centerLeft,
            child: FilledButton.tonal(
              onPressed: () {
                Navigator.of(context).push(
                  MaterialPageRoute(
                    builder: (_) => _ChartsView(samplesByVariant: _samplesByVariant, iterations: _iterations, samples: _samples, warmup: _warmup),
                  ),
                );
              },
              child: const Text('View Charts'),
            ),
          ),
        ],
      ],
    );
  }
}

final class _Controls extends StatelessWidget {
  const _Controls({
    required this.workload,
    required this.iterations,
    required this.samples,
    required this.warmup,
    required this.enabled,
    required this.running,
    required this.rustRiverpodReady,
    required this.rustHooksReady,
    required this.onWorkloadChanged,
    required this.onIterationsChanged,
    required this.onSamplesChanged,
    required this.onWarmupChanged,
    required this.onToggleVariant,
    required this.onRun,
  });

  final BenchWorkload workload;
  final int iterations;
  final int samples;
  final int warmup;
  final Set<BenchVariant> enabled;
  final bool running;
  final bool rustRiverpodReady;
  final bool rustHooksReady;
  final ValueChanged<BenchWorkload> onWorkloadChanged;
  final ValueChanged<int> onIterationsChanged;
  final ValueChanged<int> onSamplesChanged;
  final ValueChanged<int> onWarmupChanged;
  final void Function(BenchVariant v, bool enabled) onToggleVariant;
  final Future<void> Function() onRun;

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: InputDecorator(
                    decoration: const InputDecoration(labelText: 'Workload', border: OutlineInputBorder()),
                    child: DropdownButtonHideUnderline(
                      child: DropdownButton<BenchWorkload>(
                        value: workload,
                        isExpanded: true,
                        items: [for (final w in BenchWorkload.values) DropdownMenuItem(value: w, child: Text(w.label))],
                        onChanged: running ? null : (w) => w == null ? null : onWorkloadChanged(w),
                      ),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                FilledButton(onPressed: running ? null : () => unawaited(onRun()), child: Text(running ? 'Running…' : 'Run')),
              ],
            ),
            const SizedBox(height: 12),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              children: [
                _IntChips(
                  label: 'Iterations',
                  value: iterations,
                  options: const [1, 5, 10, 100, 1000],
                  enabled: !running,
                  onChanged: onIterationsChanged,
                ),
                _IntChips(label: 'Samples', value: samples, options: const [5, 10, 20, 40], enabled: !running, onChanged: onSamplesChanged),
                _IntChips(label: 'Warmup', value: warmup, options: const [0, 1, 3, 5], enabled: !running, onChanged: onWarmupChanged),
              ],
            ),
            const SizedBox(height: 12),
            Text('Variants', style: Theme.of(context).textTheme.titleSmall),
            const SizedBox(height: 8),
            Wrap(
              spacing: 12,
              runSpacing: 8,
              children: [
                for (final v in BenchVariant.values)
                  FilterChip(
                    selected: enabled.contains(v),
                    onSelected: running ? null : (selected) => onToggleVariant(v, selected),
                    label: Text(v.label),
                  ),
              ],
            ),
            const SizedBox(height: 8),
            if (!rustRiverpodReady && enabled.contains(BenchVariant.rustRiverpod))
              Text('Rust Riverpod engine not ready (loading/error).', style: TextStyle(color: Theme.of(context).colorScheme.error)),
            if (!rustHooksReady && enabled.contains(BenchVariant.rustHooks))
              Text('Rust Hooks engine not ready (loading/error).', style: TextStyle(color: Theme.of(context).colorScheme.error)),
          ],
        ),
      ),
    );
  }
}

final class _IntChips extends StatelessWidget {
  const _IntChips({required this.label, required this.value, required this.options, required this.enabled, required this.onChanged});

  final String label;
  final int value;
  final List<int> options;
  final bool enabled;
  final ValueChanged<int> onChanged;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Text('$label:'),
        const SizedBox(width: 8),
        for (final v in options)
          Padding(
            padding: const EdgeInsets.only(right: 6),
            child: ChoiceChip(label: Text('$v'), selected: value == v, onSelected: enabled ? (_) => onChanged(v) : null),
          ),
      ],
    );
  }
}

final class _ResultTable extends StatelessWidget {
  const _ResultTable({required this.samplesByVariant});

  final Map<BenchVariant, List<Duration>> samplesByVariant;

  @override
  Widget build(BuildContext context) {
    final rows = <DataRow>[];
    for (final v in BenchVariant.values) {
      final summary = summarizeDurations(samplesByVariant[v] ?? const []);
      if (summary == null) continue;
      rows.add(
        DataRow(
          cells: [
            DataCell(Text(v.label)),
            DataCell(Text('${summary.runs}')),
            DataCell(Text(_fmt(summary.median))),
            DataCell(Text(_fmt(summary.mean))),
            DataCell(Text(_fmt(summary.p95))),
            DataCell(Text('${_fmt(summary.min)} / ${_fmt(summary.max)}')),
          ],
        ),
      );
    }

    if (rows.isEmpty) return const SizedBox.shrink();

    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: LayoutBuilder(
          builder: (context, constraints) {
            const minTableWidth = 720.0;
            final tableWidth = max(minTableWidth, constraints.maxWidth);
            return SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: ConstrainedBox(
                constraints: BoxConstraints(minWidth: tableWidth),
                child: DataTable(
                  columns: const [
                    DataColumn(label: Text('Variant')),
                    DataColumn(label: Text('Runs')),
                    DataColumn(label: Text('Median')),
                    DataColumn(label: Text('Mean')),
                    DataColumn(label: Text('P95')),
                    DataColumn(label: Text('Min/Max')),
                  ],
                  rows: rows,
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}

final class _ChartsView extends StatelessWidget {
  const _ChartsView({required this.samplesByVariant, required this.iterations, required this.samples, required this.warmup});

  final Map<BenchVariant, List<Duration>> samplesByVariant;
  final int iterations;
  final int samples;
  final int warmup;

  @override
  Widget build(BuildContext context) {
    final groups = <BenchMetricGroup>[];
    for (final v in BenchVariant.values) {
      final summary = summarizeDurations(samplesByVariant[v] ?? const []);
      if (summary == null) continue;
      groups.add(BenchMetricGroup(label: v.label, median: summary.median, mean: summary.mean, p95: summary.p95));
    }

    final BenchMetricGroup? fastestMedian = groups.isEmpty ? null : groups.reduce((a, b) => a.median <= b.median ? a : b);
    final BenchMetricGroup? slowestMedian = groups.isEmpty ? null : groups.reduce((a, b) => a.median >= b.median ? a : b);
    final double? medianPctFaster = (fastestMedian == null || slowestMedian == null || slowestMedian.median.inMicroseconds == 0)
        ? null
        : ((slowestMedian.median.inMicroseconds - fastestMedian.median.inMicroseconds) / slowestMedian.median.inMicroseconds) * 100.0;

    return Scaffold(
      appBar: AppBar(title: const Text('Benchmark Charts')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Median / Mean / P95', style: Theme.of(context).textTheme.titleSmall),
                  const SizedBox(height: 8),
                  Wrap(
                    spacing: 8,
                    runSpacing: 8,
                    children: [
                      Chip(label: Text('Iterations: $iterations')),
                      Chip(label: Text('Samples: $samples')),
                      Chip(label: Text('Warmup: $warmup')),
                      if (fastestMedian != null) Chip(label: Text('Fastest (median): ${fastestMedian.label} • ${_fmt(fastestMedian.median)}')),
                      if (slowestMedian != null) Chip(label: Text('Slowest (median): ${slowestMedian.label} • ${_fmt(slowestMedian.median)}')),
                      if (medianPctFaster != null) Chip(label: Text('Median delta: ${medianPctFaster.toStringAsFixed(0)}% faster')),
                    ],
                  ),
                  const SizedBox(height: 8),
                  const _ChartLegend(),
                  const SizedBox(height: 8),
                  Text('Tap a bar for exact values.', style: Theme.of(context).textTheme.bodySmall),
                  const SizedBox(height: 12),
                  BenchGroupedBarChart(groups: groups),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

final class _ChartLegend extends StatelessWidget {
  const _ChartLegend();

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Wrap(
      spacing: 12,
      runSpacing: 8,
      children: [
        _LegendItem(label: 'Median', color: colors.primary),
        _LegendItem(label: 'Mean', color: colors.secondary),
        _LegendItem(label: 'P95', color: colors.tertiary),
      ],
    );
  }
}

final class _LegendItem extends StatelessWidget {
  const _LegendItem({required this.label, required this.color});

  final String label;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(
          width: 12,
          height: 12,
          decoration: BoxDecoration(color: color, borderRadius: BorderRadius.circular(3)),
        ),
        const SizedBox(width: 6),
        Text(label),
      ],
    );
  }
}

String _fmt(Duration d) {
  final micros = d.inMicroseconds;
  if (micros >= 1000) return '${(micros / 1000).toStringAsFixed(2)} ms';
  return '$micros µs';
}
