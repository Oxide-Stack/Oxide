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
}

final class _Envelope<T> {
  _Envelope({required this.id, required this.request});

  final int id;
  final T request;
}
