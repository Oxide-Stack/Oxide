// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'comments_action.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$CommentsAction {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CommentsAction);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'CommentsAction()';
}


}

/// @nodoc
class $CommentsActionCopyWith<$Res>  {
$CommentsActionCopyWith(CommentsAction _, $Res Function(CommentsAction) __);
}


/// Adds pattern-matching-related methods to [CommentsAction].
extension CommentsActionPatterns on CommentsAction {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CommentsAction_LoadForPost value)?  loadForPost,TResult Function( CommentsAction_Refresh value)?  refresh,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CommentsAction_LoadForPost() when loadForPost != null:
return loadForPost(_that);case CommentsAction_Refresh() when refresh != null:
return refresh(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CommentsAction_LoadForPost value)  loadForPost,required TResult Function( CommentsAction_Refresh value)  refresh,}){
final _that = this;
switch (_that) {
case CommentsAction_LoadForPost():
return loadForPost(_that);case CommentsAction_Refresh():
return refresh(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CommentsAction_LoadForPost value)?  loadForPost,TResult? Function( CommentsAction_Refresh value)?  refresh,}){
final _that = this;
switch (_that) {
case CommentsAction_LoadForPost() when loadForPost != null:
return loadForPost(_that);case CommentsAction_Refresh() when refresh != null:
return refresh(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt postId)?  loadForPost,TResult Function()?  refresh,required TResult orElse(),}) {final _that = this;
switch (_that) {
case CommentsAction_LoadForPost() when loadForPost != null:
return loadForPost(_that.postId);case CommentsAction_Refresh() when refresh != null:
return refresh();case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt postId)  loadForPost,required TResult Function()  refresh,}) {final _that = this;
switch (_that) {
case CommentsAction_LoadForPost():
return loadForPost(_that.postId);case CommentsAction_Refresh():
return refresh();}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt postId)?  loadForPost,TResult? Function()?  refresh,}) {final _that = this;
switch (_that) {
case CommentsAction_LoadForPost() when loadForPost != null:
return loadForPost(_that.postId);case CommentsAction_Refresh() when refresh != null:
return refresh();case _:
  return null;

}
}

}

/// @nodoc


class CommentsAction_LoadForPost extends CommentsAction {
  const CommentsAction_LoadForPost({required this.postId}): super._();
  

 final  BigInt postId;

/// Create a copy of CommentsAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CommentsAction_LoadForPostCopyWith<CommentsAction_LoadForPost> get copyWith => _$CommentsAction_LoadForPostCopyWithImpl<CommentsAction_LoadForPost>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CommentsAction_LoadForPost&&(identical(other.postId, postId) || other.postId == postId));
}


@override
int get hashCode => Object.hash(runtimeType,postId);

@override
String toString() {
  return 'CommentsAction.loadForPost(postId: $postId)';
}


}

/// @nodoc
abstract mixin class $CommentsAction_LoadForPostCopyWith<$Res> implements $CommentsActionCopyWith<$Res> {
  factory $CommentsAction_LoadForPostCopyWith(CommentsAction_LoadForPost value, $Res Function(CommentsAction_LoadForPost) _then) = _$CommentsAction_LoadForPostCopyWithImpl;
@useResult
$Res call({
 BigInt postId
});




}
/// @nodoc
class _$CommentsAction_LoadForPostCopyWithImpl<$Res>
    implements $CommentsAction_LoadForPostCopyWith<$Res> {
  _$CommentsAction_LoadForPostCopyWithImpl(this._self, this._then);

  final CommentsAction_LoadForPost _self;
  final $Res Function(CommentsAction_LoadForPost) _then;

/// Create a copy of CommentsAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? postId = null,}) {
  return _then(CommentsAction_LoadForPost(
postId: null == postId ? _self.postId : postId // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class CommentsAction_Refresh extends CommentsAction {
  const CommentsAction_Refresh(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CommentsAction_Refresh);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'CommentsAction.refresh()';
}


}




// dart format on
