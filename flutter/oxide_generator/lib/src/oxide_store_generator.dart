// Analyzer-driven configuration extraction for Oxide store generation.
import 'package:analyzer/dart/element/element.dart';
import 'package:analyzer/dart/element/type.dart';
import 'package:build/build.dart';
import 'package:oxide_annotations/oxide_annotations.dart';
import 'package:source_gen/source_gen.dart';

import 'oxide_store_codegen.dart';

final class OxideStoreGenerator extends GeneratorForAnnotation<OxideStore> {
  @override
  String generateForAnnotatedElement(
    Element element,
    ConstantReader annotation,
    BuildStep buildStep,
  ) {
    final target = _readTargetClass(element);
    final prefix = _resolvePrefix(annotation, target);

    final stateType = _readType(annotation, 'state');
    final snapshotType = _readType(annotation, 'snapshot');
    final actionsType = _readType(annotation, 'actions');
    final engineType = _readType(annotation, 'engine');

    final keepAlive = annotation.peek('keepAlive')?.boolValue ?? false;
    final bindings = _readBindings(annotation);
    final slices = _readSlices(annotation, snapshotType, target);
    final actions = _readActions(actionsType, target);

    final stateTypeName = _typeName(stateType);
    final snapshotTypeName = _typeName(snapshotType);
    final actionsTypeName = _typeName(actionsType);
    final engineTypeName = _typeName(engineType);

    return generateOxideStoreSource(
      OxideCodegenConfig(
        prefix: prefix,
        stateType: stateTypeName,
        snapshotType: snapshotTypeName,
        actionsType: actionsTypeName,
        actionsIsEnum: actions.isEnum,
        engineType: engineTypeName,
        sliceType: slices.type,
        slices: slices.values,
        backend: _readBackend(annotation),
        keepAlive: keepAlive,
        createEngine: bindings.createEngine,
        disposeEngine: bindings.disposeEngine,
        dispatch: bindings.dispatch,
        stateStream: bindings.stateStream,
        current: bindings.current,
        initApp: annotation.peek('initApp')?.stringValue,
        encodeCurrentState: annotation.peek('encodeCurrentState')?.stringValue,
        encodeState: annotation.peek('encodeState')?.stringValue,
        decodeState: annotation.peek('decodeState')?.stringValue,
        actionConstructors: actions.constructors,
      ),
    );
  }
}

DartType _readType(ConstantReader annotation, String field) {
  return annotation.read(field).typeValue;
}

ClassElement _readTargetClass(Element element) {
  if (element is! ClassElement) {
    throw InvalidGenerationSourceError(
      '@OxideStore can only be used on classes.',
      element: element,
    );
  }

  final name = element.name;
  if (name == null || name.isEmpty) {
    throw InvalidGenerationSourceError(
      '@OxideStore can only be used on named classes.',
      element: element,
    );
  }

  return element;
}

String _resolvePrefix(ConstantReader annotation, ClassElement element) {
  final override = annotation.peek('name')?.stringValue;
  if (override != null && override.isNotEmpty) {
    return override;
  }
  return element.name!;
}

_Bindings _readBindings(ConstantReader annotation) {
  final scope = annotation.peek('bindings')?.stringValue;
  return _Bindings(
    createEngine: _resolveBinding(
      annotation.read('createEngine').stringValue,
      scope,
    ),
    disposeEngine: _resolveBinding(
      annotation.read('disposeEngine').stringValue,
      scope,
    ),
    dispatch: _resolveBinding(annotation.read('dispatch').stringValue, scope),
    stateStream: _resolveBinding(
      annotation.read('stateStream').stringValue,
      scope,
    ),
    current: _resolveBinding(annotation.read('current').stringValue, scope),
  );
}

String _resolveBinding(String value, String? scope) {
  if (scope == null || scope.isEmpty) {
    return value;
  }

  return switch (value) {
    'createEngine' => '$scope.createEngine',
    'disposeEngine' => '$scope.disposeEngine',
    'dispatch' => '$scope.dispatch',
    'stateStream' => '$scope.stateStream',
    'current' => '$scope.current',
    _ => value,
  };
}

String _readBackend(ConstantReader annotation) {
  final index = annotation
      .peek('backend')
      ?.objectValue
      .getField('index')
      ?.toIntValue();
  return switch (index) {
    1 => 'inheritedHooks',
    2 => 'riverpod',
    3 => 'bloc',
    _ => 'inherited',
  };
}

_SliceConfig _readSlices(
  ConstantReader annotation,
  DartType snapshotType,
  Element element,
) {
  final slicesReader = annotation.peek('slices');
  if (slicesReader == null || slicesReader.isNull) {
    return const _SliceConfig.empty();
  }

  final values = slicesReader.listValue;
  if (values.isEmpty) {
    return const _SliceConfig.empty();
  }

  String? sliceType;
  final slices = <String>[];
  final enumConstantsByType = <EnumElement, List<String>>{};
  for (final value in values) {
    final enumType = value.type;
    final enumElement = enumType?.element;
    if (enumType == null || enumElement is! EnumElement) {
      throw InvalidGenerationSourceError(
        '@OxideStore.slices must contain enum values.',
        element: element,
      );
    }

    final enumIndex = value.getField('index')?.toIntValue();
    if (enumIndex == null) {
      throw InvalidGenerationSourceError(
        '@OxideStore.slices must contain enum values.',
        element: element,
      );
    }

    final constantNames = enumConstantsByType.putIfAbsent(
      enumElement,
      () => _enumConstantNames(enumElement),
    );
    if (enumIndex < 0 || enumIndex >= constantNames.length) {
      throw InvalidGenerationSourceError(
        '@OxideStore.slices contains an unknown enum value.',
        element: element,
      );
    }

    final enumTypeName = _typeName(enumType);
    sliceType ??= enumTypeName;
    if (sliceType != enumTypeName) {
      throw InvalidGenerationSourceError(
        '@OxideStore.slices must contain values from a single enum type.',
        element: element,
      );
    }

    slices.add('$enumTypeName.${constantNames[enumIndex]}');
  }

  final snapshotElement = snapshotType.element;
  if (snapshotElement is! ClassElement) {
    throw InvalidGenerationSourceError(
      '@OxideStore.snapshot must be a class type when using slices.',
      element: element,
    );
  }

  final hasSlicesMember =
      snapshotElement.getGetter('slices') != null ||
      snapshotElement.fields.any((field) => field.name == 'slices');
  if (!hasSlicesMember) {
    throw InvalidGenerationSourceError(
      '@OxideStore.slices requires snapshot to expose a `slices` field or getter.',
      element: element,
    );
  }

  return _SliceConfig(type: sliceType, values: slices);
}

_ActionsConfig _readActions(DartType actionsType, Element element) {
  final actionsElement = actionsType.element;
  if (actionsElement is EnumElement) {
    return _ActionsConfig(
      isEnum: true,
      constructors: _enumConstantNames(actionsElement)
          .map(
            (name) => OxideActionConstructor(
              name: name,
              positionalParams: const [],
              namedParams: const [],
            ),
          )
          .toList(growable: false),
    );
  }

  if (actionsElement is! ClassElement) {
    throw InvalidGenerationSourceError(
      '@OxideStore.actions must be a class or enum type.',
      element: element,
    );
  }

  return _ActionsConfig(
    isEnum: false,
    constructors: actionsElement.constructors
        .where((ctor) => ctor.isFactory && !ctor.isPrivate)
        .map(_toActionConstructor)
        .where((ctor) => ctor.name.isNotEmpty)
        .toList(growable: false),
  );
}

OxideActionConstructor _toActionConstructor(ConstructorElement ctor) {
  return OxideActionConstructor(
    name: ctor.name ?? '',
    positionalParams: ctor.formalParameters
        .where(
          (param) => param.isPositional && (param.name?.isNotEmpty ?? false),
        )
        .map(_toPositionalParam)
        .toList(growable: false),
    namedParams: ctor.formalParameters
        .where((param) => param.isNamed && (param.name?.isNotEmpty ?? false))
        .map(_toNamedParam)
        .toList(growable: false),
  );
}

OxideActionParam _toPositionalParam(FormalParameterElement param) {
  return OxideActionParam(
    name: param.name!,
    type: param.type.getDisplayString(withNullability: true),
    isRequiredNamed: false,
  );
}

OxideActionParam _toNamedParam(FormalParameterElement param) {
  return OxideActionParam(
    name: param.name!,
    type: param.type.getDisplayString(withNullability: true),
    isRequiredNamed: param.isRequiredNamed,
  );
}

List<String> _enumConstantNames(EnumElement element) {
  final constants = _readEnumConstants(element);
  if (constants != null) {
    return constants
        .map((constant) {
          final dynamic dynamicConstant = constant;
          final name = dynamicConstant.name;
          return name is String ? name : null;
        })
        .whereType<String>()
        .where((name) => name.isNotEmpty)
        .toList(growable: false);
  }

  return element.fields
      .where((field) => field.isEnumConstant)
      .map((field) => field.name)
      .whereType<String>()
      .toList(growable: false);
}

List<dynamic>? _readEnumConstants(EnumElement element) {
  final dynamic dynamicElement = element;
  try {
    final constants = dynamicElement.constants;
    return constants is List ? constants : null;
  } catch (_) {
    return null;
  }
}

String _typeName(DartType type) {
  return type.getDisplayString(withNullability: false);
}

final class _Bindings {
  const _Bindings({
    required this.createEngine,
    required this.disposeEngine,
    required this.dispatch,
    required this.stateStream,
    required this.current,
  });

  final String createEngine;
  final String disposeEngine;
  final String dispatch;
  final String stateStream;
  final String current;
}

final class _SliceConfig {
  const _SliceConfig({required this.type, required this.values});

  const _SliceConfig.empty() : type = null, values = null;

  final String? type;
  final List<String>? values;
}

final class _ActionsConfig {
  const _ActionsConfig({required this.isEnum, required this.constructors});

  final bool isEnum;
  final List<OxideActionConstructor> constructors;
}
