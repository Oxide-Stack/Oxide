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
}
