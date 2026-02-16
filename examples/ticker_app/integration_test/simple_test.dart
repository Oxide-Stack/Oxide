import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

import 'package:ticker_app/main.dart';
import 'package:ticker_app/src/rust/api/bridge.dart' show initOxide;
import 'package:ticker_app/src/rust/frb_generated.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await RustLib.init();
    await initOxide();
  });

  testWidgets('Ticker stream updates are wired up', (WidgetTester tester) async {
    await tester.pumpWidget(const ProviderScope(child: MyApp()));
    for (var i = 0; i < 50 && find.text('Ticks: 0').evaluate().isEmpty; i++) {
      await tester.pump(const Duration(milliseconds: 100));
    }

    expect(find.text('Ticks: 0'), findsOneWidget);
    expect(find.text('Start'), findsOneWidget);
    expect(find.text('Reset'), findsOneWidget);

    await tester.tap(find.text('Start'));
    await tester.pump(const Duration(milliseconds: 200));
    await tester.tap(find.text('Reset'));
  });
}
