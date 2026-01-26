// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'app_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$AppAction {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AppActionCopyWith<$Res> {
  factory $AppActionCopyWith(AppAction value, $Res Function(AppAction) then) =
      _$AppActionCopyWithImpl<$Res, AppAction>;
}

/// @nodoc
class _$AppActionCopyWithImpl<$Res, $Val extends AppAction>
    implements $AppActionCopyWith<$Res> {
  _$AppActionCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$AppAction_StartTickerImplCopyWith<$Res> {
  factory _$$AppAction_StartTickerImplCopyWith(
    _$AppAction_StartTickerImpl value,
    $Res Function(_$AppAction_StartTickerImpl) then,
  ) = __$$AppAction_StartTickerImplCopyWithImpl<$Res>;
  @useResult
  $Res call({BigInt intervalMs});
}

/// @nodoc
class __$$AppAction_StartTickerImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_StartTickerImpl>
    implements _$$AppAction_StartTickerImplCopyWith<$Res> {
  __$$AppAction_StartTickerImplCopyWithImpl(
    _$AppAction_StartTickerImpl _value,
    $Res Function(_$AppAction_StartTickerImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? intervalMs = null}) {
    return _then(
      _$AppAction_StartTickerImpl(
        intervalMs: null == intervalMs
            ? _value.intervalMs
            : intervalMs // ignore: cast_nullable_to_non_nullable
                  as BigInt,
      ),
    );
  }
}

/// @nodoc

class _$AppAction_StartTickerImpl extends AppAction_StartTicker {
  const _$AppAction_StartTickerImpl({required this.intervalMs}) : super._();

  @override
  final BigInt intervalMs;

  @override
  String toString() {
    return 'AppAction.startTicker(intervalMs: $intervalMs)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_StartTickerImpl &&
            (identical(other.intervalMs, intervalMs) ||
                other.intervalMs == intervalMs));
  }

  @override
  int get hashCode => Object.hash(runtimeType, intervalMs);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppAction_StartTickerImplCopyWith<_$AppAction_StartTickerImpl>
  get copyWith =>
      __$$AppAction_StartTickerImplCopyWithImpl<_$AppAction_StartTickerImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return startTicker(intervalMs);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return startTicker?.call(intervalMs);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (startTicker != null) {
      return startTicker(intervalMs);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return startTicker(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return startTicker?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (startTicker != null) {
      return startTicker(this);
    }
    return orElse();
  }
}

abstract class AppAction_StartTicker extends AppAction {
  const factory AppAction_StartTicker({required final BigInt intervalMs}) =
      _$AppAction_StartTickerImpl;
  const AppAction_StartTicker._() : super._();

  BigInt get intervalMs;

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppAction_StartTickerImplCopyWith<_$AppAction_StartTickerImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AppAction_StopTickerImplCopyWith<$Res> {
  factory _$$AppAction_StopTickerImplCopyWith(
    _$AppAction_StopTickerImpl value,
    $Res Function(_$AppAction_StopTickerImpl) then,
  ) = __$$AppAction_StopTickerImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppAction_StopTickerImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_StopTickerImpl>
    implements _$$AppAction_StopTickerImplCopyWith<$Res> {
  __$$AppAction_StopTickerImplCopyWithImpl(
    _$AppAction_StopTickerImpl _value,
    $Res Function(_$AppAction_StopTickerImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppAction_StopTickerImpl extends AppAction_StopTicker {
  const _$AppAction_StopTickerImpl() : super._();

  @override
  String toString() {
    return 'AppAction.stopTicker()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_StopTickerImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return stopTicker();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return stopTicker?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (stopTicker != null) {
      return stopTicker();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return stopTicker(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return stopTicker?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (stopTicker != null) {
      return stopTicker(this);
    }
    return orElse();
  }
}

abstract class AppAction_StopTicker extends AppAction {
  const factory AppAction_StopTicker() = _$AppAction_StopTickerImpl;
  const AppAction_StopTicker._() : super._();
}

/// @nodoc
abstract class _$$AppAction_AutoTickImplCopyWith<$Res> {
  factory _$$AppAction_AutoTickImplCopyWith(
    _$AppAction_AutoTickImpl value,
    $Res Function(_$AppAction_AutoTickImpl) then,
  ) = __$$AppAction_AutoTickImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppAction_AutoTickImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_AutoTickImpl>
    implements _$$AppAction_AutoTickImplCopyWith<$Res> {
  __$$AppAction_AutoTickImplCopyWithImpl(
    _$AppAction_AutoTickImpl _value,
    $Res Function(_$AppAction_AutoTickImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppAction_AutoTickImpl extends AppAction_AutoTick {
  const _$AppAction_AutoTickImpl() : super._();

  @override
  String toString() {
    return 'AppAction.autoTick()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$AppAction_AutoTickImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return autoTick();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return autoTick?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (autoTick != null) {
      return autoTick();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return autoTick(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return autoTick?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (autoTick != null) {
      return autoTick(this);
    }
    return orElse();
  }
}

abstract class AppAction_AutoTick extends AppAction {
  const factory AppAction_AutoTick() = _$AppAction_AutoTickImpl;
  const AppAction_AutoTick._() : super._();
}

/// @nodoc
abstract class _$$AppAction_ManualTickImplCopyWith<$Res> {
  factory _$$AppAction_ManualTickImplCopyWith(
    _$AppAction_ManualTickImpl value,
    $Res Function(_$AppAction_ManualTickImpl) then,
  ) = __$$AppAction_ManualTickImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppAction_ManualTickImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_ManualTickImpl>
    implements _$$AppAction_ManualTickImplCopyWith<$Res> {
  __$$AppAction_ManualTickImplCopyWithImpl(
    _$AppAction_ManualTickImpl _value,
    $Res Function(_$AppAction_ManualTickImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppAction_ManualTickImpl extends AppAction_ManualTick {
  const _$AppAction_ManualTickImpl() : super._();

  @override
  String toString() {
    return 'AppAction.manualTick()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_ManualTickImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return manualTick();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return manualTick?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (manualTick != null) {
      return manualTick();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return manualTick(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return manualTick?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (manualTick != null) {
      return manualTick(this);
    }
    return orElse();
  }
}

abstract class AppAction_ManualTick extends AppAction {
  const factory AppAction_ManualTick() = _$AppAction_ManualTickImpl;
  const AppAction_ManualTick._() : super._();
}

/// @nodoc
abstract class _$$AppAction_EmitSideEffectTickImplCopyWith<$Res> {
  factory _$$AppAction_EmitSideEffectTickImplCopyWith(
    _$AppAction_EmitSideEffectTickImpl value,
    $Res Function(_$AppAction_EmitSideEffectTickImpl) then,
  ) = __$$AppAction_EmitSideEffectTickImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppAction_EmitSideEffectTickImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_EmitSideEffectTickImpl>
    implements _$$AppAction_EmitSideEffectTickImplCopyWith<$Res> {
  __$$AppAction_EmitSideEffectTickImplCopyWithImpl(
    _$AppAction_EmitSideEffectTickImpl _value,
    $Res Function(_$AppAction_EmitSideEffectTickImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppAction_EmitSideEffectTickImpl extends AppAction_EmitSideEffectTick {
  const _$AppAction_EmitSideEffectTickImpl() : super._();

  @override
  String toString() {
    return 'AppAction.emitSideEffectTick()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_EmitSideEffectTickImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return emitSideEffectTick();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return emitSideEffectTick?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (emitSideEffectTick != null) {
      return emitSideEffectTick();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return emitSideEffectTick(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return emitSideEffectTick?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (emitSideEffectTick != null) {
      return emitSideEffectTick(this);
    }
    return orElse();
  }
}

abstract class AppAction_EmitSideEffectTick extends AppAction {
  const factory AppAction_EmitSideEffectTick() =
      _$AppAction_EmitSideEffectTickImpl;
  const AppAction_EmitSideEffectTick._() : super._();
}

/// @nodoc
abstract class _$$AppAction_ResetImplCopyWith<$Res> {
  factory _$$AppAction_ResetImplCopyWith(
    _$AppAction_ResetImpl value,
    $Res Function(_$AppAction_ResetImpl) then,
  ) = __$$AppAction_ResetImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AppAction_ResetImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_ResetImpl>
    implements _$$AppAction_ResetImplCopyWith<$Res> {
  __$$AppAction_ResetImplCopyWithImpl(
    _$AppAction_ResetImpl _value,
    $Res Function(_$AppAction_ResetImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc

class _$AppAction_ResetImpl extends AppAction_Reset {
  const _$AppAction_ResetImpl() : super._();

  @override
  String toString() {
    return 'AppAction.reset()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$AppAction_ResetImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(BigInt intervalMs) startTicker,
    required TResult Function() stopTicker,
    required TResult Function() autoTick,
    required TResult Function() manualTick,
    required TResult Function() emitSideEffectTick,
    required TResult Function() reset,
  }) {
    return reset();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(BigInt intervalMs)? startTicker,
    TResult? Function()? stopTicker,
    TResult? Function()? autoTick,
    TResult? Function()? manualTick,
    TResult? Function()? emitSideEffectTick,
    TResult? Function()? reset,
  }) {
    return reset?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(BigInt intervalMs)? startTicker,
    TResult Function()? stopTicker,
    TResult Function()? autoTick,
    TResult Function()? manualTick,
    TResult Function()? emitSideEffectTick,
    TResult Function()? reset,
    required TResult orElse(),
  }) {
    if (reset != null) {
      return reset();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_StartTicker value) startTicker,
    required TResult Function(AppAction_StopTicker value) stopTicker,
    required TResult Function(AppAction_AutoTick value) autoTick,
    required TResult Function(AppAction_ManualTick value) manualTick,
    required TResult Function(AppAction_EmitSideEffectTick value)
    emitSideEffectTick,
    required TResult Function(AppAction_Reset value) reset,
  }) {
    return reset(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_StartTicker value)? startTicker,
    TResult? Function(AppAction_StopTicker value)? stopTicker,
    TResult? Function(AppAction_AutoTick value)? autoTick,
    TResult? Function(AppAction_ManualTick value)? manualTick,
    TResult? Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult? Function(AppAction_Reset value)? reset,
  }) {
    return reset?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_StartTicker value)? startTicker,
    TResult Function(AppAction_StopTicker value)? stopTicker,
    TResult Function(AppAction_AutoTick value)? autoTick,
    TResult Function(AppAction_ManualTick value)? manualTick,
    TResult Function(AppAction_EmitSideEffectTick value)? emitSideEffectTick,
    TResult Function(AppAction_Reset value)? reset,
    required TResult orElse(),
  }) {
    if (reset != null) {
      return reset(this);
    }
    return orElse();
  }
}

abstract class AppAction_Reset extends AppAction {
  const factory AppAction_Reset() = _$AppAction_ResetImpl;
  const AppAction_Reset._() : super._();
}
