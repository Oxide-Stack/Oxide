import 'package:flutter_test/flutter_test.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

enum TestSlice { a, b }

final class TestSnapshot {
  const TestSnapshot(this.slices);

  final List<TestSlice> slices;
}

void main() {
  test('filterSnapshotsBySlices passes full updates and selected slices', () async {
    final source = Stream<TestSnapshot>.fromIterable(const [
      TestSnapshot([]),
      TestSnapshot([TestSlice.a]),
      TestSnapshot([TestSlice.b]),
      TestSnapshot([TestSlice.a, TestSlice.b]),
    ]);

    final filtered = filterSnapshotsBySlices<TestSnapshot, TestSlice>(
      source,
      const [TestSlice.a],
      (snap) => snap.slices,
    );

    final results = await filtered.map((s) => s.slices).toList();
    expect(results, const [
      [],
      [TestSlice.a],
      [TestSlice.a, TestSlice.b],
    ]);
  });
}

