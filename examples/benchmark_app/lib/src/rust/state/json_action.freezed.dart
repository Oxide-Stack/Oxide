// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'json_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$JsonAction {

 int get iterations;
/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$JsonActionCopyWith<JsonAction> get copyWith => _$JsonActionCopyWithImpl<JsonAction>(this as JsonAction, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JsonAction&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'JsonAction(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $JsonActionCopyWith<$Res>  {
  factory $JsonActionCopyWith(JsonAction value, $Res Function(JsonAction) _then) = _$JsonActionCopyWithImpl;
@useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$JsonActionCopyWithImpl<$Res>
    implements $JsonActionCopyWith<$Res> {
  _$JsonActionCopyWithImpl(this._self, this._then);

  final JsonAction _self;
  final $Res Function(JsonAction) _then;

/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? iterations = null,}) {
  return _then(_self.copyWith(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [JsonAction].
extension JsonActionPatterns on JsonAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( JsonAction_RunLight value)?  runLight,TResult Function( JsonAction_RunHeavy value)?  runHeavy,required TResult orElse(),}){
final _that = this;
switch (_that) {
case JsonAction_RunLight() when runLight != null:
return runLight(_that);case JsonAction_RunHeavy() when runHeavy != null:
return runHeavy(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( JsonAction_RunLight value)  runLight,required TResult Function( JsonAction_RunHeavy value)  runHeavy,}){
final _that = this;
switch (_that) {
case JsonAction_RunLight():
return runLight(_that);case JsonAction_RunHeavy():
return runHeavy(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( JsonAction_RunLight value)?  runLight,TResult? Function( JsonAction_RunHeavy value)?  runHeavy,}){
final _that = this;
switch (_that) {
case JsonAction_RunLight() when runLight != null:
return runLight(_that);case JsonAction_RunHeavy() when runHeavy != null:
return runHeavy(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( int iterations)?  runLight,TResult Function( int iterations)?  runHeavy,required TResult orElse(),}) {final _that = this;
switch (_that) {
case JsonAction_RunLight() when runLight != null:
return runLight(_that.iterations);case JsonAction_RunHeavy() when runHeavy != null:
return runHeavy(_that.iterations);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( int iterations)  runLight,required TResult Function( int iterations)  runHeavy,}) {final _that = this;
switch (_that) {
case JsonAction_RunLight():
return runLight(_that.iterations);case JsonAction_RunHeavy():
return runHeavy(_that.iterations);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( int iterations)?  runLight,TResult? Function( int iterations)?  runHeavy,}) {final _that = this;
switch (_that) {
case JsonAction_RunLight() when runLight != null:
return runLight(_that.iterations);case JsonAction_RunHeavy() when runHeavy != null:
return runHeavy(_that.iterations);case _:
  return null;

}
}

}

/// @nodoc


class JsonAction_RunLight extends JsonAction {
  const JsonAction_RunLight({required this.iterations}): super._();
  

@override final  int iterations;

/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$JsonAction_RunLightCopyWith<JsonAction_RunLight> get copyWith => _$JsonAction_RunLightCopyWithImpl<JsonAction_RunLight>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JsonAction_RunLight&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'JsonAction.runLight(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $JsonAction_RunLightCopyWith<$Res> implements $JsonActionCopyWith<$Res> {
  factory $JsonAction_RunLightCopyWith(JsonAction_RunLight value, $Res Function(JsonAction_RunLight) _then) = _$JsonAction_RunLightCopyWithImpl;
@override @useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$JsonAction_RunLightCopyWithImpl<$Res>
    implements $JsonAction_RunLightCopyWith<$Res> {
  _$JsonAction_RunLightCopyWithImpl(this._self, this._then);

  final JsonAction_RunLight _self;
  final $Res Function(JsonAction_RunLight) _then;

/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? iterations = null,}) {
  return _then(JsonAction_RunLight(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

/// @nodoc


class JsonAction_RunHeavy extends JsonAction {
  const JsonAction_RunHeavy({required this.iterations}): super._();
  

@override final  int iterations;

/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$JsonAction_RunHeavyCopyWith<JsonAction_RunHeavy> get copyWith => _$JsonAction_RunHeavyCopyWithImpl<JsonAction_RunHeavy>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is JsonAction_RunHeavy&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'JsonAction.runHeavy(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $JsonAction_RunHeavyCopyWith<$Res> implements $JsonActionCopyWith<$Res> {
  factory $JsonAction_RunHeavyCopyWith(JsonAction_RunHeavy value, $Res Function(JsonAction_RunHeavy) _then) = _$JsonAction_RunHeavyCopyWithImpl;
@override @useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$JsonAction_RunHeavyCopyWithImpl<$Res>
    implements $JsonAction_RunHeavyCopyWith<$Res> {
  _$JsonAction_RunHeavyCopyWithImpl(this._self, this._then);

  final JsonAction_RunHeavy _self;
  final $Res Function(JsonAction_RunHeavy) _then;

/// Create a copy of JsonAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? iterations = null,}) {
  return _then(JsonAction_RunHeavy(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
