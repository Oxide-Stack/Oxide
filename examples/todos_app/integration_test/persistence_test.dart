import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:todos_app/main.dart';
import 'package:todos_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  Future<void> deletePersistenceFiles() async {
    final tempDir = Directory.systemTemp.path;
    final sep = Platform.pathSeparator;
    const key = 'oxide.todos.state.v1';
    final dir =
        '$tempDir$sep'
        'oxide';
    final binPath =
        '$dir$sep$key'
        '.bin';
    final jsonPath =
        '$dir$sep$key'
        '.json';
    final binFile = File(binPath);
    final jsonFile = File(jsonPath);
    if (await binFile.exists()) await binFile.delete();
    if (await jsonFile.exists()) await jsonFile.delete();
  }

  setUpAll(() async {
    await deletePersistenceFiles();
    await RustLib.init();
  });

  tearDownAll(() async {
    await deletePersistenceFiles();
  });

  testWidgets('State persists across widget re-mounts', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    await tester.pumpAndSettle();

    await tester.enterText(
      find.byWidgetPredicate((w) {
        if (w is! TextField) return false;
        return w.decoration?.labelText == 'Add Todo Title (must be non-empty)';
      }),
      'persist me',
    );
    await tester.tap(find.text('Add Todo'));
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('persist me'), findsOneWidget);

    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
    await tester.pump(const Duration(milliseconds: 250));

    await tester.pumpWidget(const MyApp());
    await tester.pumpAndSettle(const Duration(seconds: 1));
    expect(find.text('persist me'), findsOneWidget);
  });
}
