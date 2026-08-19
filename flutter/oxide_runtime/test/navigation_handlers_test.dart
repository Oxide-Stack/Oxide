import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:go_router/go_router.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

enum _RouteKind { home, details, settings }

final class _RouteModel {
  const _RouteModel(this.kind, this.payload);

  final _RouteKind kind;
  final String payload;
}

Widget _routeText(String value) => Scaffold(body: Text(value, textDirection: TextDirection.ltr));

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  group('NavigatorNavigationHandler', () {
    testWidgets('push/pop/popUntil/reset work with mounted navigator', (tester) async {
      final key = GlobalKey<NavigatorState>();
      final handler = NavigatorNavigationHandler<_RouteModel, _RouteKind>(
        navigatorKey: key,
        kindOf: (route) => route.kind,
        routeBuilders: {
          _RouteKind.home: (context, route) => _routeText('home:${route.payload}'),
          _RouteKind.details: (context, route) => _routeText('details:${route.payload}'),
          _RouteKind.settings: (context, route) => _routeText('settings:${route.payload}'),
        },
      );

      await tester.pumpWidget(MaterialApp(navigatorKey: key, home: _routeText('root')));
      await tester.pumpAndSettle();

      unawaited(handler.push(const _RouteModel(_RouteKind.home, 'A')));
      await tester.pumpAndSettle();
      expect(find.text('home:A'), findsOneWidget);

      unawaited(handler.push(const _RouteModel(_RouteKind.details, 'B')));
      await tester.pumpAndSettle();
      expect(find.text('details:B'), findsOneWidget);

      handler.popUntil(_RouteKind.home);
      await tester.pumpAndSettle();
      expect(find.text('home:A'), findsOneWidget);

      unawaited(handler.reset(const [_RouteModel(_RouteKind.settings, 'C')]));
      await tester.pumpAndSettle();
      expect(find.text('settings:C'), findsOneWidget);

      handler.pop('done');
      await tester.pumpAndSettle();

      await handler.reset(const []);
      await tester.pumpAndSettle();
      expect(find.text('settings:C'), findsNothing);
    });

    testWidgets('missing builder throws', (tester) async {
      final key = GlobalKey<NavigatorState>();
      final handler = NavigatorNavigationHandler<_RouteModel, _RouteKind>(
        navigatorKey: key,
        kindOf: (route) => route.kind,
        routeBuilders: {
          _RouteKind.home: (context, route) => _routeText(route.payload),
        },
      );

      await tester.pumpWidget(MaterialApp(navigatorKey: key, home: _routeText('root')));
      await tester.pumpAndSettle();

      expect(
        () => handler.push(const _RouteModel(_RouteKind.details, 'missing')),
        throwsA(isA<StateError>()),
      );

      expect(
        () => handler.reset(const [_RouteModel(_RouteKind.details, 'missing')]),
        throwsA(isA<StateError>()),
      );
    });

    testWidgets('reset can progress through multiple routes and timeout fallback reports errors', (tester) async {
      final key = GlobalKey<NavigatorState>();
      final handler = NavigatorNavigationHandler<_RouteModel, _RouteKind>(
        navigatorKey: key,
        kindOf: (route) => route.kind,
        routeBuilders: {
          _RouteKind.home: (context, route) => _routeText('home:${route.payload}'),
          _RouteKind.details: (context, route) => _routeText('details:${route.payload}'),
        },
      );

      await tester.pumpWidget(MaterialApp(navigatorKey: key, home: _routeText('root')));
      await tester.pumpAndSettle();

      unawaited(handler.reset(const [
        _RouteModel(_RouteKind.home, 'A'),
        _RouteModel(_RouteKind.details, 'B'),
      ]));
      await tester.pumpAndSettle();
      expect(find.text('home:A'), findsOneWidget);

      handler.pop('first');
      await tester.pumpAndSettle();
      expect(find.text('details:B'), findsOneWidget);

      handler.pop('second');
      await tester.pumpAndSettle();

      final missingNavigator = NavigatorNavigationHandler<_RouteModel, _RouteKind>(
        navigatorKey: GlobalKey<NavigatorState>(),
        kindOf: (route) => route.kind,
        routeBuilders: {
          _RouteKind.home: (context, route) => _routeText(route.payload),
        },
        navigatorWaitTimeout: const Duration(milliseconds: 1),
      );

      final uncaught = <Object>[];
      await runZonedGuarded(() async {
        final timeoutPush = missingNavigator.push(const _RouteModel(_RouteKind.home, 'x'));
        missingNavigator.pop('ignored');
        missingNavigator.popUntil(_RouteKind.home);
        unawaited(missingNavigator.reset(const [_RouteModel(_RouteKind.home, 'z')]));

        for (var i = 0; i < 6; i++) {
          await tester.pump(const Duration(milliseconds: 2));
        }

        await expectLater(timeoutPush, throwsA(isA<StateError>()));
      }, (error, _) {
        uncaught.add(error);
      });

      expect(uncaught, isNotEmpty);
      expect(uncaught.every((error) => error is StateError), isTrue);
    });

    testWidgets('deferred navigator availability executes fallback branches', (tester) async {
      final key = GlobalKey<NavigatorState>();
      final handler = NavigatorNavigationHandler<_RouteModel, _RouteKind>(
        navigatorKey: key,
        kindOf: (route) => route.kind,
        routeBuilders: {
          _RouteKind.home: (context, route) => _routeText('home:${route.payload}'),
        },
      );

      handler.pop('early-pop');
      handler.popUntil(_RouteKind.home);
      unawaited(handler.reset(const []));
      unawaited(handler.reset(const [_RouteModel(_RouteKind.home, 'late')]));
      handler.setCurrentRoute(const _RouteModel(_RouteKind.home, 'noop'));

      await tester.pump(const Duration(milliseconds: 1));
      await tester.pumpWidget(MaterialApp(navigatorKey: key, home: _routeText('root')));
      for (var i = 0; i < 6; i++) {
        await tester.pump(const Duration(milliseconds: 2));
      }

      handler.pop('complete-late');
      await tester.pump(const Duration(milliseconds: 4));
    });
  });

  group('GoRouterNavigationHandler', () {
    testWidgets('push/pop/popUntil/reset and setCurrentRoute work', (tester) async {
      final routeState = ValueNotifier<String>('none');
      final router = GoRouter(
        initialLocation: '/home',
        routes: [
          GoRoute(path: '/home', builder: (context, state) => _routeText('home')),
          GoRoute(path: '/details', builder: (context, state) => _routeText('details')),
          GoRoute(path: '/settings', builder: (context, state) => _routeText('settings')),
        ],
      );
      addTearDown(router.dispose);

      final handler = GoRouterNavigationHandler<_RouteModel, _RouteKind>(
        router: router,
        kindOf: (route) {
          routeState.value = route.kind.name;
          return route.kind;
        },
        locationOf: (route) => '/${route.kind.name}',
        locationOfKind: (kind) => '/${kind.name}',
      );

      await tester.pumpWidget(MaterialApp.router(routerConfig: router));
      await tester.pumpAndSettle();

      unawaited(handler.push(const _RouteModel(_RouteKind.details, 'x')));
      await tester.pumpAndSettle();

      // no-op path when router cannot pop
      handler.pop('no-op');
      await tester.pump(const Duration(milliseconds: 1));

      handler.pop('done');
      await tester.pumpAndSettle();

      unawaited(handler.push(const _RouteModel(_RouteKind.details, 'x')));
      await tester.pumpAndSettle();
      unawaited(handler.push(const _RouteModel(_RouteKind.settings, 'y')));
      await tester.pumpAndSettle();
      handler.popUntil(_RouteKind.home);
      await tester.pumpAndSettle();

      // No pop path should fall back to go(target).
      handler.popUntil(_RouteKind.settings);
      await tester.pumpAndSettle();

      await handler.reset(const [_RouteModel(_RouteKind.details, 'b')]);
      await tester.pumpAndSettle();

      unawaited(handler.reset(const [
        _RouteModel(_RouteKind.home, 'a'),
        _RouteModel(_RouteKind.details, 'b'),
      ]));
      await tester.pumpAndSettle();
      handler.pop('finish-multi-reset');
      await tester.pumpAndSettle();

      await handler.reset(const []);
      await tester.pumpAndSettle();

      handler.setCurrentRoute(const _RouteModel(_RouteKind.settings, 's'));
      expect(routeState.value, 'settings');
    });
  });
}