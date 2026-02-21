import 'dart:convert';
import 'dart:io';

import 'package:analyzer/dart/element/element.dart';
import 'package:analyzer/dart/constant/value.dart';
import 'package:build/build.dart';
import 'package:dart_style/dart_style.dart';
import 'package:glob/glob.dart';
import 'package:oxide_annotations/oxide_annotations.dart';
import 'package:source_gen/source_gen.dart';

final _formatter = DartFormatter(languageVersion: DartFormatter.latestShortStyleLanguageVersion, pageWidth: 100);

final class RustRouteFieldMeta {
  RustRouteFieldMeta({required this.name, required this.type});

  final String name;
  final String type;
}

final class RustRouteMeta {
  RustRouteMeta({
    required this.kind,
    required this.rustType,
    required this.path,
    required this.returnType,
    required this.extraType,
    required this.fields,
  });

  final String kind;
  final String rustType;
  final String? path;
  final String returnType;
  final String extraType;
  final List<RustRouteFieldMeta> fields;
}

final class RustRouteMetadata {
  RustRouteMetadata({required this.crateName, required this.routes});

  final String crateName;
  final List<RustRouteMeta> routes;
}

final class RoutePageBinding {
  const RoutePageBinding({required this.kindKey, required this.widgetType, required this.libraryUri});

  final String kindKey;
  final String widgetType;
  final String libraryUri;
}

Future<RustRouteMetadata> readRustRouteMetadata() async {
  final dir = Directory('rust/target/oxide_routes');
  if (!dir.existsSync()) {
    return RustRouteMetadata(crateName: 'unknown', routes: const []);
  }

  final routes = <RustRouteMeta>[];
  String crateName = 'unknown';

  final files = dir.listSync(followLinks: false).whereType<File>().where((f) => f.path.endsWith('.json')).toList(growable: false);

  for (final file in files) {
    final jsonStr = await file.readAsString();
    final obj = jsonDecode(jsonStr);
    if (obj is! Map<String, dynamic>) continue;

    final fileCrate = obj['crate_name'];
    if (fileCrate is String && fileCrate.isNotEmpty) {
      crateName = fileCrate;
    }

    final rawRoutes = obj['routes'];
    if (rawRoutes is! List) continue;

    for (final raw in rawRoutes) {
      if (raw is! Map<String, dynamic>) continue;
      final kind = raw['kind'];
      final rustType = raw['rust_type'];
      final path = raw['path'];
      final returnType = raw['return_type'];
      final extraType = raw['extra_type'];
      final rawFields = raw['fields'];

      if (kind is! String || kind.isEmpty) continue;
      if (rustType is! String || rustType.isEmpty) continue;

      final fields = <RustRouteFieldMeta>[];
      if (rawFields is List) {
        for (final f in rawFields) {
          if (f is! Map<String, dynamic>) continue;
          final name = f['name'];
          final type = f['ty'];
          if (name is String && type is String) {
            fields.add(RustRouteFieldMeta(name: name, type: type));
          }
        }
      }

      routes.add(
        RustRouteMeta(
          kind: kind,
          rustType: rustType,
          path: path is String ? path : null,
          returnType: returnType is String ? returnType : 'oxide_core::navigation::NoReturn',
          extraType: extraType is String ? extraType : 'oxide_core::navigation::NoExtra',
          fields: fields,
        ),
      );
    }
  }

  final byKind = <String, RustRouteMeta>{};
  for (final r in routes) {
    final existing = byKind[r.kind];
    if (existing == null) {
      byKind[r.kind] = r;
      continue;
    }
    if (!_sameRouteMeta(existing, r)) {
      throw StateError('Conflicting route metadata for kind "${r.kind}" under rust/target/oxide_routes');
    }
  }

  final dedupedRoutes = byKind.values.toList(growable: false)..sort((a, b) => a.kind.compareTo(b.kind));
  return RustRouteMetadata(crateName: crateName, routes: dedupedRoutes);
}

Future<List<RoutePageBinding>> discoverRoutePages(BuildStep buildStep) async {
  final bindings = <RoutePageBinding>[];

  final package = buildStep.inputId.package;
  final byKindKey = <String, (RoutePageBinding, int)>{};
  final annotationRe = RegExp(r'@OxideRoutePage\s*\(\s*([^\)\r\n]+?)\s*\)');
  final classRe = RegExp(r'\bclass\s+([A-Za-z_]\w*)');
  final partOfRe = RegExp(r'^\s*part\s+of\b', multiLine: true);
  final routeCtorRe = RegExp(r'required\s+this\.route\b');
  final routeFieldRe = RegExp(r'\bfinal\s+[A-Za-z_]\w*Route\s+route\s*;');

  await for (final assetId in buildStep.findAssets(Glob('lib/**.dart'))) {
    if (assetId.package != package) continue;
    if (assetId.path.startsWith('lib/oxide_generated/')) continue;
    if (assetId.path.endsWith('.g.dart')) continue;

    final src = await buildStep.readAsString(assetId);
    if (partOfRe.hasMatch(src)) continue;

    final libraryUri = Uri(scheme: 'package', path: '$package/${assetId.path.substring('lib/'.length)}').toString();

    for (final annMatch in annotationRe.allMatches(src)) {
      final kindKey = _routeKindKeyFromSource(annMatch.group(1) ?? '');
      if (kindKey == null || kindKey.isEmpty) continue;

      final afterAnn = src.substring(annMatch.end);
      final classMatch = classRe.firstMatch(afterAnn);
      if (classMatch == null) continue;

      final widgetType = classMatch.group(1) ?? '';
      if (widgetType.isEmpty) continue;

      final classStart = annMatch.end + classMatch.start;
      final classSnippetEnd = (classStart + 2500) < src.length ? (classStart + 2500) : src.length;
      final classSnippet = src.substring(classStart, classSnippetEnd);
      final hasRouteCtor = routeCtorRe.hasMatch(classSnippet);
      final hasRouteField = routeFieldRe.hasMatch(classSnippet);
      final isLikelyPage = hasRouteCtor || hasRouteField || widgetType.endsWith('Page');
      if (!isLikelyPage) continue;

      var score = 0;
      if (widgetType.endsWith('Page')) score += 2;
      if (hasRouteCtor) score += 2;
      if (hasRouteField) score += 2;
      if (widgetType.endsWith('Screen')) score -= 1;

      final binding = RoutePageBinding(kindKey: kindKey, widgetType: widgetType, libraryUri: libraryUri);
      final existing = byKindKey[kindKey];
      if (existing == null) {
        byKindKey[kindKey] = (binding, score);
        continue;
      }
      if (score > existing.$2) {
        byKindKey[kindKey] = (binding, score);
        continue;
      }
      if (score == existing.$2) {
        throw StateError('Duplicate @OxideRoutePage binding for "$kindKey": ${existing.$1.widgetType} and ${binding.widgetType}');
      }
    }
  }

  for (final entry in byKindKey.values) {
    bindings.add(entry.$1);
  }
  bindings.sort((a, b) => a.kindKey.compareTo(b.kindKey));
  return bindings;
}

String generateRouteKindSource(RustRouteMetadata metadata) {
  if (metadata.routes.isEmpty) {
    final buf = StringBuffer()
      ..writeln('// Generated by oxide_generator. Do not edit.')
      ..writeln()
      ..writeln('enum RouteKind {')
      ..writeln('  unknown,')
      ..writeln('}')
      ..writeln()
      ..writeln('extension RouteKindX on RouteKind {')
      ..writeln("  String get asStr => switch (this) {")
      ..writeln("    RouteKind.unknown => 'Unknown',")
      ..writeln('  };')
      ..writeln('}');

    return _formatter.format(buf.toString());
  }

  final buf = StringBuffer()
    ..writeln('// Generated by oxide_generator. Do not edit.')
    ..writeln()
    ..writeln('enum RouteKind {');

  for (final r in metadata.routes) {
    buf.writeln('  ${_lowerCamel(r.kind)},');
  }

  buf
    ..writeln('}')
    ..writeln()
    ..writeln('extension RouteKindX on RouteKind {')
    ..writeln("  String get asStr => switch (this) {");

  for (final r in metadata.routes) {
    final v = _lowerCamel(r.kind);
    buf.writeln("    RouteKind.$v => '${r.kind}',");
  }

  buf
    ..writeln('  };')
    ..writeln('}');

  return _formatter.format(buf.toString());
}

String generateRouteModelsSource(RustRouteMetadata metadata) {
  final buf = StringBuffer()
    ..writeln('// Generated by oxide_generator. Do not edit.')
    ..writeln()
    ..writeln("import 'route_kind.g.dart';")
    ..writeln()
    ..writeln('abstract interface class OxideRoute {')
    ..writeln('  RouteKind get kind;')
    ..writeln('  Map<String, dynamic> toJson();')
    ..writeln('}')
    ..writeln();

  for (final r in metadata.routes) {
    final className = r.rustType;
    buf.writeln('final class $className implements OxideRoute {');
    if (r.fields.isEmpty) {
      buf.writeln('  const $className();');
    } else {
      buf.writeln('  const $className({');
      for (final f in r.fields) {
        buf.writeln('    required this.${_lowerCamel(f.name)},');
      }
      buf.writeln('  });');
    }
    buf.writeln();
    if (r.fields.isNotEmpty) {
      for (final f in r.fields) {
        buf.writeln('  final Object ${_lowerCamel(f.name)};');
      }
      buf.writeln();
    }
    buf.writeln('  @override');
    buf.writeln('  RouteKind get kind => RouteKind.${_lowerCamel(r.kind)};');
    buf.writeln();
    buf.writeln('  factory $className.fromJson(Map<String, dynamic> json) {');
    if (r.fields.isEmpty) {
      buf.writeln('    return const $className();');
    } else {
      buf.writeln('    return $className(');
      for (final f in r.fields) {
        final name = _lowerCamel(f.name);
        buf.writeln("      $name: json['$name'] as Object,");
      }
      buf.writeln('    );');
    }
    buf.writeln('  }');
    buf.writeln();
    buf.writeln('  @override');
    buf.writeln('  Map<String, dynamic> toJson() {');
    if (r.fields.isEmpty) {
      buf.writeln('    return const <String, dynamic>{};');
    } else {
      buf.writeln('    return <String, dynamic>{');
      for (final f in r.fields) {
        final name = _lowerCamel(f.name);
        buf.writeln("      '$name': $name,");
      }
      buf.writeln('    };');
    }
    buf.writeln('  }');
    buf.writeln('}');
    buf.writeln();
  }

  return _formatter.format(buf.toString());
}

String generateRouteBuildersSource(RustRouteMetadata metadata, List<RoutePageBinding> bindings) {
  final buf = StringBuffer()
    ..writeln('// Generated by oxide_generator. Do not edit.')
    ..writeln()
    ..writeln("import 'package:flutter/widgets.dart';")
    ..writeln("import '../routes/route_kind.g.dart';")
    ..writeln("import '../routes/route_models.g.dart';")
    ..writeln()
    ..writeln('typedef OxideRouteBuilder = Widget Function(BuildContext context, OxideRoute route);')
    ..writeln()
    ..writeln('final Map<RouteKind, OxideRouteBuilder> oxideRouteBuilders = <RouteKind, OxideRouteBuilder>{');

  final byKind = <String, RoutePageBinding>{};
  for (final b in bindings) {
    byKind[b.kindKey] = b;
  }

  for (final r in metadata.routes) {
    final kindKey = _lowerCamel(r.kind);
    final kindExpr = 'RouteKind.$kindKey';
    final binding = byKind[kindKey];
    if (binding == null) {
      buf.writeln(
        '  $kindExpr: (context, route) => throw UnimplementedError('
        "'Missing @OxideRoutePage mapping for $kindExpr'),",
      );
      continue;
    }
    buf.writeln('  $kindExpr: (context, route) {');
    buf.writeln('    final r = route as ${r.rustType};');
    buf.writeln('    return ${binding.widgetType}(route: r);');
    buf.writeln('  },');
  }

  buf.writeln('};');

  final raw = buf.toString();
  final importLines = bindings.map((b) => b.libraryUri).toSet().toList(growable: false)..sort();
  if (importLines.isEmpty) {
    return _formatter.format(raw);
  }

  final insertAfter = "import '../routes/route_models.g.dart';";
  final insertion = importLines.map((u) => "import '$u';").join('\n');
  return _formatter.format(raw.replaceFirst(insertAfter, '$insertAfter\n$insertion'));
}

String generateNavigationRuntimeSource(RustRouteMetadata metadata) {
  final buf = StringBuffer()
    ..writeln('// Generated by oxide_generator. Do not edit.')
    ..writeln()
    ..writeln("import 'dart:async';")
    ..writeln("import 'dart:convert';")
    ..writeln()
    ..writeln("import 'package:flutter/widgets.dart';")
    ..writeln("import 'package:oxide_runtime/oxide_runtime.dart';")
    ..writeln()
    ..writeln("import 'route_builders.g.dart';")
    ..writeln("import '../routes/route_kind.g.dart';")
    ..writeln("import '../routes/route_models.g.dart';")
    ..writeln()
    ..writeln("import '../../src/rust/api/navigation_bridge.dart' as rust;")
    ..writeln()
    ..writeln('final GlobalKey<NavigatorState> oxideNavigatorKey = GlobalKey<NavigatorState>();')
    ..writeln()
    ..writeln(
      'final NavigatorNavigationHandler<OxideRoute, RouteKind> oxideNavigationHandler = '
      'NavigatorNavigationHandler<OxideRoute, RouteKind>(',
    )
    ..writeln('  navigatorKey: oxideNavigatorKey,')
    ..writeln('  kindOf: (r) => r.kind,')
    ..writeln('  routeBuilders: oxideRouteBuilders,')
    ..writeln(');')
    ..writeln();

  buf
    ..writeln('OxideNavigationCommand<OxideRoute, RouteKind>? _decodeOxideNavCommand(String json) {')
    ..writeln('  final obj = jsonDecode(json);')
    ..writeln('  if (obj is! Map<String, dynamic> || obj.length != 1) return null;')
    ..writeln('  final entry = obj.entries.first;')
    ..writeln()
    ..writeln('  switch (entry.key) {')
    ..writeln("    case 'Push':")
    ..writeln('      final payload = entry.value;')
    ..writeln('      if (payload is! Map<String, dynamic>) return null;')
    ..writeln("      final routeObj = payload['route'];")
    ..writeln('      if (routeObj is! Map<String, dynamic>) return null;')
    ..writeln("      final kind = routeObj['kind'];")
    ..writeln("      final routePayload = routeObj['payload'];")
    ..writeln('      if (kind is! String || routePayload is! Map<String, dynamic>) return null;')
    ..writeln("      final ticket = payload['ticket'] as String?;")
    ..writeln()
    ..writeln('      final route = switch (kind) {');

  for (final r in metadata.routes) {
    final rustType = r.rustType;
    buf.writeln("        '${r.kind}' => $rustType.fromJson(routePayload),");
  }
  buf
    ..writeln('        _ => null,')
    ..writeln('      };')
    ..writeln('      if (route == null) return null;')
    ..writeln('      return OxideNavigationCommand.push(route: route, ticket: ticket);')
    ..writeln("    case 'Pop':")
    ..writeln('      final payload = entry.value;')
    ..writeln('      if (payload is! Map<String, dynamic>) return null;')
    ..writeln("      return OxideNavigationCommand.pop(result: payload['result']);")
    ..writeln("    case 'PopUntil':")
    ..writeln('      final payload = entry.value;')
    ..writeln('      if (payload is! Map<String, dynamic>) return null;')
    ..writeln("      final kind = payload['kind'];")
    ..writeln('      if (kind is! String) return null;')
    ..writeln('      final routeKind = switch (kind) {');

  for (final r in metadata.routes) {
    buf.writeln("        '${r.kind}' => RouteKind.${_lowerCamel(r.kind)},");
  }
  buf
    ..writeln('        _ => null,')
    ..writeln('      };')
    ..writeln('      if (routeKind == null) return null;')
    ..writeln('      return OxideNavigationCommand.popUntil(kind: routeKind);')
    ..writeln("    case 'Reset':")
    ..writeln('      final payload = entry.value;')
    ..writeln('      if (payload is! Map<String, dynamic>) return null;')
    ..writeln("      final routes = payload['routes'];")
    ..writeln('      if (routes is! List) return null;')
    ..writeln('      final decoded = <OxideRoute>[];')
    ..writeln('      for (final r in routes) {')
    ..writeln('        if (r is! Map<String, dynamic>) continue;')
    ..writeln("        final kind = r['kind'];")
    ..writeln("        final routePayload = r['payload'];")
    ..writeln('        if (kind is! String || routePayload is! Map<String, dynamic>) continue;')
    ..writeln('        final route = switch (kind) {');

  for (final r in metadata.routes) {
    final rustType = r.rustType;
    buf.writeln("          '${r.kind}' => $rustType.fromJson(routePayload),");
  }
  buf
    ..writeln('          _ => null,')
    ..writeln('        };')
    ..writeln('        if (route != null) decoded.add(route);')
    ..writeln('      }')
    ..writeln('      return OxideNavigationCommand.reset(routes: decoded);')
    ..writeln('  }')
    ..writeln()
    ..writeln('  return null;')
    ..writeln('}')
    ..writeln()
    ..writeln(
      'final OxideNavigationRuntime<OxideRoute, RouteKind> oxideNavigationRuntime = '
      'OxideNavigationRuntime<OxideRoute, RouteKind>(',
    )
    ..writeln('  commands: rust.oxideNavCommandsStream()')
    ..writeln('      .map(_decodeOxideNavCommand)')
    ..writeln('      .where((c) => c != null)')
    ..writeln('      .cast<OxideNavigationCommand<OxideRoute, RouteKind>>(),')
    ..writeln('  handler: oxideNavigationHandler,')
    ..writeln(
      '  emitResult: (ticket, result) => rust.oxideNavEmitResult('
      'ticket: ticket, resultJson: jsonEncode(result)),',
    )
    ..writeln(
      '  setCurrentRoute: (route) => rust.oxideNavSetCurrentRoute('
      'kind: route.kind.asStr, payloadJson: jsonEncode(route.toJson())),',
    )
    ..writeln(');')
    ..writeln()
    ..writeln('void oxideNavStart() {')
    ..writeln('  oxideNavigationRuntime.start();')
    ..writeln('}')
    ..writeln()
    ..writeln('Future<void> oxideNavStop() => oxideNavigationRuntime.stop();');

  return _formatter.format(buf.toString());
}

String _lowerCamel(String s) {
  if (s.isEmpty) return s;
  final parts = s.split(RegExp(r'[_\-\s]+')).where((p) => p.isNotEmpty).toList(growable: false);
  if (parts.isEmpty) return s;

  final first = parts.first;
  final firstLower = first[0].toLowerCase() + first.substring(1);
  if (parts.length == 1) return firstLower;

  final rest = parts.skip(1).map((p) => p.isEmpty ? p : (p[0].toUpperCase() + p.substring(1))).join();
  return '$firstLower$rest';
}

bool _sameRouteMeta(RustRouteMeta a, RustRouteMeta b) {
  if (a.kind != b.kind) return false;
  if (a.rustType != b.rustType) return false;
  if (a.path != b.path) return false;
  if (a.returnType != b.returnType) return false;
  if (a.extraType != b.extraType) return false;
  if (a.fields.length != b.fields.length) return false;
  for (var i = 0; i < a.fields.length; i++) {
    final af = a.fields[i];
    final bf = b.fields[i];
    if (af.name != bf.name) return false;
    if (af.type != bf.type) return false;
  }
  return true;
}

String? _routeKindKeyFromSource(String expr) {
  final s = expr.trim();
  if (s.isEmpty) return null;

  final strMatch = RegExp(r"""^(['"])(.*)\1$""").firstMatch(s);
  if (strMatch != null) {
    final raw = strMatch.group(2) ?? '';
    if (raw.isEmpty) return null;
    return _lowerCamel(raw);
  }

  final enumMatch = RegExp(r'^RouteKind\.([A-Za-z_]\w*)$').firstMatch(s);
  if (enumMatch != null) {
    final raw = enumMatch.group(1) ?? '';
    if (raw.isEmpty) return null;
    return _lowerCamel(raw);
  }

  return null;
}

String? _enumValueExpression(DartObject obj) {
  final objType = obj.type;
  final objElement = objType?.element;
  if (objType == null || objElement is! EnumElement) return null;

  final nameValue = obj.getField('name')?.toStringValue();
  if (nameValue != null && nameValue.isNotEmpty) {
    final enumTypeName = objType.getDisplayString(withNullability: false);
    return '$enumTypeName.$nameValue';
  }

  final enumIndex = obj.getField('index')?.toIntValue();
  if (enumIndex == null) return null;

  final constants = (() {
    final dynamic dyn = objElement;
    try {
      final value = dyn.constants;
      if (value is List) return value;
    } catch (_) {}
    return null;
  })();

  String? constantName;
  if (constants != null) {
    if (enumIndex >= 0 && enumIndex < constants.length) {
      final dynamic dynConstant = constants[enumIndex];
      final name = dynConstant.name;
      if (name is String && name.isNotEmpty) constantName = name;
    }
  } else {
    final fields = objElement.fields.where((f) => f.isEnumConstant).toList(growable: false);
    if (enumIndex >= 0 && enumIndex < fields.length) constantName = fields[enumIndex].name;
  }

  if (constantName == null || constantName.isEmpty) return null;
  final enumTypeName = objType.getDisplayString(withNullability: false);
  return '$enumTypeName.$constantName';
}

String? _routeKindKey(DartObject obj) {
  final str = obj.toStringValue();
  if (str != null && str.isNotEmpty) {
    return _lowerCamel(str);
  }

  final expr = _enumValueExpression(obj);
  if (expr == null || !expr.contains('.')) return null;
  final last = expr.split('.').last;
  if (last.isEmpty) return null;
  return _lowerCamel(last);
}
