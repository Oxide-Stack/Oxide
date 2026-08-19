import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:showcase_app/app.dart';
import 'package:showcase_app/oxide.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  testWidgets('Showcase app boots and renders Rust-pushed navigation', (
    WidgetTester tester,
  ) async {
    await runOxideApp(const ProviderScope(child: MyApp()));

    // The splash screen animates indefinitely, so pump bounded real-time
    // frames instead of pumpAndSettle and wait for the Rust-pushed route.
    var foundRoute = false;
    for (var i = 0; i < 60; i++) {
      await tester.pump(const Duration(milliseconds: 200));
      if (find.text('Backend Matrix').evaluate().isNotEmpty) {
        foundRoute = true;
        break;
      }
    }

    expect(foundRoute, isTrue, reason: 'Rust-pushed route never rendered');
    expect(find.text('Backend Matrix'), findsOneWidget);
    expect(find.text('Riverpod'), findsOneWidget);
  });
}
