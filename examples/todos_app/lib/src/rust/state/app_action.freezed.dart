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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( AppAction_AddTodo value)?  addTodo,TResult Function( AppAction_ToggleTodo value)?  toggleTodo,TResult Function( AppAction_DeleteTodo value)?  deleteTodo,required TResult orElse(),}){
final _that = this;
switch (_that) {
case AppAction_AddTodo() when addTodo != null:
return addTodo(_that);case AppAction_ToggleTodo() when toggleTodo != null:
return toggleTodo(_that);case AppAction_DeleteTodo() when deleteTodo != null:
return deleteTodo(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( AppAction_AddTodo value)  addTodo,required TResult Function( AppAction_ToggleTodo value)  toggleTodo,required TResult Function( AppAction_DeleteTodo value)  deleteTodo,}){
final _that = this;
switch (_that) {
case AppAction_AddTodo():
return addTodo(_that);case AppAction_ToggleTodo():
return toggleTodo(_that);case AppAction_DeleteTodo():
return deleteTodo(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( AppAction_AddTodo value)?  addTodo,TResult? Function( AppAction_ToggleTodo value)?  toggleTodo,TResult? Function( AppAction_DeleteTodo value)?  deleteTodo,}){
final _that = this;
switch (_that) {
case AppAction_AddTodo() when addTodo != null:
return addTodo(_that);case AppAction_ToggleTodo() when toggleTodo != null:
return toggleTodo(_that);case AppAction_DeleteTodo() when deleteTodo != null:
return deleteTodo(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String title)?  addTodo,TResult Function( String id)?  toggleTodo,TResult Function( String id)?  deleteTodo,required TResult orElse(),}) {final _that = this;
switch (_that) {
case AppAction_AddTodo() when addTodo != null:
return addTodo(_that.title);case AppAction_ToggleTodo() when toggleTodo != null:
return toggleTodo(_that.id);case AppAction_DeleteTodo() when deleteTodo != null:
return deleteTodo(_that.id);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String title)  addTodo,required TResult Function( String id)  toggleTodo,required TResult Function( String id)  deleteTodo,}) {final _that = this;
switch (_that) {
case AppAction_AddTodo():
return addTodo(_that.title);case AppAction_ToggleTodo():
return toggleTodo(_that.id);case AppAction_DeleteTodo():
return deleteTodo(_that.id);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String title)?  addTodo,TResult? Function( String id)?  toggleTodo,TResult? Function( String id)?  deleteTodo,}) {final _that = this;
switch (_that) {
case AppAction_AddTodo() when addTodo != null:
return addTodo(_that.title);case AppAction_ToggleTodo() when toggleTodo != null:
return toggleTodo(_that.id);case AppAction_DeleteTodo() when deleteTodo != null:
return deleteTodo(_that.id);case _:
  return null;

}
}

}

/// @nodoc


class AppAction_AddTodo extends AppAction {
  const AppAction_AddTodo({required this.title}): super._();
  

 final  String title;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AppAction_AddTodoCopyWith<AppAction_AddTodo> get copyWith => _$AppAction_AddTodoCopyWithImpl<AppAction_AddTodo>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_AddTodo&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,title);

@override
String toString() {
  return 'AppAction.addTodo(title: $title)';
}


}

/// @nodoc
abstract mixin class $AppAction_AddTodoCopyWith<$Res> implements $AppActionCopyWith<$Res> {
  factory $AppAction_AddTodoCopyWith(AppAction_AddTodo value, $Res Function(AppAction_AddTodo) _then) = _$AppAction_AddTodoCopyWithImpl;
@useResult
$Res call({
 String title
});




}
/// @nodoc
class _$AppAction_AddTodoCopyWithImpl<$Res>
    implements $AppAction_AddTodoCopyWith<$Res> {
  _$AppAction_AddTodoCopyWithImpl(this._self, this._then);

  final AppAction_AddTodo _self;
  final $Res Function(AppAction_AddTodo) _then;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? title = null,}) {
  return _then(AppAction_AddTodo(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AppAction_ToggleTodo extends AppAction {
  const AppAction_ToggleTodo({required this.id}): super._();
  

 final  String id;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AppAction_ToggleTodoCopyWith<AppAction_ToggleTodo> get copyWith => _$AppAction_ToggleTodoCopyWithImpl<AppAction_ToggleTodo>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_ToggleTodo&&(identical(other.id, id) || other.id == id));
}


@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'AppAction.toggleTodo(id: $id)';
}


}

/// @nodoc
abstract mixin class $AppAction_ToggleTodoCopyWith<$Res> implements $AppActionCopyWith<$Res> {
  factory $AppAction_ToggleTodoCopyWith(AppAction_ToggleTodo value, $Res Function(AppAction_ToggleTodo) _then) = _$AppAction_ToggleTodoCopyWithImpl;
@useResult
$Res call({
 String id
});




}
/// @nodoc
class _$AppAction_ToggleTodoCopyWithImpl<$Res>
    implements $AppAction_ToggleTodoCopyWith<$Res> {
  _$AppAction_ToggleTodoCopyWithImpl(this._self, this._then);

  final AppAction_ToggleTodo _self;
  final $Res Function(AppAction_ToggleTodo) _then;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? id = null,}) {
  return _then(AppAction_ToggleTodo(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class AppAction_DeleteTodo extends AppAction {
  const AppAction_DeleteTodo({required this.id}): super._();
  

 final  String id;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$AppAction_DeleteTodoCopyWith<AppAction_DeleteTodo> get copyWith => _$AppAction_DeleteTodoCopyWithImpl<AppAction_DeleteTodo>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is AppAction_DeleteTodo&&(identical(other.id, id) || other.id == id));
}


@override
int get hashCode => Object.hash(runtimeType,id);

@override
String toString() {
  return 'AppAction.deleteTodo(id: $id)';
}


}

/// @nodoc
abstract mixin class $AppAction_DeleteTodoCopyWith<$Res> implements $AppActionCopyWith<$Res> {
  factory $AppAction_DeleteTodoCopyWith(AppAction_DeleteTodo value, $Res Function(AppAction_DeleteTodo) _then) = _$AppAction_DeleteTodoCopyWithImpl;
@useResult
$Res call({
 String id
});




}
/// @nodoc
class _$AppAction_DeleteTodoCopyWithImpl<$Res>
    implements $AppAction_DeleteTodoCopyWith<$Res> {
  _$AppAction_DeleteTodoCopyWithImpl(this._self, this._then);

  final AppAction_DeleteTodo _self;
  final $Res Function(AppAction_DeleteTodo) _then;

/// Create a copy of AppAction
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? id = null,}) {
  return _then(AppAction_DeleteTodo(
id: null == id ? _self.id : id // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
