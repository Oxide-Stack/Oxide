import 'dart:async';

/// Consumes a typed Rust → Dart outgoing stream for a duplex channel.
///
/// This helper is a small convenience wrapper around `Stream.listen` that keeps
/// error handling consistent with the rest of the Oxide runtime.
StreamSubscription<Outgoing> listenOxideDuplexOutgoing<Outgoing>({
  required Stream<Outgoing> outgoing,
  required void Function(Outgoing event) onEvent,
  void Function(Object error, StackTrace stackTrace)? onError,
  void Function()? onDone,
  bool? cancelOnError,
}) {
  return outgoing.listen(
    onEvent,
    onError: onError ??
        (Object error, StackTrace stackTrace) {
          Zone.current.handleUncaughtError(error, stackTrace);
        },
    onDone: onDone,
    cancelOnError: cancelOnError,
  );
}

