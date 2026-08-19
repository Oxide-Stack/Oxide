// Shared engine lifecycle and snapshot coordination for generated stores.
import 'dart:async';

import 'logger.dart';
import 'types.dart';

final class OxideStoreCore<S, A, E, Snap> {
  OxideStoreCore({
    required this.createEngine,
    required this.disposeEngine,
    required this.dispatch,
    required this.current,
    required this.stateStream,
    required this.stateFromSnapshot,
    this.initApp,
    this.encodeCurrentState,
    this.revisionOf,
  });

  final OxideCreateEngine<E, S> createEngine;
  final OxideDisposeEngine<E> disposeEngine;
  final OxideDispatch<E, A, Snap> dispatch;
  final OxideCurrent<E, Snap> current;
  final OxideStateStream<E, Snap> stateStream;
  final OxideStateFromSnapshot<S, Snap> stateFromSnapshot;
  final OxideInitApp? initApp;
  final OxideEncodeCurrentState<E>? encodeCurrentState;
  final int Function(Snap snap)? revisionOf;

  E? _engine;
  StreamSubscription<Snap>? _subscription;
  Snap? _snapshot;
  final StreamController<Snap> _snapshotsController =
      StreamController<Snap>.broadcast();

  // instrumentation counters (debug only)
  int _engineCreationCount = 0;
  int _snapshotEmissionCount = 0;

  bool _isDisposed = false;
  bool _disposeRequested = false;
  int _inFlight = 0;

  bool _isLoading = true;
  Object? _error;
  StackTrace? _errorStackTrace;

  int? _lastDeliveredRevision;

  bool get isLoading => _isLoading;
  Object? get error => _error;
  StackTrace? get errorStackTrace => _errorStackTrace;
  E? get engine => _engine;
  Snap? get snapshot => _snapshot;
  int get engineCreationCount => _engineCreationCount;
  int get snapshotEmissionCount => _snapshotEmissionCount;
  S? get state {
    final snapshot = _snapshot;
    if (snapshot == null) return null;
    return stateFromSnapshot(snapshot);
  }

  Stream<Snap> get snapshots => _snapshotsController.stream;

  Future<void> initialize({S? initialState}) async {
    if (_isDisposed) return;

    OxideLogger.trace('OxideStore', 'Initializing engine...');
    _prepareForInitialization();

    try {
      initApp?.call();
      if (_isDisposed) return;

      final engine = await _createTrackedEngine(initialState);
      if (_isDisposed) {
        unawaited(Future<void>.value(disposeEngine(engine)));
        return;
      }

      _engine = engine;
      final initialSnap = await _track(() => current(engine));
      _recordSnapshot(initialSnap);

      if (OxideLogger.isAdvancedLoggingEnabled) {
        OxideLogger.advanced(
          'OxideStore',
          'Initial snapshot: ${_snapshotSummary(_snapshot)}',
        );
      }

      OxideLogger.debug(
        'OxideStore',
        'Engine initialized and snapshot recorded.',
      );
      if (_isDisposed) return;

      _subscription = stateStream(
        engine,
      ).listen(_handleStreamSnapshot, onError: _recordError);
    } catch (err, st) {
      _recordError(err, st);
    } finally {
      _isLoading = false;
    }
  }

  Future<void> dispose() async {
    _isDisposed = true;
    _disposeRequested = true;
    await _subscription?.cancel();
    if (_inFlight == 0) await _disposeEngine();
    await _snapshotsController.close();
  }

  Future<void> dispatchAction(A action) async {
    if (_isDisposed) return;
    final engine = _engine;
    if (engine == null) return;

    _error = null;
    _errorStackTrace = null;
    final previousSnapshot = _snapshot;

    if (OxideLogger.isAdvancedLoggingEnabled) {
      OxideLogger.advanced(
        'OxideStore',
        'Dispatch payload: before=${_snapshotSummary(previousSnapshot)} action=$action',
      );
    }

    try {
      OxideLogger.trace('OxideStore', 'Dispatching action: $action');
      final snap = await _track(() => dispatch(engine, action));
      _recordSnapshot(snap);

      OxideLogger.advancedTransition(
        'OxideStore',
        before: _snapshotSummary(previousSnapshot),
        action: action,
        after: _snapshotSummary(snap),
      );
    } catch (err, st) {
      OxideLogger.error(
        'OxideStore',
        'Error dispatching action: $action',
        err,
        st,
      );
      OxideLogger.advanced(
        'OxideStore',
        'Dispatch failure: before=${_snapshotSummary(previousSnapshot)} action=$action error=$err',
      );
      _recordError(err, st);
    }
  }

  Future<List<int>?> encodeCurrentStateBytes() async {
    final engine = _engine;
    final encode = encodeCurrentState;
    if (engine == null || encode == null) return null;
    return await _track(() => encode(engine));
  }

  Future<T> _track<T>(FutureOr<T> Function() op) async {
    _inFlight++;
    try {
      return await Future<T>.value(op());
    } finally {
      _inFlight--;
      if (_disposeRequested && _inFlight == 0) await _disposeEngine();
    }
  }

  void _prepareForInitialization() {
    _isLoading = true;
    _error = null;
    _errorStackTrace = null;
    _snapshot = null;
    _lastDeliveredRevision = null;
  }

  Future<E> _createTrackedEngine(S? initialState) async {
    return _track(() {
      _engineCreationCount++;
      return createEngine(initialState);
    });
  }

  void _handleStreamSnapshot(Snap snap) {
    if (_isDisposed) return;
    _recordSnapshot(snap);
  }

  void _recordSnapshot(Snap snap) {
    if (_isDisposed) return;
    _snapshot = snap;
    if (_shouldEmit(snap)) {
      _emitSnapshot(snap);
    }
  }

  void _recordError(Object err, StackTrace st) {
    if (_isDisposed) return;
    _error = err;
    _errorStackTrace = st;
  }

  Future<void> _disposeEngine() async {
    final engine = _engine;
    _engine = null;
    if (engine == null) return;
    await Future<void>.value(disposeEngine(engine));
  }

  bool _shouldEmit(Snap snap) {
    if (revisionOf != null) {
      final rev = revisionOf!(snap);
      if (_lastDeliveredRevision != null && _lastDeliveredRevision == rev) {
        return false;
      }
      _lastDeliveredRevision = rev;
    }
    return true;
  }

  String _snapshotSummary(Snap? snap) {
    if (snap == null) return 'null';

    final parts = <String>[];

    if (revisionOf != null) {
      try {
        final rev = revisionOf!(snap);
        parts.add('revision=$rev');
      } catch (_) {
        parts.add('revision=<unavailable>');
      }
    }

    try {
      final state = stateFromSnapshot(snap);
      parts.add('state=$state');
    } catch (_) {
      parts.add('state=<unavailable>');
    }

    return parts.join(' ');
  }

  void _emitSnapshot(Snap snap) {
    _snapshotsController.add(snap);
    _snapshotEmissionCount++;
  }
}
