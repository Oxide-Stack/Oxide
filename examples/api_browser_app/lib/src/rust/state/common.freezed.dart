// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'common.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$LoadPhase {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoadPhase);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoadPhase()';
}


}

/// @nodoc
class $LoadPhaseCopyWith<$Res>  {
$LoadPhaseCopyWith(LoadPhase _, $Res Function(LoadPhase) __);
}


/// Adds pattern-matching-related methods to [LoadPhase].
extension LoadPhasePatterns on LoadPhase {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( LoadPhase_Idle value)?  idle,TResult Function( LoadPhase_Loading value)?  loading,TResult Function( LoadPhase_Ready value)?  ready,TResult Function( LoadPhase_Error value)?  error,required TResult orElse(),}){
final _that = this;
switch (_that) {
case LoadPhase_Idle() when idle != null:
return idle(_that);case LoadPhase_Loading() when loading != null:
return loading(_that);case LoadPhase_Ready() when ready != null:
return ready(_that);case LoadPhase_Error() when error != null:
return error(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( LoadPhase_Idle value)  idle,required TResult Function( LoadPhase_Loading value)  loading,required TResult Function( LoadPhase_Ready value)  ready,required TResult Function( LoadPhase_Error value)  error,}){
final _that = this;
switch (_that) {
case LoadPhase_Idle():
return idle(_that);case LoadPhase_Loading():
return loading(_that);case LoadPhase_Ready():
return ready(_that);case LoadPhase_Error():
return error(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( LoadPhase_Idle value)?  idle,TResult? Function( LoadPhase_Loading value)?  loading,TResult? Function( LoadPhase_Ready value)?  ready,TResult? Function( LoadPhase_Error value)?  error,}){
final _that = this;
switch (_that) {
case LoadPhase_Idle() when idle != null:
return idle(_that);case LoadPhase_Loading() when loading != null:
return loading(_that);case LoadPhase_Ready() when ready != null:
return ready(_that);case LoadPhase_Error() when error != null:
return error(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  idle,TResult Function()?  loading,TResult Function()?  ready,TResult Function( String message)?  error,required TResult orElse(),}) {final _that = this;
switch (_that) {
case LoadPhase_Idle() when idle != null:
return idle();case LoadPhase_Loading() when loading != null:
return loading();case LoadPhase_Ready() when ready != null:
return ready();case LoadPhase_Error() when error != null:
return error(_that.message);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  idle,required TResult Function()  loading,required TResult Function()  ready,required TResult Function( String message)  error,}) {final _that = this;
switch (_that) {
case LoadPhase_Idle():
return idle();case LoadPhase_Loading():
return loading();case LoadPhase_Ready():
return ready();case LoadPhase_Error():
return error(_that.message);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  idle,TResult? Function()?  loading,TResult? Function()?  ready,TResult? Function( String message)?  error,}) {final _that = this;
switch (_that) {
case LoadPhase_Idle() when idle != null:
return idle();case LoadPhase_Loading() when loading != null:
return loading();case LoadPhase_Ready() when ready != null:
return ready();case LoadPhase_Error() when error != null:
return error(_that.message);case _:
  return null;

}
}

}

/// @nodoc


class LoadPhase_Idle extends LoadPhase {
  const LoadPhase_Idle(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoadPhase_Idle);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoadPhase.idle()';
}


}




/// @nodoc


class LoadPhase_Loading extends LoadPhase {
  const LoadPhase_Loading(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoadPhase_Loading);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoadPhase.loading()';
}


}




/// @nodoc


class LoadPhase_Ready extends LoadPhase {
  const LoadPhase_Ready(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoadPhase_Ready);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'LoadPhase.ready()';
}


}




/// @nodoc


class LoadPhase_Error extends LoadPhase {
  const LoadPhase_Error({required this.message}): super._();
  

 final  String message;

/// Create a copy of LoadPhase
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$LoadPhase_ErrorCopyWith<LoadPhase_Error> get copyWith => _$LoadPhase_ErrorCopyWithImpl<LoadPhase_Error>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is LoadPhase_Error&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'LoadPhase.error(message: $message)';
}


}

/// @nodoc
abstract mixin class $LoadPhase_ErrorCopyWith<$Res> implements $LoadPhaseCopyWith<$Res> {
  factory $LoadPhase_ErrorCopyWith(LoadPhase_Error value, $Res Function(LoadPhase_Error) _then) = _$LoadPhase_ErrorCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$LoadPhase_ErrorCopyWithImpl<$Res>
    implements $LoadPhase_ErrorCopyWith<$Res> {
  _$LoadPhase_ErrorCopyWithImpl(this._self, this._then);

  final LoadPhase_Error _self;
  final $Res Function(LoadPhase_Error) _then;

/// Create a copy of LoadPhase
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(LoadPhase_Error(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
