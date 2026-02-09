import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:counter_app/main.dart';
import 'package:counter_app/src/rust/api/bridge.dart' show initOxide;
import 'package:counter_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
    await initOxide();
  });

  testWidgets('Counter dispatch updates Rust state', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: CounterHomeScreen())));
    await tester.pumpAndSettle(const Duration(seconds: 1));

    expect(find.text('Counter: 0'), findsOneWidget);

    await tester.tap(find.text('Increment'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('Counter: 1'), findsOneWidget);

    await tester.tap(find.text('Reset'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('Counter: 0'), findsOneWidget);
  });
}
