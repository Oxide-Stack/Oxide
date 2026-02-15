import 'model.dart';

// Engine binding wiring for generated stores.
//
// Why: backends (Inherited/Riverpod/BLoC) should talk to `OxideStoreCore`, not
// to FRB binding functions directly, to keep lifecycle code consistent.
//
// How: generate the single block that binds create/dispatch/current/stream into
// the `OxideStoreCore` constructor, plus optional hooks.
String buildCoreInstantiation(OxideCodegenConfig c) {
  final initAppArg = (c.initApp == null || c.initApp!.isEmpty) ? '' : '    initApp: () => ${c.initApp}(),\n';
  final encodeCurrentStateArg = (c.encodeCurrentState == null || c.encodeCurrentState!.isEmpty)
      ? ''
      : '    encodeCurrentState: (engine) async => await ${c.encodeCurrentState}(engine: engine),\n';

  return '''
  late final OxideStoreCore<${c.stateType}, ${c.actionsType}, ${c.engineType}, ${c.snapshotType}> _core =
      OxideStoreCore<${c.stateType}, ${c.actionsType}, ${c.engineType}, ${c.snapshotType}>(
    createEngine: (_) => ${c.createEngine}(),
    disposeEngine: (engine) => ${c.disposeEngine}(engine: engine),
    dispatch: (engine, action) => ${c.dispatch}(engine: engine, action: action),
    current: (engine) => ${c.current}(engine: engine),
    stateStream: (engine) => ${c.stateStream}(engine: engine),
    stateFromSnapshot: (snap) => snap.state,
$initAppArg$encodeCurrentStateArg  );
''';
}
