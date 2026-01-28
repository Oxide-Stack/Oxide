// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'sieve_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$SieveAction {

 int get iterations;
/// Create a copy of SieveAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SieveActionCopyWith<SieveAction> get copyWith => _$SieveActionCopyWithImpl<SieveAction>(this as SieveAction, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SieveAction&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'SieveAction(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $SieveActionCopyWith<$Res>  {
  factory $SieveActionCopyWith(SieveAction value, $Res Function(SieveAction) _then) = _$SieveActionCopyWithImpl;
@useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$SieveActionCopyWithImpl<$Res>
    implements $SieveActionCopyWith<$Res> {
  _$SieveActionCopyWithImpl(this._self, this._then);

  final SieveAction _self;
  final $Res Function(SieveAction) _then;

/// Create a copy of SieveAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? iterations = null,}) {
  return _then(_self.copyWith(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [SieveAction].
extension SieveActionPatterns on SieveAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( SieveAction_Run value)?  run,required TResult orElse(),}){
final _that = this;
switch (_that) {
case SieveAction_Run() when run != null:
return run(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( SieveAction_Run value)  run,}){
final _that = this;
switch (_that) {
case SieveAction_Run():
return run(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( SieveAction_Run value)?  run,}){
final _that = this;
switch (_that) {
case SieveAction_Run() when run != null:
return run(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( int iterations)?  run,required TResult orElse(),}) {final _that = this;
switch (_that) {
case SieveAction_Run() when run != null:
return run(_that.iterations);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( int iterations)  run,}) {final _that = this;
switch (_that) {
case SieveAction_Run():
return run(_that.iterations);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( int iterations)?  run,}) {final _that = this;
switch (_that) {
case SieveAction_Run() when run != null:
return run(_that.iterations);case _:
  return null;

}
}

}

/// @nodoc


class SieveAction_Run extends SieveAction {
  const SieveAction_Run({required this.iterations}): super._();
  

@override final  int iterations;

/// Create a copy of SieveAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SieveAction_RunCopyWith<SieveAction_Run> get copyWith => _$SieveAction_RunCopyWithImpl<SieveAction_Run>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SieveAction_Run&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'SieveAction.run(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $SieveAction_RunCopyWith<$Res> implements $SieveActionCopyWith<$Res> {
  factory $SieveAction_RunCopyWith(SieveAction_Run value, $Res Function(SieveAction_Run) _then) = _$SieveAction_RunCopyWithImpl;
@override @useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$SieveAction_RunCopyWithImpl<$Res>
    implements $SieveAction_RunCopyWith<$Res> {
  _$SieveAction_RunCopyWithImpl(this._self, this._then);

  final SieveAction_Run _self;
  final $Res Function(SieveAction_Run) _then;

/// Create a copy of SieveAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? iterations = null,}) {
  return _then(SieveAction_Run(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
