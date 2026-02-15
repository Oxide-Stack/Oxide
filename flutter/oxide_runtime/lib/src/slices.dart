import 'dart:async';

/// Filters a snapshot stream using slice metadata produced by Rust sliced updates.
///
/// Snapshots with an empty slice list are treated as full updates and always pass
/// through the filter.
Stream<Snap> filterSnapshotsBySlices<Snap, Slice>(
  Stream<Snap> source,
  List<Slice> watchSlices,
  List<Slice> Function(Snap snap) slicesFromSnapshot,
) {
  if (watchSlices.isEmpty) return source;
  final watchSet = watchSlices.toSet();
  return source.where((snap) {
    final slices = slicesFromSnapshot(snap);
    if (slices.isEmpty) return true;
    for (final slice in slices) {
      if (watchSet.contains(slice)) return true;
    }
    return false;
  });
}
