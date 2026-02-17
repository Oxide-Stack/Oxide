import 'dart:async';

/// Runs a Rust → Dart → Rust callbacking loop for a single callback service.
///
/// This helper is intended to be used with FRB-generated bindings produced by
/// `#[oxide_callback]` on the Rust side.
///
/// The Rust macro emits a typed request stream where each item contains:
/// - a numeric request id
/// - a strongly typed request enum value
///
/// Dart is responsible for:
/// - handling the request (often via UI logic)
/// - sending the corresponding response enum value back to Rust, using the same id
///
/// The locked binding rule is enforced on the Rust side by variant name. This
/// function does not attempt to perform any dynamic routing.
Future<void> runOxideCallbacking<Envelope, Request, Response>({
  required Stream<Envelope> requests,
  required int Function(Envelope envelope) requestIdOf,
  required Request Function(Envelope envelope) requestOf,
  required FutureOr<Response> Function(Request request) handler,
  required FutureOr<void> Function(int requestId, Response response) respond,
  void Function(Object error, StackTrace stackTrace)? onError,
}) async {
  await for (final envelope in requests) {
    try {
      final id = requestIdOf(envelope);
      final request = requestOf(envelope);
      final response = await handler(request);
      await respond(id, response);
    } catch (error, stackTrace) {
      if (onError != null) {
        onError(error, stackTrace);
      } else {
        Zone.current.handleUncaughtError(error, stackTrace);
      }
    }
  }
}

