import 'dart:math';

enum BenchWorkload { small, sieve, jsonLight, jsonHeavy }

enum BenchVariant { dartRiverpod, dartBloc, rustRiverpod, rustBloc, rustHooks }

extension BenchVariantLabel on BenchVariant {
  String get label => switch (this) {
    BenchVariant.dartRiverpod => 'Dart Riverpod',
    BenchVariant.dartBloc => 'Dart BLoC/Cubit',
    BenchVariant.rustRiverpod => 'Rust/Oxide Riverpod',
    BenchVariant.rustBloc => 'Rust/Oxide BLoC/Cubit',
    BenchVariant.rustHooks => 'Rust/Oxide Hooks',
  };
}

extension BenchWorkloadLabel on BenchWorkload {
  String get label => switch (this) {
    BenchWorkload.small => 'Counter (small)',
    BenchWorkload.sieve => 'Sieve (CPU)',
    BenchWorkload.jsonLight => 'JSON (light)',
    BenchWorkload.jsonHeavy => 'JSON (heavy)',
  };
}

final class BenchSummary {
  const BenchSummary({
    required this.runs,
    required this.mean,
    required this.median,
    required this.p95,
    required this.min,
    required this.max,
  });

  final int runs;
  final Duration mean;
  final Duration median;
  final Duration p95;
  final Duration min;
  final Duration max;
}

BenchSummary? summarizeDurations(List<Duration> samples) {
  if (samples.isEmpty) return null;
  final micros = samples.map((d) => d.inMicroseconds).toList()..sort();
  final minV = micros.first;
  final maxV = micros.last;
  final meanV = micros.reduce((a, b) => a + b) ~/ micros.length;
  final medianV = micros[micros.length ~/ 2];
  final p95V = micros[_percentileIndex(micros.length, 0.95)];
  return BenchSummary(
    runs: micros.length,
    mean: Duration(microseconds: meanV),
    median: Duration(microseconds: medianV),
    p95: Duration(microseconds: p95V),
    min: Duration(microseconds: minV),
    max: Duration(microseconds: maxV),
  );
}

int _percentileIndex(int len, double p) {
  if (len <= 1) return 0;
  final idx = (p * (len - 1)).round();
  return max(0, min(len - 1, idx));
}
