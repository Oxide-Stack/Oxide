// Regression tests for `OxideStoreCore` snapshot forwarding and lifecycle.
//
// Why: these behaviors are easy to break when refactoring stream wiring or
// disposal tracking.
import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

void main() {
  test('core forwards snapshots from the engine stream', () async {
    final controller = StreamController<_Snap>.broadcast();
    addTearDown(controller.close);

    final seen = <int>[];
    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) {},
      dispatch: (engine, action) async => _Snap(engine + action),
      current: (engine) async => _Snap(engine),
      stateStream: (_) => controller.stream,
      stateFromSnapshot: (snap) => snap.state,
    );

    final sub = core.snapshots.listen((snap) => seen.add(snap.state));
    addTearDown(sub.cancel);

    await core.initialize();
    await pumpEventQueue();
    expect(core.state, 0);
    expect(seen, [0]);

    controller.add(_Snap(7));
    await pumpEventQueue();
    expect(core.state, 7);
    expect(seen, [0, 7]);
  });

  test('core captures stream errors and stops updating after dispose', () async {
    final controller = StreamController<_Snap>.broadcast();
    addTearDown(controller.close);

    var disposed = false;
    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) => disposed = true,
      dispatch: (engine, action) async => _Snap(engine + action),
      current: (engine) async => _Snap(engine),
      stateStream: (_) => controller.stream,
      stateFromSnapshot: (snap) => snap.state,
    );

    await core.initialize();
    expect(controller.hasListener, true);

    final err = StateError('boom');
    controller.addError(err, StackTrace.current);
    await pumpEventQueue();
    expect(core.error, err);

    await core.dispose();
    expect(disposed, true);
    expect(controller.hasListener, false);

    controller.add(_Snap(123));
    await pumpEventQueue();
    expect(core.state, 0);
  });

  test('core defers engine disposal until in-flight work completes', () async {
    final controller = StreamController<_Snap>.broadcast();
    addTearDown(controller.close);

    final dispatchCompleter = Completer<_Snap>();
    var disposed = false;

    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) => disposed = true,
      dispatch: (_, __) => dispatchCompleter.future,
      current: (engine) async => _Snap(engine),
      stateStream: (_) => controller.stream,
      stateFromSnapshot: (snap) => snap.state,
    );

    await core.initialize();

    final dispatchFuture = core.dispatchAction(1);
    await pumpEventQueue();
    expect(disposed, false);

    final disposeFuture = core.dispose();
    await pumpEventQueue();
    expect(disposed, false);

    dispatchCompleter.complete(_Snap(1));
    await dispatchFuture;
    await disposeFuture;
    expect(disposed, true);
  });
}

final class _Snap {
  _Snap(this.state);
  final int state;
}
