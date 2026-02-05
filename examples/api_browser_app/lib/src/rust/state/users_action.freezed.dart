// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'users_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$UsersAction {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UsersAction);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'UsersAction()';
}


}

/// @nodoc
class $UsersActionCopyWith<$Res>  {
$UsersActionCopyWith(UsersAction _, $Res Function(UsersAction) __);
}


/// Adds pattern-matching-related methods to [UsersAction].
extension UsersActionPatterns on UsersAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( UsersAction_Refresh value)?  refresh,TResult Function( UsersAction_SelectUser value)?  selectUser,required TResult orElse(),}){
final _that = this;
switch (_that) {
case UsersAction_Refresh() when refresh != null:
return refresh(_that);case UsersAction_SelectUser() when selectUser != null:
return selectUser(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( UsersAction_Refresh value)  refresh,required TResult Function( UsersAction_SelectUser value)  selectUser,}){
final _that = this;
switch (_that) {
case UsersAction_Refresh():
return refresh(_that);case UsersAction_SelectUser():
return selectUser(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( UsersAction_Refresh value)?  refresh,TResult? Function( UsersAction_SelectUser value)?  selectUser,}){
final _that = this;
switch (_that) {
case UsersAction_Refresh() when refresh != null:
return refresh(_that);case UsersAction_SelectUser() when selectUser != null:
return selectUser(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  refresh,TResult Function( BigInt userId)?  selectUser,required TResult orElse(),}) {final _that = this;
switch (_that) {
case UsersAction_Refresh() when refresh != null:
return refresh();case UsersAction_SelectUser() when selectUser != null:
return selectUser(_that.userId);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  refresh,required TResult Function( BigInt userId)  selectUser,}) {final _that = this;
switch (_that) {
case UsersAction_Refresh():
return refresh();case UsersAction_SelectUser():
return selectUser(_that.userId);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  refresh,TResult? Function( BigInt userId)?  selectUser,}) {final _that = this;
switch (_that) {
case UsersAction_Refresh() when refresh != null:
return refresh();case UsersAction_SelectUser() when selectUser != null:
return selectUser(_that.userId);case _:
  return null;

}
}

}

/// @nodoc


class UsersAction_Refresh extends UsersAction {
  const UsersAction_Refresh(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UsersAction_Refresh);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'UsersAction.refresh()';
}


}




/// @nodoc


class UsersAction_SelectUser extends UsersAction {
  const UsersAction_SelectUser({required this.userId}): super._();
  

 final  BigInt userId;

/// Create a copy of UsersAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$UsersAction_SelectUserCopyWith<UsersAction_SelectUser> get copyWith => _$UsersAction_SelectUserCopyWithImpl<UsersAction_SelectUser>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is UsersAction_SelectUser&&(identical(other.userId, userId) || other.userId == userId));
}


@override
int get hashCode => Object.hash(runtimeType,userId);

@override
String toString() {
  return 'UsersAction.selectUser(userId: $userId)';
}


}

/// @nodoc
abstract mixin class $UsersAction_SelectUserCopyWith<$Res> implements $UsersActionCopyWith<$Res> {
  factory $UsersAction_SelectUserCopyWith(UsersAction_SelectUser value, $Res Function(UsersAction_SelectUser) _then) = _$UsersAction_SelectUserCopyWithImpl;
@useResult
$Res call({
 BigInt userId
});




}
/// @nodoc
class _$UsersAction_SelectUserCopyWithImpl<$Res>
    implements $UsersAction_SelectUserCopyWith<$Res> {
  _$UsersAction_SelectUserCopyWithImpl(this._self, this._then);

  final UsersAction_SelectUser _self;
  final $Res Function(UsersAction_SelectUser) _then;

/// Create a copy of UsersAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? userId = null,}) {
  return _then(UsersAction_SelectUser(
userId: null == userId ? _self.userId : userId // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
