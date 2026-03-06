import 'dart:async';

import 'package:flutter_test/flutter_test.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

final class _TestHandler implements OxideNavigationHandler<String, String> {
  final pushed = <String>[];
  final popped = <Object?>[];
  final popUntils = <String>[];
  final resets = <List<String>>[];
  String? current;
  final pending = <String, Completer<Object?>>{};

  @override
  Future<Object?> push(String route, {String? ticket}) async {
    pushed.add(route);
    current = route;
    final completer = Completer<Object?>();
    pending[route] = completer;
    return completer.future;
  }

  @override
  void pop([Object? result]) {
    popped.add(result);
  }

  @override
  void popUntil(String kind) {
    popUntils.add(kind);
  }

  @override
  void reset(List<String> routes) {
    resets.add(List<String>.from(routes));
    current = routes.isEmpty ? null : routes.last;
  }

  @override
  void setCurrentRoute(String route) {
    current = route;
  }

  void completePush(String route, [Object? result]) {
    pending.remove(route)?.complete(result);
  }
}

void main() {
  test('push/pop/reset update state deterministically', () async {
    final controller = StreamController<OxideNavigationCommand<String, String>>();
    final handler = _TestHandler();
    final currentRoutes = <String?>[];

    final runtime = OxideNavigationRuntime<String, String>(
      commands: controller.stream,
      handler: handler,
      emitResult: (_, __) async {},
      setCurrentRoute: (r) async => currentRoutes.add(r),
      kindOf: (r) => r,
    );

    runtime.start();

    controller.add(OxideNavigationCommand.push(route: 'A', ticket: null));
    await Future<void>.delayed(const Duration(milliseconds: 10));
    expect(runtime.state.value.current, 'A');
    expect(runtime.state.value.stack, ['A']);

    controller.add(OxideNavigationCommand.push(route: 'B', ticket: null));
    await Future<void>.delayed(const Duration(milliseconds: 10));
    expect(runtime.state.value.current, 'B');
    expect(runtime.state.value.stack, ['A', 'B']);

    controller.add(OxideNavigationCommand.pop(result: 123));
    await Future<void>.delayed(const Duration(milliseconds: 10));
    expect(runtime.state.value.current, 'A');
    expect(runtime.state.value.stack, ['A']);
    handler.completePush('B', 123);

    controller.add(OxideNavigationCommand.reset(routes: ['X', 'Y']));
    await Future<void>.delayed(const Duration(milliseconds: 10));
    expect(runtime.state.value.current, 'Y');
    expect(runtime.state.value.stack, ['X', 'Y']);
    handler.completePush('A');

    await runtime.stop();
    expect(runtime.state.value.current, isNull);
    expect(runtime.state.value.stack, isEmpty);
    expect(currentRoutes.last, isNull);

    await controller.close();
    await runtime.dispose();
  });

  test('dispose prevents restart', () async {
    final controller = StreamController<OxideNavigationCommand<String, String>>();
    final handler = _TestHandler();

    final runtime = OxideNavigationRuntime<String, String>(
      commands: controller.stream,
      handler: handler,
      emitResult: (_, __) async {},
      setCurrentRoute: (_) async {},
      kindOf: (r) => r,
    );

    runtime.start();
    await runtime.dispose();

    expect(runtime.start, throwsStateError);

    await controller.close();
  });
}
