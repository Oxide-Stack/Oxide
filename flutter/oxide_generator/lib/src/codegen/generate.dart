import 'actions.dart';
import 'backend_bloc.dart';
import 'backend_inherited.dart';
import 'backend_inherited_hooks.dart';
import 'backend_riverpod.dart';
import 'core_instantiation.dart';
import 'model.dart';

// High-level store source generation.
//
// Why: keep the orchestration logic centralized so it is obvious which pieces
// contribute to the final output (actions facade + selected backend adapter).
String generateOxideStoreSource(OxideCodegenConfig c) {
  final actionsMethods = generateActionsMethods(actionsType: c.actionsType, actionsIsEnum: c.actionsIsEnum, constructors: c.actionConstructors);

  final coreInstantiation = buildCoreInstantiation(c);

  final actionsClass =
      '''
class ${c.prefix}Actions {
  ${c.prefix}Actions._(Future<void> Function(${c.actionsType} action) dispatch)
      : _dispatch = dispatch;

  Future<void> Function(${c.actionsType} action) _dispatch;

  void _bind(Future<void> Function(${c.actionsType} action) dispatch) {
    _dispatch = dispatch;
  }

$actionsMethods
}
''';

  final inheritedBackend = buildInheritedBackend(c, coreInstantiation);
  final inheritedHooksBackend = buildInheritedHooksBackend(c, inheritedBackend);
  final riverpodBackend = buildRiverpodBackend(c, coreInstantiation);
  final blocBackend = buildBlocBackend(c, coreInstantiation);

  final backendCode = switch (c.backend) {
    'inherited' => inheritedBackend,
    'inheritedHooks' => inheritedHooksBackend,
    'riverpod' => riverpodBackend,
    'bloc' => blocBackend,
    _ => inheritedBackend,
  };

  return '''
$actionsClass

$backendCode
''';
}
