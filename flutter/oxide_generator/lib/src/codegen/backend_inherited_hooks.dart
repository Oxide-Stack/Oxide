import 'model.dart';

// flutter_hooks convenience wrapper for the inherited backend.
//
// Why: some apps prefer hooks APIs for widget access; this keeps hooks usage
// isolated to the hooks backend variant.
String buildInheritedHooksBackend(OxideCodegenConfig c, String inheritedBackend) {
  return '''
$inheritedBackend

OxideView<${c.stateType}, ${c.prefix}Actions> use${c.prefix}Oxide() {
  final context = useContext();
  return ${c.prefix}Scope.useOxide(context);
}
''';
}
