// Smoke test for basic OxideStoreCore lifecycle.
//
// Why: confirms the minimum contract expected by generated store wrappers.
import 'package:flutter_test/flutter_test.dart';

import 'package:oxide_runtime/oxide_runtime.dart';

void main() {
  test('core initializes, dispatches, and disposes engine', () async {
    var disposed = false;

    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (initialState) async => initialState ?? 0,
      disposeEngine: (engine) => disposed = true,
      dispatch: (engine, action) async => _Snap(engine + action),
      current: (engine) async => _Snap(engine),
      stateStream: (_) => const Stream<_Snap>.empty(),
      stateFromSnapshot: (snap) => snap.state,
    );

    await core.initialize(initialState: 10);
    expect(core.state, 10);

    await core.dispatchAction(5);
    expect(core.state, 15);

    await core.dispose();
    expect(disposed, true);
  });
}

final class _Snap {
  _Snap(this.state);
  final int state;
}
