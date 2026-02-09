import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:todos_app/main.dart';
import 'package:todos_app/src/rust/api/bridge.dart' show initOxide;
import 'package:todos_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
    await initOxide();
  });

  testWidgets('Todo CRUD updates Rust state', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MaterialApp(home: TodosHomeScreen())));
    await tester.pump(const Duration(milliseconds: 250));

    final input = find.byType(TextField);
    expect(input, findsWidgets);
    await tester.enterText(input.first, 'buy milk');
    await tester.tap(find.text('Add Todo'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('buy milk'), findsWidgets);
  });
}
