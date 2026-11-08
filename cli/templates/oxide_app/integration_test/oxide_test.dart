import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:{{app_name}}/oxide.dart';
import 'package:{{app_name}}/src/app.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Counter dispatch updates Rust state', (WidgetTester tester) async {
    // Wrap the real app rather than a bare MaterialApp: navigation starts when
    // runOxideApp is called, and it needs the navigatorKey that MyApp supplies.
    await runOxideApp(const MyApp());
    await tester.pumpAndSettle(const Duration(seconds: 1));

    // Must match the label rendered by the scaffolded counter page.
    expect(find.text('Counter: 0'), findsOneWidget);

    await tester.tap(find.text('Increment'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('Counter: 1'), findsOneWidget);

    await tester.tap(find.text('Reset'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('Counter: 0'), findsOneWidget);
  });
}
