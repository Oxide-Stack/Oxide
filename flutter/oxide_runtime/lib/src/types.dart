// Public callback and transformer typedefs used by the Oxide runtime core.
//
// Why: generated bindings can vary (FRB, mocks, alternative backends), but the
// runtime core needs a stable, strongly-typed contract for wiring them in.
import 'dart:async';

/// Creates an engine instance, optionally from an initial state.
///
/// This callback is typically implemented by generated FFI bindings.
typedef OxideCreateEngine<E, S> = FutureOr<E> Function(S? initialState);

/// Disposes an engine instance.
///
/// Implementations should release native resources and cancel background work.
typedef OxideDisposeEngine<E> = FutureOr<void> Function(E engine);

/// Dispatches an action to the engine and returns an updated snapshot.
typedef OxideDispatch<E, A, Snap> = Future<Snap> Function(E engine, A action);

/// Returns the current snapshot from the engine.
typedef OxideCurrent<E, Snap> = Future<Snap> Function(E engine);

/// Returns a stream of snapshots emitted by the engine.
typedef OxideStateStream<E, Snap> = Stream<Snap> Function(E engine);

/// One-time application initialization hook.
///
/// This is typically used for lightweight per-library initialization in the
/// bindings layer.
///
/// In Flutter Rust Bridge (FRB) apps, you should still call `RustLib.init()` and
/// the app's `initOxide()` during `main()`; this hook is not a substitute for
/// that required runtime initialization.
typedef OxideInitApp = void Function();

/// Extracts the state value from a snapshot.
typedef OxideStateFromSnapshot<S, Snap> = S Function(Snap snapshot);

/// Encodes the current engine state to bytes suitable for persistence.
typedef OxideEncodeCurrentState<E> = Future<List<int>> Function(E engine);
