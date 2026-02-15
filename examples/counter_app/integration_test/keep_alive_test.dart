import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:counter_app/src/oxide.dart';
import 'package:counter_app/src/rust/api/bridge.dart' show initOxide;
import 'package:counter_app/src/rust/frb_generated.dart';

Future<void> _pumpUntil(
  WidgetTester tester, {
  required bool Function() condition,
  Duration step = const Duration(milliseconds: 50),
  Duration timeout = const Duration(seconds: 10),
}) async {
  final deadline = DateTime.now().add(timeout);
  while (!condition()) {
    if (DateTime.now().isAfter(deadline)) {
      throw StateError('Timed out waiting for condition');
    }
    await tester.pump(step);
  }
}

Future<void> _waitUntil(
  bool Function() condition, {
  Duration step = const Duration(milliseconds: 50),
  Duration timeout = const Duration(seconds: 10),
}) async {
  final deadline = DateTime.now().add(timeout);
  while (!condition()) {
    if (DateTime.now().isAfter(deadline)) {
      throw StateError('Timed out waiting for condition');
    }
    await Future<void>.delayed(step);
  }
}

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await RustLib.init();
    await initOxide();
  });

  testWidgets(
    'Inherited controller survives widget interruptions when reused',
    (WidgetTester tester) async {
      final controller = StateBridgeOxideController();
      addTearDown(controller.dispose);

      Widget buildWithScope() {
        return MaterialApp(
          home: StateBridgeOxideScope(
            controller: controller,
            child: AnimatedBuilder(
              animation: controller,
              builder: (context, _) {
                final view = StateBridgeOxideScope.useOxide(context);
                final counter = view.state?.counter;
                return Text(counter?.toString() ?? '-');
              },
            ),
          ),
        );
      }

      await tester.pumpWidget(buildWithScope());
      await _pumpUntil(tester, condition: () => !controller.isLoading && controller.state != null);

      await controller.actions.increment();
      await _pumpUntil(tester, condition: () => controller.state?.counter == BigInt.one);

      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();

      await tester.pumpWidget(buildWithScope());
      await _pumpUntil(tester, condition: () => controller.state != null);
      expect(find.text('1'), findsOneWidget);
    },
    timeout: const Timeout(Duration(minutes: 30)),
  );

  testWidgets(
    'Riverpod provider keeps state without listeners when keepAlive is true',
    (WidgetTester tester) async {
      final container = ProviderContainer();
      addTearDown(container.dispose);

      final sub = container.listen(stateBridgeRiverpodOxideProvider, (prev, next) {}, fireImmediately: true);
      addTearDown(sub.close);

      await _waitUntil(() {
        final view = container.read(stateBridgeRiverpodOxideProvider);
        return !view.isLoading && view.state != null;
      });

      await container.read(stateBridgeRiverpodOxideProvider).actions.increment();

      await _waitUntil(() {
        final view = container.read(stateBridgeRiverpodOxideProvider);
        return view.state?.counter == BigInt.one;
      });

      sub.close();

      final viewAfterUnlisten = container.read(stateBridgeRiverpodOxideProvider);
      expect(viewAfterUnlisten.state?.counter, BigInt.one);

      final sub2 = container.listen(stateBridgeRiverpodOxideProvider, (prev, next) {}, fireImmediately: true);
      addTearDown(sub2.close);
      expect(container.read(stateBridgeRiverpodOxideProvider).state?.counter, BigInt.one);

      await tester.pump();
    },
    timeout: const Timeout(Duration(minutes: 30)),
  );
}
