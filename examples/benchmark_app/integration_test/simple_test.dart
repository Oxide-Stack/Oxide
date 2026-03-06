import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:benchmark_app/src/bench/bench_screen.dart';
import 'package:benchmark_app/oxide.dart';
import 'package:benchmark_app/src/oxide.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async {
    await OxideStack.init();
  });

  testWidgets('App boots and renders dashboard', (WidgetTester tester) async {
    await tester.pumpWidget(ProviderScope(
      child: BenchCounterHooksOxideScope(
        child: BenchJsonHooksOxideScope(
          child: BenchSieveHooksOxideScope(
            child: MaterialApp(home: BenchHomeScreen(route: const HomeRoute())),
          ),
        ),
      ),
    ));
    await tester.pumpAndSettle();

    expect(find.text('Benchmark Dashboard'), findsOneWidget);
    expect(find.text('Workload'), findsOneWidget);
    expect(find.text('Run'), findsOneWidget);
  });
}
