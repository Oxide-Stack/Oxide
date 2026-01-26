import 'package:build/build.dart';
import 'package:source_gen/source_gen.dart';

import 'src/oxide_store_generator.dart';

/// Creates the Oxide code generator builder.
///
/// This builder scans for `@OxideStore(...)` annotations and generates a
/// `*.oxide.g.dart` part file containing store glue code.
///
/// # Returns
/// A [Builder] suitable for use in `build.yaml`.
Builder oxideBuilder(BuilderOptions options) {
  return PartBuilder([OxideStoreGenerator()], '.oxide.g.dart');
}
