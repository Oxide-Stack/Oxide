// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'json_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$JsonAction {
  int get iterations => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int iterations) runLight,
    required TResult Function(int iterations) runHeavy,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int iterations)? runLight,
    TResult? Function(int iterations)? runHeavy,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int iterations)? runLight,
    TResult Function(int iterations)? runHeavy,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JsonAction_RunLight value) runLight,
    required TResult Function(JsonAction_RunHeavy value) runHeavy,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JsonAction_RunLight value)? runLight,
    TResult? Function(JsonAction_RunHeavy value)? runHeavy,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JsonAction_RunLight value)? runLight,
    TResult Function(JsonAction_RunHeavy value)? runHeavy,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $JsonActionCopyWith<JsonAction> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $JsonActionCopyWith<$Res> {
  factory $JsonActionCopyWith(
    JsonAction value,
    $Res Function(JsonAction) then,
  ) = _$JsonActionCopyWithImpl<$Res, JsonAction>;
  @useResult
  $Res call({int iterations});
}

/// @nodoc
class _$JsonActionCopyWithImpl<$Res, $Val extends JsonAction>
    implements $JsonActionCopyWith<$Res> {
  _$JsonActionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? iterations = null}) {
    return _then(
      _value.copyWith(
            iterations: null == iterations
                ? _value.iterations
                : iterations // ignore: cast_nullable_to_non_nullable
                      as int,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$JsonAction_RunLightImplCopyWith<$Res>
    implements $JsonActionCopyWith<$Res> {
  factory _$$JsonAction_RunLightImplCopyWith(
    _$JsonAction_RunLightImpl value,
    $Res Function(_$JsonAction_RunLightImpl) then,
  ) = __$$JsonAction_RunLightImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int iterations});
}

/// @nodoc
class __$$JsonAction_RunLightImplCopyWithImpl<$Res>
    extends _$JsonActionCopyWithImpl<$Res, _$JsonAction_RunLightImpl>
    implements _$$JsonAction_RunLightImplCopyWith<$Res> {
  __$$JsonAction_RunLightImplCopyWithImpl(
    _$JsonAction_RunLightImpl _value,
    $Res Function(_$JsonAction_RunLightImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? iterations = null}) {
    return _then(
      _$JsonAction_RunLightImpl(
        iterations: null == iterations
            ? _value.iterations
            : iterations // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$JsonAction_RunLightImpl extends JsonAction_RunLight {
  const _$JsonAction_RunLightImpl({required this.iterations}) : super._();

  @override
  final int iterations;

  @override
  String toString() {
    return 'JsonAction.runLight(iterations: $iterations)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JsonAction_RunLightImpl &&
            (identical(other.iterations, iterations) ||
                other.iterations == iterations));
  }

  @override
  int get hashCode => Object.hash(runtimeType, iterations);

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JsonAction_RunLightImplCopyWith<_$JsonAction_RunLightImpl> get copyWith =>
      __$$JsonAction_RunLightImplCopyWithImpl<_$JsonAction_RunLightImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int iterations) runLight,
    required TResult Function(int iterations) runHeavy,
  }) {
    return runLight(iterations);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int iterations)? runLight,
    TResult? Function(int iterations)? runHeavy,
  }) {
    return runLight?.call(iterations);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int iterations)? runLight,
    TResult Function(int iterations)? runHeavy,
    required TResult orElse(),
  }) {
    if (runLight != null) {
      return runLight(iterations);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JsonAction_RunLight value) runLight,
    required TResult Function(JsonAction_RunHeavy value) runHeavy,
  }) {
    return runLight(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JsonAction_RunLight value)? runLight,
    TResult? Function(JsonAction_RunHeavy value)? runHeavy,
  }) {
    return runLight?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JsonAction_RunLight value)? runLight,
    TResult Function(JsonAction_RunHeavy value)? runHeavy,
    required TResult orElse(),
  }) {
    if (runLight != null) {
      return runLight(this);
    }
    return orElse();
  }
}

abstract class JsonAction_RunLight extends JsonAction {
  const factory JsonAction_RunLight({required final int iterations}) =
      _$JsonAction_RunLightImpl;
  const JsonAction_RunLight._() : super._();

  @override
  int get iterations;

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JsonAction_RunLightImplCopyWith<_$JsonAction_RunLightImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$JsonAction_RunHeavyImplCopyWith<$Res>
    implements $JsonActionCopyWith<$Res> {
  factory _$$JsonAction_RunHeavyImplCopyWith(
    _$JsonAction_RunHeavyImpl value,
    $Res Function(_$JsonAction_RunHeavyImpl) then,
  ) = __$$JsonAction_RunHeavyImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int iterations});
}

/// @nodoc
class __$$JsonAction_RunHeavyImplCopyWithImpl<$Res>
    extends _$JsonActionCopyWithImpl<$Res, _$JsonAction_RunHeavyImpl>
    implements _$$JsonAction_RunHeavyImplCopyWith<$Res> {
  __$$JsonAction_RunHeavyImplCopyWithImpl(
    _$JsonAction_RunHeavyImpl _value,
    $Res Function(_$JsonAction_RunHeavyImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? iterations = null}) {
    return _then(
      _$JsonAction_RunHeavyImpl(
        iterations: null == iterations
            ? _value.iterations
            : iterations // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$JsonAction_RunHeavyImpl extends JsonAction_RunHeavy {
  const _$JsonAction_RunHeavyImpl({required this.iterations}) : super._();

  @override
  final int iterations;

  @override
  String toString() {
    return 'JsonAction.runHeavy(iterations: $iterations)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$JsonAction_RunHeavyImpl &&
            (identical(other.iterations, iterations) ||
                other.iterations == iterations));
  }

  @override
  int get hashCode => Object.hash(runtimeType, iterations);

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$JsonAction_RunHeavyImplCopyWith<_$JsonAction_RunHeavyImpl> get copyWith =>
      __$$JsonAction_RunHeavyImplCopyWithImpl<_$JsonAction_RunHeavyImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int iterations) runLight,
    required TResult Function(int iterations) runHeavy,
  }) {
    return runHeavy(iterations);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int iterations)? runLight,
    TResult? Function(int iterations)? runHeavy,
  }) {
    return runHeavy?.call(iterations);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int iterations)? runLight,
    TResult Function(int iterations)? runHeavy,
    required TResult orElse(),
  }) {
    if (runHeavy != null) {
      return runHeavy(iterations);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(JsonAction_RunLight value) runLight,
    required TResult Function(JsonAction_RunHeavy value) runHeavy,
  }) {
    return runHeavy(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(JsonAction_RunLight value)? runLight,
    TResult? Function(JsonAction_RunHeavy value)? runHeavy,
  }) {
    return runHeavy?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(JsonAction_RunLight value)? runLight,
    TResult Function(JsonAction_RunHeavy value)? runHeavy,
    required TResult orElse(),
  }) {
    if (runHeavy != null) {
      return runHeavy(this);
    }
    return orElse();
  }
}

abstract class JsonAction_RunHeavy extends JsonAction {
  const factory JsonAction_RunHeavy({required final int iterations}) =
      _$JsonAction_RunHeavyImpl;
  const JsonAction_RunHeavy._() : super._();

  @override
  int get iterations;

  /// Create a copy of JsonAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$JsonAction_RunHeavyImplCopyWith<_$JsonAction_RunHeavyImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
