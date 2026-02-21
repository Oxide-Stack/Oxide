import 'package:build/build.dart';

import 'oxide_navigation_codegen.dart';

/// Build_runner builder that generates Oxide navigation glue under `lib/oxide_generated/`.
final class OxideNavigationBuilder implements Builder {
  @override
  final buildExtensions = const {
    r'$package$': [
      'lib/oxide_generated/routes/route_kind.g.dart',
      'lib/oxide_generated/routes/route_models.g.dart',
      'lib/oxide_generated/navigation/route_builders.g.dart',
      'lib/oxide_generated/navigation/navigation_runtime.g.dart',
    ],
  };

  @override
  Future<void> build(BuildStep buildStep) async {
    final metadata = await readRustRouteMetadata();
    final routePages = await discoverRoutePages(buildStep);

    await buildStep.writeAsString(
      AssetId(buildStep.inputId.package, 'lib/oxide_generated/routes/route_kind.g.dart'),
      generateRouteKindSource(metadata),
    );
    await buildStep.writeAsString(
      AssetId(buildStep.inputId.package, 'lib/oxide_generated/routes/route_models.g.dart'),
      generateRouteModelsSource(metadata),
    );
    await buildStep.writeAsString(
      AssetId(buildStep.inputId.package, 'lib/oxide_generated/navigation/route_builders.g.dart'),
      generateRouteBuildersSource(metadata, routePages),
    );
    await buildStep.writeAsString(
      AssetId(buildStep.inputId.package, 'lib/oxide_generated/navigation/navigation_runtime.g.dart'),
      generateNavigationRuntimeSource(metadata),
    );
  }
}
