// Engine lifecycle and snapshot coordination for generated stores.
//
// application state management layers (Inherited/Riverpod/BLoC) need a
// single, backend-agnostic core that owns engine creation, dispatch, streaming,
// and error capture.
import 'dart:async';

import 'logger.dart';
import 'types.dart';

/// Core runtime used by generated store wrappers.
///
/// `OxideStoreCore` coordinates engine lifecycle, dispatching, snapshot
/// subscription, and error tracking. It is intended to be driven by codegen and
/// used by application-level state management layers.
final class OxideStoreCore<S, A, E, Snap> {
  /// Creates a new core instance.
  ///
  /// Most callbacks are required because they are engine-specific and are
  /// provided by generated bindings.
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

  /// Creates the engine.
  final OxideCreateEngine<E, S> createEngine;

  /// Disposes the engine.
  final OxideDisposeEngine<E> disposeEngine;

  /// Dispatches an action and yields a snapshot.
  final OxideDispatch<E, A, Snap> dispatch;

  /// Reads the current snapshot.
  final OxideCurrent<E, Snap> current;

  /// Subscribes to the engine's snapshot stream.
  final OxideStateStream<E, Snap> stateStream;

  /// Converts a snapshot into a state value.
  final OxideStateFromSnapshot<S, Snap> stateFromSnapshot;

  /// Optional one-time initialization hook.
  ///
  /// If set, this hook is called at the start of [initialize]. It is invoked
  /// inside a try/catch and any thrown error is captured into [error].
  final OxideInitApp? initApp;

  /// Optional encoder for the engine's current state.
  final OxideEncodeCurrentState<E>? encodeCurrentState;

  /// Extracts revision number from a snapshot for deduplication.
  ///
  /// Generated bindings should supply this to allow the core to drop
  /// duplicate snapshots that share the same revision.
  final int Function(Snap snap)? revisionOf;

  E? _engine;
  StreamSubscription<Snap>? _subscription;
  Snap? _snapshot;
  final StreamController<Snap> _snapshotsController = StreamController<Snap>.broadcast();

  // instrumentation counters (debug only)
  int _engineCreationCount = 0;
  int _snapshotEmissionCount = 0;

  bool _isDisposed = false;
  bool _disposeRequested = false;
  int _inFlight = 0;

  bool _isLoading = true;
  Object? _error;
  StackTrace? _errorStackTrace;

  /// last revision that was delivered to listeners, if known.
  int? _lastDeliveredRevision;

  /// Whether the store is currently initializing.
  bool get isLoading => _isLoading;

  /// The most recent error captured by the core runtime, if any.
  Object? get error => _error;

  /// Stack trace associated with [error], if available.
  StackTrace? get errorStackTrace => _errorStackTrace;

  /// The current engine instance, if initialized.
  E? get engine => _engine;

  /// The most recent snapshot received from the engine, if any.
  Snap? get snapshot => _snapshot;

  /// Number of times the engine was created.
  int get engineCreationCount => _engineCreationCount;

  /// Number of snapshots emitted through [snapshots] stream.
  int get snapshotEmissionCount => _snapshotEmissionCount;

  /// The derived state value from [snapshot], if available.
  S? get state {
    final snapshot = _snapshot;
    if (snapshot == null) return null;
    return stateFromSnapshot(snapshot);
  }

  /// Broadcast stream of snapshots.
  ///
  /// This stream emits the initial snapshot (once available) and then forwards
  /// updates from the underlying engine's stream.
  Stream<Snap> get snapshots => _snapshotsController.stream;

  /// Initializes the engine and starts listening for snapshots.
  ///
  /// Errors thrown by engine callbacks are captured into [error] and
  /// [errorStackTrace]. They are not rethrown.
  ///
  /// # Returns
  /// A future that completes once the core is initialized (successfully or with
  /// an error recorded).
  Future<void> initialize({S? initialState}) async {
    OxideLogger.trace('OxideStore', 'Initializing engine...');
    _isLoading = true;
    _error = null;
    _errorStackTrace = null;

    try {
      initApp?.call();
      if (_isDisposed) return;

      final engine = await _track(() {
        _engineCreationCount++;
        return createEngine(initialState);
      });
      if (_isDisposed) {
        unawaited(Future<void>.value(disposeEngine(engine)));
        return;
      }

      _engine = engine;
      _snapshot = await _track(() => current(engine));
      final initialSnap = _snapshot;
      if (initialSnap != null) {
        if (!_shouldEmit(initialSnap)) {
          // drop duplicate initial snapshot
        } else {
          _emitSnapshot(initialSnap);
        }
      }

      if (OxideLogger.isAdvancedLoggingEnabled) {
        OxideLogger.advanced('OxideStore', 'Initial snapshot: ${_snapshotSummary(initialSnap)}');
      }

      OxideLogger.debug('OxideStore', 'Engine initialized and snapshot recorded.');
      if (_isDisposed) return;

      _subscription = stateStream(engine).listen(
        (snap) {
          if (_isDisposed) return;
          _snapshot = snap;
          if (_shouldEmit(snap)) {
            _emitSnapshot(snap);
          }
        },
        onError: (Object err, StackTrace st) {
          if (_isDisposed) return;
          _error = err;
          _errorStackTrace = st;
        },
      );
    } catch (err, st) {
      _error = err;
      _errorStackTrace = st;
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

  /// Dispatches an action to the engine and updates [snapshot].
  ///
  /// Any error thrown by the underlying dispatch is captured into [error] and
  /// [errorStackTrace].
  Future<void> dispatchAction(A action) async {
    if (_isDisposed) return;
    final engine = _engine;
    if (engine == null) return;

    _error = null;
    _errorStackTrace = null;
    final previousSnapshot = _snapshot;

    if (OxideLogger.isAdvancedLoggingEnabled) {
      OxideLogger.advanced('OxideStore', 'Dispatch payload: before=${_snapshotSummary(previousSnapshot)} action=$action');
    }

    try {
      OxideLogger.trace('OxideStore', 'Dispatching action: $action');
      _snapshot = await _track(() => dispatch(engine, action));
      final snap = _snapshot;
      if (snap != null) {
        if (_shouldEmit(snap)) {
          _emitSnapshot(snap);
        }
      }

      OxideLogger.advancedTransition('OxideStore', before: _snapshotSummary(previousSnapshot), action: action, after: _snapshotSummary(snap));
    } catch (err, st) {
      OxideLogger.error('OxideStore', 'Error dispatching action: $action', err, st);
      OxideLogger.advanced('OxideStore', 'Dispatch failure: before=${_snapshotSummary(previousSnapshot)} action=$action error=$err');
      _error = err;
      _errorStackTrace = st;
    }
  }

  /// Encodes the current state to bytes, if [encodeCurrentState] is provided.
  ///
  /// # Returns
  /// The encoded bytes, or `null` if the engine is not initialized or encoding
  /// is not supported.
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
