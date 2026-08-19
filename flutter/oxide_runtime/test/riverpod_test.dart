import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:oxide_runtime/riverpod.dart';

void main() {
  test('engine provider creates and disposes engine', () async {
    var disposed = false;

    final provider = oxideEngineProvider<int>(
      OxideEngineController<int>(
        create: () async => 123,
        dispose: (_) => disposed = true,
      ),
    );

    final container = ProviderContainer();
    addTearDown(container.dispose);

    final value = await container.read(provider.future);
    expect(value, 123);

    container.dispose();
    await Future<void>.delayed(Duration.zero);
    expect(disposed, true);
  });

  test('stream and future providers from engine forward values', () async {
    final provider = oxideEngineProvider<int>(
      OxideEngineController<int>(
        create: () => 10,
        dispose: (_) {},
      ),
    );

    final streamProvider = oxideStreamFromEngineProvider<int, int>(
      engineProvider: provider,
      createStream: (engine) => Stream<int>.value(engine + 1),
    );

    final futureProvider = oxideFutureFromEngineProvider<int, int>(
      engineProvider: provider,
      createFuture: (engine) async => engine * 2,
    );

    final passthroughProvider = oxideStreamProvider<String>((_) => Stream.value('ok'));

    final container = ProviderContainer();
    addTearDown(container.dispose);

    final doubled = await container.read(futureProvider.future);
    expect(doubled, 20);

    final streamEvents = <int>[];
    final streamSub = container.listen<AsyncValue<int>>(
      streamProvider,
      (_, next) => next.whenData(streamEvents.add),
      fireImmediately: true,
    );

    final passthroughEvents = <String>[];
    final passthroughSub = container.listen<AsyncValue<String>>(
      passthroughProvider,
      (_, next) => next.whenData(passthroughEvents.add),
      fireImmediately: true,
    );

    await Future<void>.delayed(const Duration(milliseconds: 10));
    expect(streamEvents, contains(11));
    expect(passthroughEvents, contains('ok'));

    streamSub.close();
    passthroughSub.close();
  });
}
