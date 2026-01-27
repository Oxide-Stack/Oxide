import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Creates an engine instance for Riverpod-managed lifecycles.
typedef EngineCreate<E> = FutureOr<E> Function();

/// Disposes an engine instance when a provider is disposed.
typedef EngineDispose<E> = FutureOr<void> Function(E engine);

/// Configuration for creating and disposing an Oxide engine in Riverpod.
final class OxideEngineController<E> {
  /// Creates a controller from the required callbacks.
  OxideEngineController({required this.create, required this.dispose});

  /// Creates a new engine instance.
  final EngineCreate<E> create;

  /// Disposes an existing engine instance.
  final EngineDispose<E> dispose;
}

/// Creates a [FutureProvider] that owns an Oxide engine.
///
/// The engine is created the first time the provider is read, and disposed when
/// the provider is destroyed.
FutureProvider<E> oxideEngineProvider<E>(OxideEngineController<E> controller) {
  return FutureProvider<E>((ref) async {
    final engine = await controller.create();
    ref.onDispose(() {
      unawaited(Future(() => controller.dispose(engine)));
    });
    return engine;
  });
}

/// Creates a [StreamProvider] from an arbitrary stream factory.
StreamProvider<T> oxideStreamProvider<T>(Stream<T> Function(Ref ref) createStream) {
  return StreamProvider<T>((ref) => createStream(ref));
}

/// Creates a [StreamProvider] whose stream depends on a created engine.
StreamProvider<T> oxideStreamFromEngineProvider<E, T>({
  required FutureProvider<E> engineProvider,
  required Stream<T> Function(E engine) createStream,
}) {
  return StreamProvider<T>((ref) async* {
    final engine = await ref.watch(engineProvider.future);
    yield* createStream(engine);
  });
}

/// Creates a [FutureProvider] whose future depends on a created engine.
FutureProvider<T> oxideFutureFromEngineProvider<E, T>({
  required FutureProvider<E> engineProvider,
  required Future<T> Function(E engine) createFuture,
}) {
  return FutureProvider<T>((ref) async {
    final engine = await ref.watch(engineProvider.future);
    return await createFuture(engine);
  });
}
