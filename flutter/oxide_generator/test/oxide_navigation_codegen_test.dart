import 'package:oxide_generator/src/oxide_navigation_codegen.dart';
import 'package:test/test.dart';

void main() {
  test('generateRouteKindSource emits an enum with lowerCamel cases', () {
    final metadata = RustRouteMetadata(
      crateName: 'rust_lib_example',
      routes: [
        RustRouteMeta(
          kind: 'Splash',
          rustType: 'SplashRoute',
          path: null,
          returnType: 'oxide_core::navigation::NoReturn',
          extraType: 'oxide_core::navigation::NoExtra',
          fields: const [],
        ),
        RustRouteMeta(
          kind: 'Home',
          rustType: 'HomeRoute',
          path: null,
          returnType: 'oxide_core::navigation::NoReturn',
          extraType: 'oxide_core::navigation::NoExtra',
          fields: const [],
        ),
      ],
    );

    final src = generateRouteKindSource(metadata);
    expect(src, contains('enum RouteKind'));
    expect(src, contains('splash'));
    expect(src, contains('home'));
  });

  test('generateRouteBuildersSource stubs missing page bindings', () {
    final metadata = RustRouteMetadata(
      crateName: 'rust_lib_example',
      routes: [
        RustRouteMeta(
          kind: 'Splash',
          rustType: 'SplashRoute',
          path: null,
          returnType: 'oxide_core::navigation::NoReturn',
          extraType: 'oxide_core::navigation::NoExtra',
          fields: const [],
        ),
      ],
    );

    final src = generateRouteBuildersSource(metadata, const []);
    expect(src, contains('Missing @OxideRoutePage mapping'));
  });

  test('generateRouteBuildersSource wires page bindings with imports and route args', () {
    final metadata = RustRouteMetadata(
      crateName: 'rust_lib_example',
      routes: [
        RustRouteMeta(
          kind: 'Splash',
          rustType: 'SplashRoute',
          path: null,
          returnType: 'oxide_core::navigation::NoReturn',
          extraType: 'oxide_core::navigation::NoExtra',
          fields: const [],
        ),
      ],
    );

    final src = generateRouteBuildersSource(
      metadata,
      const [
        RoutePageBinding(
          kindKey: 'splash',
          widgetType: 'SplashScreen',
          libraryUri: 'package:example/splash_screen.dart',
        ),
      ],
    );

    expect(src, contains("import 'package:example/splash_screen.dart';"));
    expect(src, contains('final r = route as SplashRoute;'));
    expect(src, contains('return SplashScreen(route: r);'));
  });
}
