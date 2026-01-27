import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:todos_app/main.dart';
import 'package:todos_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());

  testWidgets('Todo CRUD updates Rust state', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    await tester.pumpAndSettle();

    await tester.enterText(
      find.byWidgetPredicate((w) {
        if (w is! TextField) return false;
        return w.decoration?.labelText == 'Add Todo Title (must be non-empty)';
      }),
      'buy milk',
    );
    await tester.tap(find.text('Add Todo'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('buy milk'), findsOneWidget);
  });
}
