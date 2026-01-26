import 'package:analyzer/dart/element/element.dart';
import 'package:analyzer/dart/element/type.dart';
import 'package:build/build.dart';
import 'package:oxide_annotations/oxide_annotations.dart';
import 'package:source_gen/source_gen.dart';

import 'oxide_store_codegen.dart';

/// Source_gen generator for `@OxideStore` annotations.
///
/// This generator emits glue code that instantiates `OxideStoreCore` and
/// provides a backend-specific controller/actions API.
final class OxideStoreGenerator extends GeneratorForAnnotation<OxideStore> {
  /// Generates code for a single annotated element.
  ///
  /// # Throws
  /// Throws [InvalidGenerationSourceError] if the annotation is applied to an
  /// unsupported element or its configuration is invalid.
  @override
  String generateForAnnotatedElement(Element element, ConstantReader annotation, BuildStep buildStep) {
    if (element is! ClassElement) {
      throw InvalidGenerationSourceError('@OxideStore can only be used on classes.', element: element);
    }

    final nameOverride = annotation.peek('name')?.stringValue;
    final prefix = (nameOverride == null || nameOverride.isEmpty) ? element.name : nameOverride;

    final stateType = _readType(annotation, 'state');
    final snapshotType = _readType(annotation, 'snapshot');
    final actionsType = _readType(annotation, 'actions');
    final engineType = _readType(annotation, 'engine');

    final createEngine = annotation.read('createEngine').stringValue;
    final disposeEngine = annotation.read('disposeEngine').stringValue;
    final dispatchFn = annotation.read('dispatch').stringValue;
    final stateStreamFn = annotation.read('stateStream').stringValue;
    final currentFn = annotation.read('current').stringValue;
    final initAppFn = annotation.peek('initApp')?.stringValue;
    final backendIndex = annotation.peek('backend')?.objectValue.getField('index')?.toIntValue();
    final backend = switch (backendIndex) {
      0 => 'inherited',
      1 => 'inheritedHooks',
      2 => 'riverpod',
      3 => 'bloc',
      _ => 'inherited',
    };
    final encodeCurrentStateFn = annotation.peek('encodeCurrentState')?.stringValue;
    final encodeStateFn = annotation.peek('encodeState')?.stringValue;
    final decodeStateFn = annotation.peek('decodeState')?.stringValue;

    final actionsElement = actionsType.element;
    final actionsIsEnum = actionsElement is EnumElement;
    if (actionsElement is! ClassElement && actionsElement is! EnumElement) {
      throw InvalidGenerationSourceError('@OxideStore.actions must be a class or enum type.', element: element);
    }

    final stateTypeName = _typeName(stateType);
    final snapshotTypeName = _typeName(snapshotType);
    final actionsTypeName = _typeName(actionsType);
    final engineTypeName = _typeName(engineType);

    final actionConstructors = <OxideActionConstructor>[];
    if (actionsElement is ClassElement) {
      for (final ctor in actionsElement.constructors) {
        if (!ctor.isFactory) continue;
        if (ctor.isPrivate) continue;
        if (ctor.name.isEmpty) continue;

        actionConstructors.add(
          OxideActionConstructor(
            name: ctor.name,
            positionalParams: ctor.parameters
                .where((p) => p.isPositional)
                .map((p) => OxideActionParam(name: p.name, type: p.type.getDisplayString(withNullability: true), isRequiredNamed: false))
                .toList(growable: false),
            namedParams: ctor.parameters
                .where((p) => p.isNamed)
                .map((p) => OxideActionParam(name: p.name, type: p.type.getDisplayString(withNullability: true), isRequiredNamed: p.isRequiredNamed))
                .toList(growable: false),
          ),
        );
      }
    } else if (actionsElement is EnumElement) {
      final constants = (() {
        final dynamic dyn = actionsElement;
        try {
          final value = dyn.constants;
          if (value is List) return value;
        } catch (_) {}
        return null;
      })();

      if (constants != null) {
        for (final constant in constants) {
          final dynamic dynConstant = constant;
          final name = dynConstant.name;
          if (name is! String || name.isEmpty) continue;
          actionConstructors.add(OxideActionConstructor(name: name, positionalParams: const [], namedParams: const []));
        }
      } else {
        for (final field in actionsElement.fields) {
          if (!field.isEnumConstant) continue;
          actionConstructors.add(OxideActionConstructor(name: field.name, positionalParams: const [], namedParams: const []));
        }
      }
    }

    return generateOxideStoreSource(
      OxideCodegenConfig(
        prefix: prefix,
        stateType: stateTypeName,
        snapshotType: snapshotTypeName,
        actionsType: actionsTypeName,
        actionsIsEnum: actionsIsEnum,
        engineType: engineTypeName,
        backend: backend,
        createEngine: createEngine,
        disposeEngine: disposeEngine,
        dispatch: dispatchFn,
        stateStream: stateStreamFn,
        current: currentFn,
        initApp: initAppFn,
        encodeCurrentState: encodeCurrentStateFn,
        encodeState: encodeStateFn,
        decodeState: decodeStateFn,
        actionConstructors: actionConstructors,
      ),
    );
  }
}

/// Reads a type-valued field from an annotation.
DartType _readType(ConstantReader annotation, String field) {
  return annotation.read(field).typeValue;
}

/// Returns a display name for a type without nullability markers.
String _typeName(DartType type) {
  return type.getDisplayString(withNullability: false);
}
