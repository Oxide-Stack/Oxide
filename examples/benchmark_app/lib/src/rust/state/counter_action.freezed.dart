// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'counter_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$CounterAction {
  int get iterations => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int iterations) run,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int iterations)? run,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int iterations)? run,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CounterAction_Run value) run,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CounterAction_Run value)? run,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CounterAction_Run value)? run,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;

  /// Create a copy of CounterAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $CounterActionCopyWith<CounterAction> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CounterActionCopyWith<$Res> {
  factory $CounterActionCopyWith(
    CounterAction value,
    $Res Function(CounterAction) then,
  ) = _$CounterActionCopyWithImpl<$Res, CounterAction>;
  @useResult
  $Res call({int iterations});
}

/// @nodoc
class _$CounterActionCopyWithImpl<$Res, $Val extends CounterAction>
    implements $CounterActionCopyWith<$Res> {
  _$CounterActionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of CounterAction
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
abstract class _$$CounterAction_RunImplCopyWith<$Res>
    implements $CounterActionCopyWith<$Res> {
  factory _$$CounterAction_RunImplCopyWith(
    _$CounterAction_RunImpl value,
    $Res Function(_$CounterAction_RunImpl) then,
  ) = __$$CounterAction_RunImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({int iterations});
}

/// @nodoc
class __$$CounterAction_RunImplCopyWithImpl<$Res>
    extends _$CounterActionCopyWithImpl<$Res, _$CounterAction_RunImpl>
    implements _$$CounterAction_RunImplCopyWith<$Res> {
  __$$CounterAction_RunImplCopyWithImpl(
    _$CounterAction_RunImpl _value,
    $Res Function(_$CounterAction_RunImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of CounterAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? iterations = null}) {
    return _then(
      _$CounterAction_RunImpl(
        iterations: null == iterations
            ? _value.iterations
            : iterations // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$CounterAction_RunImpl extends CounterAction_Run {
  const _$CounterAction_RunImpl({required this.iterations}) : super._();

  @override
  final int iterations;

  @override
  String toString() {
    return 'CounterAction.run(iterations: $iterations)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$CounterAction_RunImpl &&
            (identical(other.iterations, iterations) ||
                other.iterations == iterations));
  }

  @override
  int get hashCode => Object.hash(runtimeType, iterations);

  /// Create a copy of CounterAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$CounterAction_RunImplCopyWith<_$CounterAction_RunImpl> get copyWith =>
      __$$CounterAction_RunImplCopyWithImpl<_$CounterAction_RunImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(int iterations) run,
  }) {
    return run(iterations);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(int iterations)? run,
  }) {
    return run?.call(iterations);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(int iterations)? run,
    required TResult orElse(),
  }) {
    if (run != null) {
      return run(iterations);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(CounterAction_Run value) run,
  }) {
    return run(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(CounterAction_Run value)? run,
  }) {
    return run?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(CounterAction_Run value)? run,
    required TResult orElse(),
  }) {
    if (run != null) {
      return run(this);
    }
    return orElse();
  }
}

abstract class CounterAction_Run extends CounterAction {
  const factory CounterAction_Run({required final int iterations}) =
      _$CounterAction_RunImpl;
  const CounterAction_Run._() : super._();

  @override
  int get iterations;

  /// Create a copy of CounterAction
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$CounterAction_RunImplCopyWith<_$CounterAction_RunImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
