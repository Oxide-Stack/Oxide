import 'model.dart';

// Action facade string generation.
//
// Why: generated stores expose an `Actions` class so UI code can call strongly-
// typed methods without manually constructing action objects.
//
// How: emit one method per enum constant (enum actions) or per factory
// constructor (union-class actions), delegating to the store's `_dispatch`.
String generateActionsMethods({required String actionsType, required bool actionsIsEnum, required List<OxideActionConstructor> constructors}) {
  final buffer = StringBuffer();
  for (final ctor in constructors) {
    if (ctor.name.isEmpty) continue;

    final signature = StringBuffer();
    signature.write('  Future<void> ${ctor.name}(');

    if (ctor.positionalParams.isNotEmpty) {
      signature.write(ctor.positionalParams.map(_formatPositional).join(', '));
    }
    if (ctor.namedParams.isNotEmpty) {
      if (ctor.positionalParams.isNotEmpty) signature.write(', ');
      signature.write('{');
      signature.write(ctor.namedParams.map(_formatNamed).join(', '));
      signature.write('}');
    }
    signature.write(') async {\n');

    final ctorArgs = StringBuffer();
    if (ctor.positionalParams.isNotEmpty) {
      ctorArgs.write(ctor.positionalParams.map((p) => p.name).join(', '));
    }
    if (ctor.namedParams.isNotEmpty) {
      if (ctor.positionalParams.isNotEmpty) ctorArgs.write(', ');
      ctorArgs.write(ctor.namedParams.map((p) => '${p.name}: ${p.name}').join(', '));
    }

    buffer.write(signature);
    if (actionsIsEnum) {
      buffer.write('    await _dispatch($actionsType.${ctor.name});\n');
    } else {
      buffer.write('    await _dispatch($actionsType.${ctor.name}($ctorArgs));\n');
    }
    buffer.write('  }\n\n');
  }

  return buffer.toString().trimRight();
}

String _formatPositional(OxideActionParam p) => '${p.type} ${p.name}';

String _formatNamed(OxideActionParam p) {
  if (p.isRequiredNamed) return 'required ${p.type} ${p.name}';
  return '${p.type} ${p.name}';
}
