// Regression tests for `OxideStoreCore` snapshot forwarding and lifecycle.
//
// these behaviors are easy to break when refactoring stream wiring or
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
      dispatch: (engine, action) async => _Snap(engine + action, revision: 0),
      current: (engine) async => _Snap(engine, revision: 0),
      stateStream: (_) => controller.stream,
      stateFromSnapshot: (snap) => snap.state,
      revisionOf: (snap) => snap.revision,
    );

    final sub = core.snapshots.listen((snap) => seen.add(snap.state));
    addTearDown(sub.cancel);

    await core.initialize();
    await pumpEventQueue();
    expect(core.state, 0);
    expect(seen, [0]);

    controller.add(_Snap(7, revision: 1));
    await pumpEventQueue();
    expect(core.state, 7);
    expect(seen, [0, 7]);
  });

  test(
    'core captures stream errors and stops updating after dispose',
    () async {
      final controller = StreamController<_Snap>.broadcast();
      addTearDown(controller.close);

      var disposed = false;
      final core = OxideStoreCore<int, int, int, _Snap>(
        createEngine: (_) async => 0,
        disposeEngine: (_) => disposed = true,
        dispatch: (engine, action) async => _Snap(engine + action, revision: 0),
        current: (engine) async => _Snap(engine, revision: 0),
        stateStream: (_) => controller.stream,
        stateFromSnapshot: (snap) => snap.state,
        revisionOf: (snap) => snap.revision,
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

      controller.add(_Snap(123, revision: 0));
      await pumpEventQueue();
      expect(core.state, 0);
    },
  );

  test('core defers engine disposal until in-flight work completes', () async {
    final controller = StreamController<_Snap>.broadcast();
    addTearDown(controller.close);

    final dispatchCompleter = Completer<_Snap>();
    var disposed = false;

    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) => disposed = true,
      dispatch: (_, __) => dispatchCompleter.future,
      current: (engine) async => _Snap(engine, revision: 0),
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

    dispatchCompleter.complete(_Snap(1, revision: 0));
    await dispatchFuture;
    await disposeFuture;
    expect(disposed, true);
  });

  test('core drops duplicate revision snapshots', () async {
    final controller = StreamController<_Snap>.broadcast();
    addTearDown(controller.close);

    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) {},
      dispatch: (engine, action) async => _Snap(engine + action, revision: 1),
      current: (engine) async => _Snap(engine, revision: 1),
      stateStream: (_) => controller.stream,
      stateFromSnapshot: (snap) => snap.state,
      revisionOf: (snap) => snap.revision,
    );

    // subscribe to trigger notifications
    final seen = <int>[];
    final sub = core.snapshots.listen((snap) => seen.add(snap.revision));
    addTearDown(sub.cancel);

    await core.initialize();
    expect(core.snapshotEmissionCount, 1);

    // emit duplicate revision on stream - should be dropped
    controller.add(_Snap(999, revision: 1));
    await Future<void>.delayed(Duration.zero);
    expect(core.snapshotEmissionCount, 1);
    expect(seen, [1]);
  });

  test(
    'core exposes lifecycle getters and encodes current state bytes',
    () async {
      final core = OxideStoreCore<int, int, int, _Snap>(
        createEngine: (_) async => 41,
        disposeEngine: (_) {},
        dispatch: (engine, action) async => _Snap(engine + action, revision: 1),
        current: (engine) async => _Snap(engine, revision: 0),
        stateStream: (_) => const Stream<_Snap>.empty(),
        stateFromSnapshot: (snap) => snap.state,
        encodeCurrentState: (engine) async => [engine, engine + 1],
        revisionOf: (snap) => snap.revision,
      );

      expect(core.encodeCurrentStateBytes(), completion(isNull));

      await core.initialize();
      expect(core.isLoading, isFalse);
      expect(core.errorStackTrace, isNull);
      expect(core.engine, isNotNull);
      expect(core.snapshot, isNotNull);
      expect(core.engineCreationCount, 1);
      expect(await core.encodeCurrentStateBytes(), [41, 42]);
    },
  );

  test('core records initialization failures', () async {
    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) => Future<int>.error(StateError('init failed')),
      disposeEngine: (_) {},
      dispatch: (engine, action) async => _Snap(engine + action, revision: 0),
      current: (engine) async => _Snap(engine, revision: 0),
      stateStream: (_) => const Stream<_Snap>.empty(),
      stateFromSnapshot: (snap) => snap.state,
    );

    await core.initialize();
    expect(core.error, isA<StateError>());
    expect(core.errorStackTrace, isNotNull);
  });

  test('core disposes created engine when disposed mid-initialize', () async {
    final engineCompleter = Completer<int>();
    var disposed = false;
    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) => engineCompleter.future,
      disposeEngine: (_) => disposed = true,
      dispatch: (engine, action) async => _Snap(engine + action, revision: 0),
      current: (engine) async => _Snap(engine, revision: 0),
      stateStream: (_) => const Stream<_Snap>.empty(),
      stateFromSnapshot: (snap) => snap.state,
    );

    final initFuture = core.initialize();
    await Future<void>.delayed(Duration.zero);
    await core.dispose();

    engineCompleter.complete(1);
    await initFuture;
    await Future<void>.delayed(Duration.zero);

    expect(disposed, isTrue);
  });

  test('core records dispatch failures', () async {
    final core = OxideStoreCore<int, int, int, _Snap>(
      createEngine: (_) async => 0,
      disposeEngine: (_) {},
      dispatch: (_, __) async => throw StateError('dispatch failed'),
      current: (engine) async => _Snap(engine, revision: 0),
      stateStream: (_) => const Stream<_Snap>.empty(),
      stateFromSnapshot: (snap) => snap.state,
    );

    await core.initialize();
    await core.dispatchAction(99);

    expect(core.error, isA<StateError>());
    expect(core.errorStackTrace, isNotNull);
    expect(core.state, 0);
  });
}

final class _Snap {
  _Snap(this.state, {required this.revision});
  final int state;
  final int revision;
}
