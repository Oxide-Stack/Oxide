import 'dart:async';

import 'package:flutter_test/flutter_test.dart';

import 'package:oxide_runtime/src/isolated_channels/callback_runtime.dart';
import 'package:oxide_runtime/src/isolated_channels/duplex_runtime.dart';

void main() {
  test('runOxideCallbacking invokes handler and responds', () async {
    final requests = StreamController<_Envelope<int>>();
    final responses = <int, int>{};

    final loop = runOxideCallbacking<_Envelope<int>, int, int>(
      requests: requests.stream,
      requestIdOf: (e) => e.id,
      requestOf: (e) => e.request,
      handler: (req) => req + 1,
      respond: (id, resp) => responses[id] = resp,
    );

    requests.add(_Envelope(id: 7, request: 41));
    await requests.close();

    await loop;
    expect(responses[7], 42);
  });

  test('listenOxideDuplexOutgoing forwards events', () async {
    final outgoing = StreamController<int>();
    final events = <int>[];
    final done = Completer<void>();

    final sub = listenOxideDuplexOutgoing<int>(
      outgoing: outgoing.stream,
      onEvent: events.add,
      onDone: done.complete,
    );

    outgoing.add(1);
    outgoing.add(2);
    await outgoing.close();
    await done.future;
    await sub.cancel();

    expect(events, [1, 2]);
  });

  test('runOxideCallbacking routes handler errors to onError', () async {
    final requests = StreamController<_Envelope<int>>();
    final errors = <Object>[];

    final loop = runOxideCallbacking<_Envelope<int>, int, int>(
      requests: requests.stream,
      requestIdOf: (e) => e.id,
      requestOf: (e) => e.request,
      handler: (_) => throw StateError('boom'),
      respond: (_, __) {},
      onError: (error, _) => errors.add(error),
    );

    requests.add(_Envelope(id: 1, request: 10));
    await requests.close();
    await loop;

    expect(errors.single, isA<StateError>());
  });

  test('listenOxideDuplexOutgoing default onError uses current zone', () async {
    final outgoing = StreamController<int>();
    final uncaught = <Object>[];

    await runZonedGuarded(() async {
      final sub = listenOxideDuplexOutgoing<int>(
        outgoing: outgoing.stream,
        onEvent: (_) {},
      );
      outgoing.addError(StateError('stream-boom'));
      await outgoing.close();
      await Future<void>.delayed(Duration.zero);
      await sub.cancel();
    }, (error, _) {
      uncaught.add(error);
    });

    expect(uncaught.single, isA<StateError>());
  });

  test('runOxideCallbacking default onError uses current zone', () async {
    final requests = StreamController<_Envelope<int>>();
    final uncaught = <Object>[];

    await runZonedGuarded(() async {
      final loop = runOxideCallbacking<_Envelope<int>, int, int>(
        requests: requests.stream,
        requestIdOf: (e) => e.id,
        requestOf: (e) => e.request,
        handler: (_) => throw StateError('callback-boom'),
        respond: (_, __) {},
      );

      requests.add(_Envelope(id: 3, request: 7));
      await requests.close();
      await loop;
      await Future<void>.delayed(Duration.zero);
    }, (error, _) {
      uncaught.add(error);
    });

    expect(uncaught.single, isA<StateError>());
  });
}

final class _Envelope<T> {
  _Envelope({required this.id, required this.request});

  final int id;
  final T request;
}
