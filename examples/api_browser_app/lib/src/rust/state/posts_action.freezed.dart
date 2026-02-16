// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'posts_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$PostsAction {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PostsAction);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PostsAction()';
}


}

/// @nodoc
class $PostsActionCopyWith<$Res>  {
$PostsActionCopyWith(PostsAction _, $Res Function(PostsAction) __);
}


/// Adds pattern-matching-related methods to [PostsAction].
extension PostsActionPatterns on PostsAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( PostsAction_LoadForUser value)?  loadForUser,TResult Function( PostsAction_Refresh value)?  refresh,TResult Function( PostsAction_SelectPost value)?  selectPost,required TResult orElse(),}){
final _that = this;
switch (_that) {
case PostsAction_LoadForUser() when loadForUser != null:
return loadForUser(_that);case PostsAction_Refresh() when refresh != null:
return refresh(_that);case PostsAction_SelectPost() when selectPost != null:
return selectPost(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( PostsAction_LoadForUser value)  loadForUser,required TResult Function( PostsAction_Refresh value)  refresh,required TResult Function( PostsAction_SelectPost value)  selectPost,}){
final _that = this;
switch (_that) {
case PostsAction_LoadForUser():
return loadForUser(_that);case PostsAction_Refresh():
return refresh(_that);case PostsAction_SelectPost():
return selectPost(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( PostsAction_LoadForUser value)?  loadForUser,TResult? Function( PostsAction_Refresh value)?  refresh,TResult? Function( PostsAction_SelectPost value)?  selectPost,}){
final _that = this;
switch (_that) {
case PostsAction_LoadForUser() when loadForUser != null:
return loadForUser(_that);case PostsAction_Refresh() when refresh != null:
return refresh(_that);case PostsAction_SelectPost() when selectPost != null:
return selectPost(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt userId)?  loadForUser,TResult Function()?  refresh,TResult Function( BigInt postId)?  selectPost,required TResult orElse(),}) {final _that = this;
switch (_that) {
case PostsAction_LoadForUser() when loadForUser != null:
return loadForUser(_that.userId);case PostsAction_Refresh() when refresh != null:
return refresh();case PostsAction_SelectPost() when selectPost != null:
return selectPost(_that.postId);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt userId)  loadForUser,required TResult Function()  refresh,required TResult Function( BigInt postId)  selectPost,}) {final _that = this;
switch (_that) {
case PostsAction_LoadForUser():
return loadForUser(_that.userId);case PostsAction_Refresh():
return refresh();case PostsAction_SelectPost():
return selectPost(_that.postId);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt userId)?  loadForUser,TResult? Function()?  refresh,TResult? Function( BigInt postId)?  selectPost,}) {final _that = this;
switch (_that) {
case PostsAction_LoadForUser() when loadForUser != null:
return loadForUser(_that.userId);case PostsAction_Refresh() when refresh != null:
return refresh();case PostsAction_SelectPost() when selectPost != null:
return selectPost(_that.postId);case _:
  return null;

}
}

}

/// @nodoc


class PostsAction_LoadForUser extends PostsAction {
  const PostsAction_LoadForUser({required this.userId}): super._();
  

 final  BigInt userId;

/// Create a copy of PostsAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PostsAction_LoadForUserCopyWith<PostsAction_LoadForUser> get copyWith => _$PostsAction_LoadForUserCopyWithImpl<PostsAction_LoadForUser>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PostsAction_LoadForUser&&(identical(other.userId, userId) || other.userId == userId));
}


@override
int get hashCode => Object.hash(runtimeType,userId);

@override
String toString() {
  return 'PostsAction.loadForUser(userId: $userId)';
}


}

/// @nodoc
abstract mixin class $PostsAction_LoadForUserCopyWith<$Res> implements $PostsActionCopyWith<$Res> {
  factory $PostsAction_LoadForUserCopyWith(PostsAction_LoadForUser value, $Res Function(PostsAction_LoadForUser) _then) = _$PostsAction_LoadForUserCopyWithImpl;
@useResult
$Res call({
 BigInt userId
});




}
/// @nodoc
class _$PostsAction_LoadForUserCopyWithImpl<$Res>
    implements $PostsAction_LoadForUserCopyWith<$Res> {
  _$PostsAction_LoadForUserCopyWithImpl(this._self, this._then);

  final PostsAction_LoadForUser _self;
  final $Res Function(PostsAction_LoadForUser) _then;

/// Create a copy of PostsAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? userId = null,}) {
  return _then(PostsAction_LoadForUser(
userId: null == userId ? _self.userId : userId // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class PostsAction_Refresh extends PostsAction {
  const PostsAction_Refresh(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PostsAction_Refresh);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'PostsAction.refresh()';
}


}




/// @nodoc


class PostsAction_SelectPost extends PostsAction {
  const PostsAction_SelectPost({required this.postId}): super._();
  

 final  BigInt postId;

/// Create a copy of PostsAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$PostsAction_SelectPostCopyWith<PostsAction_SelectPost> get copyWith => _$PostsAction_SelectPostCopyWithImpl<PostsAction_SelectPost>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is PostsAction_SelectPost&&(identical(other.postId, postId) || other.postId == postId));
}


@override
int get hashCode => Object.hash(runtimeType,postId);

@override
String toString() {
  return 'PostsAction.selectPost(postId: $postId)';
}


}

/// @nodoc
abstract mixin class $PostsAction_SelectPostCopyWith<$Res> implements $PostsActionCopyWith<$Res> {
  factory $PostsAction_SelectPostCopyWith(PostsAction_SelectPost value, $Res Function(PostsAction_SelectPost) _then) = _$PostsAction_SelectPostCopyWithImpl;
@useResult
$Res call({
 BigInt postId
});




}
/// @nodoc
class _$PostsAction_SelectPostCopyWithImpl<$Res>
    implements $PostsAction_SelectPostCopyWith<$Res> {
  _$PostsAction_SelectPostCopyWithImpl(this._self, this._then);

  final PostsAction_SelectPost _self;
  final $Res Function(PostsAction_SelectPost) _then;

/// Create a copy of PostsAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? postId = null,}) {
  return _then(PostsAction_SelectPost(
postId: null == postId ? _self.postId : postId // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
