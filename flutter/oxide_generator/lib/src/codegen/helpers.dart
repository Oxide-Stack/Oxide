// Small string helpers used by codegen templates.
//
// Why: keep formatting logic centralized so template files stay focused on
// the generated output shape.
String lowerFirst(String value) {
  if (value.isEmpty) return value;
  return value[0].toLowerCase() + value.substring(1);
}
