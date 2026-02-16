// Pure-data config model for store code generation.
//
// Why: keeping the generator input as simple Dart values allows unit-testing
// without analyzer types and keeps the codegen surface stable.
//
// How: OxideStoreGenerator translates analyzer metadata into these structs, and
// the string emitter consumes them to produce the `*.oxide.g.dart` part output.
/// Configuration used to generate store glue code.
final class OxideCodegenConfig {
  const OxideCodegenConfig({
    required this.prefix,
    required this.stateType,
    required this.snapshotType,
    required this.actionsType,
    required this.actionsIsEnum,
    required this.engineType,
    // Why: Most stores do not opt into sliced updates, and the generator should
    // not force tests/callers to provide slice metadata when unused.
    //
    // How: Keep these optional; `OxideStoreGenerator` only populates them when
    // `@OxideStore(slices: [...])` is present.
    this.sliceType,
    this.slices,
    required this.backend,
    required this.keepAlive,
    required this.createEngine,
    required this.disposeEngine,
    required this.dispatch,
    required this.stateStream,
    required this.current,
    required this.initApp,
    required this.encodeCurrentState,
    required this.encodeState,
    required this.decodeState,
    required this.actionConstructors,
  });

  final String prefix;
  final String stateType;
  final String snapshotType;
  final String actionsType;
  final bool actionsIsEnum;
  final String engineType;

  final String? sliceType;
  final List<String>? slices;

  final String backend;
  final bool keepAlive;

  final String createEngine;
  final String disposeEngine;
  final String dispatch;
  final String stateStream;
  final String current;
  final String? initApp;
  final String? encodeCurrentState;
  final String? encodeState;
  final String? decodeState;

  final List<OxideActionConstructor> actionConstructors;
}

/// Describes one dispatchable action entrypoint.
final class OxideActionConstructor {
  const OxideActionConstructor({required this.name, required this.positionalParams, required this.namedParams});

  final String name;
  final List<OxideActionParam> positionalParams;
  final List<OxideActionParam> namedParams;
}

/// Describes a parameter for a generated action helper method.
final class OxideActionParam {
  const OxideActionParam({required this.name, required this.type, required this.isRequiredNamed});

  final String name;
  final String type;
  final bool isRequiredNamed;
}
