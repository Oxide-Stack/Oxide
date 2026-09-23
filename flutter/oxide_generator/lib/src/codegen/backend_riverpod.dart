import 'helpers.dart';
import 'model.dart';

// Riverpod Notifier backend templates.
//
// Riverpod is a common app-layer integration; this template wires the core
// store into a NotifierProvider while keeping the engine lifecycle consistent.
String buildRiverpodBackend(OxideCodegenConfig c, String coreInstantiation) {
  final snapshotsStream = (c.slices == null || c.slices!.isEmpty)
      ? '_core.snapshots'
      : 'filterSnapshotsBySlices<${c.snapshotType}, ${c.sliceType!}>('
            '_core.snapshots, '
            'const [${c.slices!.join(', ')}], '
            '(snap) => snap.slices'
            ')';
  return '''
final ${lowerFirst(c.prefix)}Provider = ${c.keepAlive ? 'NotifierProvider' : 'NotifierProvider.autoDispose'}<
    ${c.prefix}Notifier,
    OxideView<${c.stateType}, ${c.prefix}Actions>>(
  () => ${c.prefix}Notifier(),
);

class ${c.prefix}Notifier
    extends Notifier<OxideView<${c.stateType}, ${c.prefix}Actions>> {
  ${c.prefix}Notifier();

  late final ${c.prefix}Actions actions = ${c.prefix}Actions._(_dispatch);
  StreamSubscription<${c.snapshotType}>? _subscription;
  StreamSubscription<OxideStoreFailure>? _failureSubscription;
  Completer<void>? _initializing;
  bool _initialized = false;
  bool _disposeHandled = false;

$coreInstantiation

  @override
  OxideView<${c.stateType}, ${c.prefix}Actions> build() {
    actions._bind(_dispatch);
    // `ref.invalidate` on a keepAlive provider re-runs build() on the same
    // Notifier instance after onDispose disposed the old core. Recreate the
    // core so the new lifecycle owns a fresh engine instead of serving dead
    // state. autoDispose providers get a fresh instance and never hit this.
    if (_disposeHandled) {
      _disposeHandled = false;
      _initialized = false;
      _initializing = null;
      _subscription = null;
      _failureSubscription = null;
      _core = _createCore();
    }
    if (!_disposeHandled) {
      ref.onDispose(() {
        _disposeHandled = true;
        _subscription?.cancel();
        _failureSubscription?.cancel();
        unawaited(_core.dispose());
      });
    }

    // Normal rebuilds reuse the in-flight/finished initialization to avoid
    // leaking a second native engine; only a post-dispose rebuild (above)
    // starts over with a fresh core.
    if (_initialized || _initializing != null) {
      return OxideView(
        state: _core.state,
        actions: actions,
        isLoading: _core.isLoading,
        error: _core.error,
      );
    }
    _initializing = Completer<void>();
    _startInitialization();
    return OxideView(
      state: null,
      actions: actions,
      isLoading: true,
      error: null,
    );
  }

  void _startInitialization() {
    final initializing = _initializing!;
    _subscription = $snapshotsStream.listen((_) {
      if (!ref.mounted) return;
      state = _view();
    });
    _failureSubscription = _core.failures.listen((_) {
      if (!ref.mounted) return;
      state = _view();
    });

    unawaited(() async {
      try {
        await _core.initialize();
      } finally {
        _initialized = true;
        initializing.complete();
        _initializing = null;
      }
      // If disposal raced initialization, unwind the engine we just created
      // instead of leaking it.
      if (_disposeHandled) {
        _subscription?.cancel();
        _failureSubscription?.cancel();
        unawaited(_core.dispose());
      }
    }());
  }

  OxideView<${c.stateType}, ${c.prefix}Actions> _view() {
    return OxideView(
      state: _core.state,
      actions: actions,
      isLoading: _core.isLoading,
      error: _core.error,
    );
  }

  Future<void> _dispatch(${c.actionsType} action) async {
    await _core.dispatchAction(action);
    // state update will be handled by the stream listener; no manual write to
    // avoid duplicates.
  }
}
''';
}
