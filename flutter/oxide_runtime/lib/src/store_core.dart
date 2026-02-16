// Engine lifecycle and snapshot coordination for generated stores.
//
// Why: application state management layers (Inherited/Riverpod/BLoC) need a
// single, backend-agnostic core that owns engine creation, dispatch, streaming,
// and error capture.
import 'dart:async';

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

  E? _engine;
  StreamSubscription<Snap>? _subscription;
  Snap? _snapshot;
  final StreamController<Snap> _snapshotsController = StreamController<Snap>.broadcast();

  bool _isDisposed = false;
  bool _disposeRequested = false;
  int _inFlight = 0;

  bool _isLoading = true;
  Object? _error;
  StackTrace? _errorStackTrace;

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
    _isLoading = true;
    _error = null;
    _errorStackTrace = null;

    try {
      initApp?.call();
      if (_isDisposed) return;

      final engine = await _track(() => createEngine(initialState));
      if (_isDisposed) {
        unawaited(Future<void>.value(disposeEngine(engine)));
        return;
      }

      _engine = engine;
      _snapshot = await _track(() => current(engine));
      final initialSnap = _snapshot;
      if (initialSnap != null) _snapshotsController.add(initialSnap);
      if (_isDisposed) return;

      _subscription = stateStream(engine).listen(
        (snap) {
          if (_isDisposed) return;
          _snapshot = snap;
          _snapshotsController.add(snap);
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

    try {
      _snapshot = await _track(() => dispatch(engine, action));
      final snap = _snapshot;
      if (snap != null) _snapshotsController.add(snap);
    } catch (err, st) {
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
}
