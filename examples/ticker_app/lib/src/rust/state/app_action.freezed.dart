// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'app_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$AppAction {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction()';
}


}

/// @nodoc
class $AppActionCopyWith<$Res>  {
$AppActionCopyWith(AppAction _, $Res Function(AppAction) __);
}


/// Adds pattern-matching-related methods to [AppAction].
extension AppActionPatterns on AppAction {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AppAction_StartTicker value)?  startTicker,TResult Function( AppAction_StopTicker value)?  stopTicker,TResult Function( AppAction_AutoTick value)?  autoTick,TResult Function( AppAction_ManualTick value)?  manualTick,TResult Function( AppAction_EmitSideEffectTick value)?  emitSideEffectTick,TResult Function( AppAction_Reset value)?  reset,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AppAction_StartTicker() when startTicker != null:
return startTicker(_that);case AppAction_StopTicker() when stopTicker != null:
return stopTicker(_that);case AppAction_AutoTick() when autoTick != null:
return autoTick(_that);case AppAction_ManualTick() when manualTick != null:
return manualTick(_that);case AppAction_EmitSideEffectTick() when emitSideEffectTick != null:
return emitSideEffectTick(_that);case AppAction_Reset() when reset != null:
return reset(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AppAction_StartTicker value)  startTicker,required TResult Function( AppAction_StopTicker value)  stopTicker,required TResult Function( AppAction_AutoTick value)  autoTick,required TResult Function( AppAction_ManualTick value)  manualTick,required TResult Function( AppAction_EmitSideEffectTick value)  emitSideEffectTick,required TResult Function( AppAction_Reset value)  reset,}){
final _that = this;
switch (_that) {
case AppAction_StartTicker():
return startTicker(_that);case AppAction_StopTicker():
return stopTicker(_that);case AppAction_AutoTick():
return autoTick(_that);case AppAction_ManualTick():
return manualTick(_that);case AppAction_EmitSideEffectTick():
return emitSideEffectTick(_that);case AppAction_Reset():
return reset(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AppAction_StartTicker value)?  startTicker,TResult? Function( AppAction_StopTicker value)?  stopTicker,TResult? Function( AppAction_AutoTick value)?  autoTick,TResult? Function( AppAction_ManualTick value)?  manualTick,TResult? Function( AppAction_EmitSideEffectTick value)?  emitSideEffectTick,TResult? Function( AppAction_Reset value)?  reset,}){
final _that = this;
switch (_that) {
case AppAction_StartTicker() when startTicker != null:
return startTicker(_that);case AppAction_StopTicker() when stopTicker != null:
return stopTicker(_that);case AppAction_AutoTick() when autoTick != null:
return autoTick(_that);case AppAction_ManualTick() when manualTick != null:
return manualTick(_that);case AppAction_EmitSideEffectTick() when emitSideEffectTick != null:
return emitSideEffectTick(_that);case AppAction_Reset() when reset != null:
return reset(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt intervalMs)?  startTicker,TResult Function()?  stopTicker,TResult Function()?  autoTick,TResult Function()?  manualTick,TResult Function()?  emitSideEffectTick,TResult Function()?  reset,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AppAction_StartTicker() when startTicker != null:
return startTicker(_that.intervalMs);case AppAction_StopTicker() when stopTicker != null:
return stopTicker();case AppAction_AutoTick() when autoTick != null:
return autoTick();case AppAction_ManualTick() when manualTick != null:
return manualTick();case AppAction_EmitSideEffectTick() when emitSideEffectTick != null:
return emitSideEffectTick();case AppAction_Reset() when reset != null:
return reset();case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt intervalMs)  startTicker,required TResult Function()  stopTicker,required TResult Function()  autoTick,required TResult Function()  manualTick,required TResult Function()  emitSideEffectTick,required TResult Function()  reset,}) {final _that = this;
switch (_that) {
case AppAction_StartTicker():
return startTicker(_that.intervalMs);case AppAction_StopTicker():
return stopTicker();case AppAction_AutoTick():
return autoTick();case AppAction_ManualTick():
return manualTick();case AppAction_EmitSideEffectTick():
return emitSideEffectTick();case AppAction_Reset():
return reset();}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt intervalMs)?  startTicker,TResult? Function()?  stopTicker,TResult? Function()?  autoTick,TResult? Function()?  manualTick,TResult? Function()?  emitSideEffectTick,TResult? Function()?  reset,}) {final _that = this;
switch (_that) {
case AppAction_StartTicker() when startTicker != null:
return startTicker(_that.intervalMs);case AppAction_StopTicker() when stopTicker != null:
return stopTicker();case AppAction_AutoTick() when autoTick != null:
return autoTick();case AppAction_ManualTick() when manualTick != null:
return manualTick();case AppAction_EmitSideEffectTick() when emitSideEffectTick != null:
return emitSideEffectTick();case AppAction_Reset() when reset != null:
return reset();case _:
  return null;

}
}

}

/// @nodoc


class AppAction_StartTicker extends AppAction {
  const AppAction_StartTicker({required this.intervalMs}): super._();
  

 final  BigInt intervalMs;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AppAction_StartTickerCopyWith<AppAction_StartTicker> get copyWith => _$AppAction_StartTickerCopyWithImpl<AppAction_StartTicker>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_StartTicker&&(identical(other.intervalMs, intervalMs) || other.intervalMs == intervalMs));
}


@override
int get hashCode => Object.hash(runtimeType,intervalMs);

@override
String toString() {
  return 'AppAction.startTicker(intervalMs: $intervalMs)';
}


}

/// @nodoc
abstract mixin class $AppAction_StartTickerCopyWith<$Res> implements $AppActionCopyWith<$Res> {
  factory $AppAction_StartTickerCopyWith(AppAction_StartTicker value, $Res Function(AppAction_StartTicker) _then) = _$AppAction_StartTickerCopyWithImpl;
@useResult
$Res call({
 BigInt intervalMs
});




}
/// @nodoc
class _$AppAction_StartTickerCopyWithImpl<$Res>
    implements $AppAction_StartTickerCopyWith<$Res> {
  _$AppAction_StartTickerCopyWithImpl(this._self, this._then);

  final AppAction_StartTicker _self;
  final $Res Function(AppAction_StartTicker) _then;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? intervalMs = null,}) {
  return _then(AppAction_StartTicker(
intervalMs: null == intervalMs ? _self.intervalMs : intervalMs // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class AppAction_StopTicker extends AppAction {
  const AppAction_StopTicker(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_StopTicker);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction.stopTicker()';
}


}




/// @nodoc


class AppAction_AutoTick extends AppAction {
  const AppAction_AutoTick(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_AutoTick);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction.autoTick()';
}


}




/// @nodoc


class AppAction_ManualTick extends AppAction {
  const AppAction_ManualTick(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_ManualTick);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction.manualTick()';
}


}




/// @nodoc


class AppAction_EmitSideEffectTick extends AppAction {
  const AppAction_EmitSideEffectTick(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_EmitSideEffectTick);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction.emitSideEffectTick()';
}


}




/// @nodoc


class AppAction_Reset extends AppAction {
  const AppAction_Reset(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_Reset);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'AppAction.reset()';
}


}




// dart format on
