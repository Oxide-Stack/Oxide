// Build runner entrypoints for Oxide code generation.
import 'package:build/build.dart';
import 'package:source_gen/source_gen.dart';

import 'src/oxide_navigation_builder.dart';
import 'src/oxide_store_generator.dart';

Builder oxideBuilder(BuilderOptions options) {
  return PartBuilder([OxideStoreGenerator()], '.oxide.g.dart');
}

Builder oxideNavigationBuilder(BuilderOptions options) {
  return OxideNavigationBuilder();
}
