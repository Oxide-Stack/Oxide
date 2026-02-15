import 'dart:async';

import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import 'memory_probe.dart';

final class RoutingBenchResult {
  const RoutingBenchResult({
    required this.iterations,
    required this.navTotal,
    required this.navMean,
    required this.resolutionTotal,
    required this.resolutionMean,
    required this.rssBeforeBytes,
    required this.rssAfterBytes,
  });

  final int iterations;
  final Duration navTotal;
  final Duration navMean;
  final Duration resolutionTotal;
  final Duration resolutionMean;
  final int? rssBeforeBytes;
  final int? rssAfterBytes;
}

final class RoutingBenchScreen extends StatefulWidget {
  const RoutingBenchScreen({super.key});

  @override
  State<RoutingBenchScreen> createState() => _RoutingBenchScreenState();
}

final class _RoutingBenchScreenState extends State<RoutingBenchScreen> {
  int _iterations = 200;
  Future<RoutingBenchResult>? _run;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Routing Benchmark'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.of(context).maybePop(),
        ),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Text('Iterations'),
                const SizedBox(width: 12),
                SizedBox(
                  width: 120,
                  child: TextFormField(
                    initialValue: _iterations.toString(),
                    keyboardType: TextInputType.number,
                    onChanged: (v) {
                      final n = int.tryParse(v);
                      if (n != null && n > 0) setState(() => _iterations = n);
                    },
                  ),
                ),
                const SizedBox(width: 12),
                FilledButton(
                  onPressed: () {
                    setState(() => _run = _runBench(GoRouter.of(context), _iterations));
                  },
                  child: const Text('Run'),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Expanded(
              child: FutureBuilder<RoutingBenchResult>(
                future: _run,
                builder: (context, snapshot) {
                  if (_run == null) {
                    return const Text('Press Run to execute the routing benchmark.');
                  }
                  if (!snapshot.hasData) {
                    return const Center(child: CircularProgressIndicator());
                  }
                  final r = snapshot.requireData;
                  return _ResultView(result: r);
                },
              ),
            ),
          ],
        ),
      ),
    );
  }

  Future<RoutingBenchResult> _runBench(GoRouter router, int iterations) async {
    final rssBefore = currentRssBytes();

    final navSw = Stopwatch()..start();
    var navTotalMicros = 0;
    for (var i = 0; i < iterations; i++) {
      final sw = Stopwatch()..start();
      final future = router.push<void>('/bench/$i');
      await Future<void>.delayed(Duration.zero);
      router.pop();
      await future;
      sw.stop();
      navTotalMicros += sw.elapsedMicroseconds;
    }
    navSw.stop();

    final parseSw = Stopwatch()..start();
    var parseTotalMicros = 0;
    final parser = router.routeInformationParser;
    for (var i = 0; i < iterations; i++) {
      final sw = Stopwatch()..start();
      await parser.parseRouteInformation(RouteInformation(uri: Uri.parse('/bench/$i')));
      sw.stop();
      parseTotalMicros += sw.elapsedMicroseconds;
    }
    parseSw.stop();

    await Future<void>.delayed(const Duration(milliseconds: 10));
    final rssAfter = currentRssBytes();

    return RoutingBenchResult(
      iterations: iterations,
      navTotal: navSw.elapsed,
      navMean: Duration(microseconds: navTotalMicros ~/ iterations),
      resolutionTotal: parseSw.elapsed,
      resolutionMean: Duration(microseconds: parseTotalMicros ~/ iterations),
      rssBeforeBytes: rssBefore,
      rssAfterBytes: rssAfter,
    );
  }
}

final class _ResultView extends StatelessWidget {
  const _ResultView({required this.result});

  final RoutingBenchResult result;

  @override
  Widget build(BuildContext context) {
    final rssBefore = result.rssBeforeBytes;
    final rssAfter = result.rssAfterBytes;
    final rssDelta = (rssBefore != null && rssAfter != null) ? (rssAfter - rssBefore) : null;

    return ListView(
      children: [
        Text('Iterations: ${result.iterations}'),
        const SizedBox(height: 8),
        Text('Navigation total: ${result.navTotal.inMilliseconds} ms'),
        Text('Navigation mean: ${result.navMean.inMicroseconds} µs/op'),
        const SizedBox(height: 8),
        Text('Resolution total: ${result.resolutionTotal.inMilliseconds} ms'),
        Text('Resolution mean: ${result.resolutionMean.inMicroseconds} µs/op'),
        const SizedBox(height: 8),
        Text('RSS before: ${rssBefore ?? 'n/a'} bytes'),
        Text('RSS after: ${rssAfter ?? 'n/a'} bytes'),
        Text('RSS delta: ${rssDelta ?? 'n/a'} bytes'),
      ],
    );
  }
}

