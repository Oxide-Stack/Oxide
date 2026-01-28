// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'counter_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$CounterAction {

 int get iterations;
/// Create a copy of CounterAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterActionCopyWith<CounterAction> get copyWith => _$CounterActionCopyWithImpl<CounterAction>(this as CounterAction, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterAction&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'CounterAction(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $CounterActionCopyWith<$Res>  {
  factory $CounterActionCopyWith(CounterAction value, $Res Function(CounterAction) _then) = _$CounterActionCopyWithImpl;
@useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$CounterActionCopyWithImpl<$Res>
    implements $CounterActionCopyWith<$Res> {
  _$CounterActionCopyWithImpl(this._self, this._then);

  final CounterAction _self;
  final $Res Function(CounterAction) _then;

/// Create a copy of CounterAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? iterations = null,}) {
  return _then(_self.copyWith(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterAction].
extension CounterActionPatterns on CounterAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterAction_Run value)?  run,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterAction_Run() when run != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterAction_Run value)  run,}){
final _that = this;
switch (_that) {
case CounterAction_Run():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterAction_Run value)?  run,}){
final _that = this;
switch (_that) {
case CounterAction_Run() when run != null:
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
case CounterAction_Run() when run != null:
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
case CounterAction_Run():
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
case CounterAction_Run() when run != null:
return run(_that.iterations);case _:
  return null;

}
}

}

/// @nodoc


class CounterAction_Run extends CounterAction {
  const CounterAction_Run({required this.iterations}): super._();
  

@override final  int iterations;

/// Create a copy of CounterAction
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterAction_RunCopyWith<CounterAction_Run> get copyWith => _$CounterAction_RunCopyWithImpl<CounterAction_Run>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterAction_Run&&(identical(other.iterations, iterations) || other.iterations == iterations));
}


@override
int get hashCode => Object.hash(runtimeType,iterations);

@override
String toString() {
  return 'CounterAction.run(iterations: $iterations)';
}


}

/// @nodoc
abstract mixin class $CounterAction_RunCopyWith<$Res> implements $CounterActionCopyWith<$Res> {
  factory $CounterAction_RunCopyWith(CounterAction_Run value, $Res Function(CounterAction_Run) _then) = _$CounterAction_RunCopyWithImpl;
@override @useResult
$Res call({
 int iterations
});




}
/// @nodoc
class _$CounterAction_RunCopyWithImpl<$Res>
    implements $CounterAction_RunCopyWith<$Res> {
  _$CounterAction_RunCopyWithImpl(this._self, this._then);

  final CounterAction_Run _self;
  final $Res Function(CounterAction_Run) _then;

/// Create a copy of CounterAction
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? iterations = null,}) {
  return _then(CounterAction_Run(
iterations: null == iterations ? _self.iterations : iterations // ignore: cast_nullable_to_non_nullable
as int,
  ));
}


}

// dart format on
