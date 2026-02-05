/// String-based Dart code generation for `@OxideStore`.
///
/// This library keeps generation logic free of analyzer types so it can be
/// unit-tested with plain Dart values and remain easy to reason about.
library;

export 'codegen/model.dart';

import 'codegen/generate.dart' as internal;
import 'codegen/model.dart';

String generateOxideStoreSource(OxideCodegenConfig c) => internal.generateOxideStoreSource(c);
