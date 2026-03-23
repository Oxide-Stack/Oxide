import 'dart:io';
import 'package:oxide_generator/src/oxide_navigation_codegen.dart';
import 'package:oxide_generator/src/codegen/model.dart';
import 'package:oxide_generator/src/codegen/core_instantiation.dart';
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

    final src = generateRouteBuildersSource(metadata, const [
      RoutePageBinding(kindKey: 'splash', widgetType: 'SplashScreen', libraryUri: 'package:example/splash_screen.dart'),
    ]);

    expect(src, contains("import 'package:example/splash_screen.dart';"));
    expect(src, contains('final r = route as SplashRoute;'));
    expect(src, contains('return SplashScreen(route: r);'));
  });

  test('generateNavigationRuntimeSource uses typed FRB navigation stream', () {
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

    final src = generateNavigationRuntimeSource(metadata);
    expect(src, contains('oxideNavCommandsStream()'));
    expect(src, isNot(contains('oxideNavCommandsJsonStream')));
    expect(src, contains('oxideNavSetCurrentRoute('));
    expect(src, isNot(contains('oxideNavSetCurrentRouteJson')));
    expect(src, contains('_mapOxideNavCommand'));
    expect(src, isNot(contains('_decodeOxideNavCommand')));
    expect(src, isNot(contains('jsonDecode(json)')));
    // debug incoming command logging must be present
    expect(src, contains('print("[Oxide] received nav command'));
    // guard variable prevents repeated runtime starts
    expect(src, contains('bool _oxideNavStarted = false'));
    expect(src, contains('if (_oxideNavStarted) return;'));
    // debug log should be emitted
    expect(src, contains("print('[Oxide] oxideNavStart called"));
    expect(src, contains('Future<void> oxideNavStart() async')); // schedules init for the next frame
    expect(src, contains('WidgetsBinding.instance.addPostFrameCallback'));
    expect(src, contains('unawaited(rust_nav.initNavigation());'));
  });

  test('generateOxideStackSource forwards to bridge APIs', () {
    final channels = RustChannelMetadata(
      initFnName: 'initIsolatedChannelsDemo',
      events: [RustEventChannelMeta(name: 'CounterDemoEvents', eventType: 'CounterDemoEvent')],
      callbacks: [RustCallbackMeta(name: 'CounterDemoDialog', requestType: 'CounterDemoDialogRequest', responseType: 'CounterDemoDialogResponse')],
    );

    final src = generateOxideStackSource(channels: channels);

    // imports should include the bridge alias when channels are present
    expect(src, contains("import '../src/rust/api/isolated_channels_bridge.dart' as channels;"));
    // and the matching `show` import for pending request types should appear
    expect(src, contains('show CounterDemoDialogPendingRequest'));
    // the io variant of the FRB file gives us concrete helper types like
    // PendingRequest; it should also be imported when channels are used.
    expect(src, contains("frb_generated.io.dart"));
    // the generated import should suppress unused-import warnings
    expect(src, contains('// ignore: unused_import'));

    // verify event getter forwards correctly
    expect(src, contains('Stream<CounterDemoEvent> get counterDemoEvents =>'));
    expect(src, contains('channels.counterDemoEventsStream()'));
    // channel import must be added so that event/callback types are visible
    expect(src, contains("isolated_channels_demo/channels.dart"));

    // verify callback request/response helpers
    expect(src, contains('Stream<CounterDemoDialogPendingRequest> get counterDemoDialogRequests'));
    expect(src, contains('channels.counterDemoDialogRequestsStream()'));
    expect(src, contains('Future<void> counterDemoDialogRespond'));
    expect(src, contains('channels.counterDemoDialogRespond'));

    // duplex channels are not modelled by metadata, so the generator
    // should *not* emit any helpers for them (they are added manually).
    expect(src, isNot(contains('counterDemoDuplex')));

    // helper function for app initialization should be generated
    expect(src, contains('Future<void> runOxideApp(Widget app,'));
    // the new generic channel init helper should be present and referenced by
    // the common init path
    expect(src, contains('Future<void> oxideInitChannels()'));
    expect(src, contains('oxideInitChannels()'));
    // generated helper must actually call into the bridge alias so we don't
    // depend on manually named functions
    expect(src, contains('channels.'));
    // because we supplied a crate name with underscores, the helper should
    // call the correctly-cased init function.
    expect(src, contains('channels.initIsolatedChannelsDemo()'));
    // and navigation startup is deferred until first frame
    expect(src, contains('addPostFrameCallback'));
  });

  test('generateOxideStackSource omits bridge import when no channels', () {
    final src = generateOxideStackSource(
      channels: RustChannelMetadata(events: [], callbacks: []),
    );
    expect(src, isNot(contains("import '../src/rust/api/isolated_channels_bridge.dart'")));
    expect(src, isNot(contains('isolated_channels_demo/channels.dart')));
    expect(src, isNot(contains('oxideInitChannels')));
  });

  test('generateOxideEntrypointSource exports runOxideApp', () {
    final src = generateOxideEntrypointSource();
    expect(src, contains("show OxideStack, runOxideApp"));
    // by default we do not export src/oxide.dart
    expect(src, isNot(contains("src/oxide.dart")));
  });

  test('generateOxideEntrypointSource optionally re-exports src/oxide.dart', () {
    final src = generateOxideEntrypointSource(includeSrcOxide: true);
    expect(src, contains("export 'src/oxide.dart'"));
  });

  test('core instantiation casts revision to int', () {
    final config = OxideCodegenConfig(
      prefix: 'p',
      stateType: 'S',
      snapshotType: 'Snap',
      actionsType: 'A',
      actionsIsEnum: false,
      engineType: 'E',
      backend: 'inherited',
      keepAlive: false,
      createEngine: 'create',
      disposeEngine: 'dispose',
      dispatch: 'dispatch',
      stateStream: 'stream',
      current: 'current',
      initApp: null,
      encodeCurrentState: null,
      encodeState: null,
      decodeState: null,
      actionConstructors: const [],
    );
    final src = buildCoreInstantiation(config);
    expect(src, contains('revisionOf: (snap) => snap.revision.toInt()'));
  });

  test('readRustChannelMetadata picks up JSON files and ignores mismatched crate names', () async {
    // create temporary metadata directory structure
    final dir = Directory('rust/target/oxide_channels');
    await dir.create(recursive: true);

    // first file uses some arbitrary crate name
    final file1 = File('${dir.path}/testcrate.json');
    final contents1 = '''{
  "crate_name": "testcrate",
  "events": [
    {"name": "FooEvents", "event_type": "FooEvent"}
  ],
  "callbacks": [
    {"name": "BarCallback", "request_type": "BarReq", "response_type": "BarResp"}
  ]
}''';
    await file1.writeAsString(contents1);

    // second file uses a different crate name to ensure filtering is not applied
    final file2 = File('${dir.path}/othercrate.json');
    final contents2 = '''{
  "crate_name": "othercrate",
  "events": [
    {"name": "BazEvents", "event_type": "BazEvent"}
  ],
  "callbacks": [
    {"name": "QuxCallback", "request_type": "QuxReq", "response_type": "QuxResp"}
  ]
}''';
    await file2.writeAsString(contents2);

    final meta = await readRustChannelMetadata();
    // both files should be consumed regardless of crate names
    expect(meta.events.length, 2);
    expect(meta.events.map((e) => e.name).toSet(), {'FooEvents', 'BazEvents'});
    expect(meta.callbacks.length, 2);
    expect(meta.callbacks.map((c) => c.name).toSet(), {'BarCallback', 'QuxCallback'});
    // the crate name should be null because the two files disagree
    expect(meta.crateName, isNull);

    // cleanup
    await dir.delete(recursive: true);
  });

  test('readRustRouteMetadata picks up JSON file and ignores crate name', () async {
    final dir = Directory('rust/target/oxide_routes');
    await dir.create(recursive: true);
    final file = File('${dir.path}/routes.json');
    final contents = '''{
  "crate_name": "somecrate",
  "routes": [
    {"kind": "Splash", "rust_type": "SplashRoute", "path": null, "return_type": "oxide_core::navigation::NoReturn", "extra_type": "oxide_core::navigation::NoExtra", "fields": []}
  ]
}''';
    await file.writeAsString(contents);

    final meta = await readRustRouteMetadata();
    expect(meta.routes.length, 1);
    expect(meta.routes.first.kind, 'Splash');
    // crateName should fallback to target crate name or unknown; ours returns unknown
    expect(meta.crateName, 'unknown');

    await dir.delete(recursive: true);
  });
}
