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
    required TResult Function(String title) addTodo,
    required TResult Function(String id) toggleTodo,
    required TResult Function(String id) deleteTodo,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title)? addTodo,
    TResult? Function(String id)? toggleTodo,
    TResult? Function(String id)? deleteTodo,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title)? addTodo,
    TResult Function(String id)? toggleTodo,
    TResult Function(String id)? deleteTodo,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_AddTodo value) addTodo,
    required TResult Function(AppAction_ToggleTodo value) toggleTodo,
    required TResult Function(AppAction_DeleteTodo value) deleteTodo,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_AddTodo value)? addTodo,
    TResult? Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult? Function(AppAction_DeleteTodo value)? deleteTodo,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_AddTodo value)? addTodo,
    TResult Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult Function(AppAction_DeleteTodo value)? deleteTodo,
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
abstract class _$$AppAction_AddTodoImplCopyWith<$Res> {
  factory _$$AppAction_AddTodoImplCopyWith(
    _$AppAction_AddTodoImpl value,
    $Res Function(_$AppAction_AddTodoImpl) then,
  ) = __$$AppAction_AddTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String title});
}

/// @nodoc
class __$$AppAction_AddTodoImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_AddTodoImpl>
    implements _$$AppAction_AddTodoImplCopyWith<$Res> {
  __$$AppAction_AddTodoImplCopyWithImpl(
    _$AppAction_AddTodoImpl _value,
    $Res Function(_$AppAction_AddTodoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? title = null}) {
    return _then(
      _$AppAction_AddTodoImpl(
        title: null == title
            ? _value.title
            : title // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AppAction_AddTodoImpl extends AppAction_AddTodo {
  const _$AppAction_AddTodoImpl({required this.title}) : super._();

  @override
  final String title;

  @override
  String toString() {
    return 'AppAction.addTodo(title: $title)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_AddTodoImpl &&
            (identical(other.title, title) || other.title == title));
  }

  @override
  int get hashCode => Object.hash(runtimeType, title);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppAction_AddTodoImplCopyWith<_$AppAction_AddTodoImpl> get copyWith =>
      __$$AppAction_AddTodoImplCopyWithImpl<_$AppAction_AddTodoImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title) addTodo,
    required TResult Function(String id) toggleTodo,
    required TResult Function(String id) deleteTodo,
  }) {
    return addTodo(title);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title)? addTodo,
    TResult? Function(String id)? toggleTodo,
    TResult? Function(String id)? deleteTodo,
  }) {
    return addTodo?.call(title);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title)? addTodo,
    TResult Function(String id)? toggleTodo,
    TResult Function(String id)? deleteTodo,
    required TResult orElse(),
  }) {
    if (addTodo != null) {
      return addTodo(title);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_AddTodo value) addTodo,
    required TResult Function(AppAction_ToggleTodo value) toggleTodo,
    required TResult Function(AppAction_DeleteTodo value) deleteTodo,
  }) {
    return addTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_AddTodo value)? addTodo,
    TResult? Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult? Function(AppAction_DeleteTodo value)? deleteTodo,
  }) {
    return addTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_AddTodo value)? addTodo,
    TResult Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult Function(AppAction_DeleteTodo value)? deleteTodo,
    required TResult orElse(),
  }) {
    if (addTodo != null) {
      return addTodo(this);
    }
    return orElse();
  }
}

abstract class AppAction_AddTodo extends AppAction {
  const factory AppAction_AddTodo({required final String title}) =
      _$AppAction_AddTodoImpl;
  const AppAction_AddTodo._() : super._();

  String get title;

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppAction_AddTodoImplCopyWith<_$AppAction_AddTodoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AppAction_ToggleTodoImplCopyWith<$Res> {
  factory _$$AppAction_ToggleTodoImplCopyWith(
    _$AppAction_ToggleTodoImpl value,
    $Res Function(_$AppAction_ToggleTodoImpl) then,
  ) = __$$AppAction_ToggleTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String id});
}

/// @nodoc
class __$$AppAction_ToggleTodoImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_ToggleTodoImpl>
    implements _$$AppAction_ToggleTodoImplCopyWith<$Res> {
  __$$AppAction_ToggleTodoImplCopyWithImpl(
    _$AppAction_ToggleTodoImpl _value,
    $Res Function(_$AppAction_ToggleTodoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? id = null}) {
    return _then(
      _$AppAction_ToggleTodoImpl(
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AppAction_ToggleTodoImpl extends AppAction_ToggleTodo {
  const _$AppAction_ToggleTodoImpl({required this.id}) : super._();

  @override
  final String id;

  @override
  String toString() {
    return 'AppAction.toggleTodo(id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_ToggleTodoImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppAction_ToggleTodoImplCopyWith<_$AppAction_ToggleTodoImpl>
  get copyWith =>
      __$$AppAction_ToggleTodoImplCopyWithImpl<_$AppAction_ToggleTodoImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title) addTodo,
    required TResult Function(String id) toggleTodo,
    required TResult Function(String id) deleteTodo,
  }) {
    return toggleTodo(id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title)? addTodo,
    TResult? Function(String id)? toggleTodo,
    TResult? Function(String id)? deleteTodo,
  }) {
    return toggleTodo?.call(id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title)? addTodo,
    TResult Function(String id)? toggleTodo,
    TResult Function(String id)? deleteTodo,
    required TResult orElse(),
  }) {
    if (toggleTodo != null) {
      return toggleTodo(id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_AddTodo value) addTodo,
    required TResult Function(AppAction_ToggleTodo value) toggleTodo,
    required TResult Function(AppAction_DeleteTodo value) deleteTodo,
  }) {
    return toggleTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_AddTodo value)? addTodo,
    TResult? Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult? Function(AppAction_DeleteTodo value)? deleteTodo,
  }) {
    return toggleTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_AddTodo value)? addTodo,
    TResult Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult Function(AppAction_DeleteTodo value)? deleteTodo,
    required TResult orElse(),
  }) {
    if (toggleTodo != null) {
      return toggleTodo(this);
    }
    return orElse();
  }
}

abstract class AppAction_ToggleTodo extends AppAction {
  const factory AppAction_ToggleTodo({required final String id}) =
      _$AppAction_ToggleTodoImpl;
  const AppAction_ToggleTodo._() : super._();

  String get id;

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppAction_ToggleTodoImplCopyWith<_$AppAction_ToggleTodoImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AppAction_DeleteTodoImplCopyWith<$Res> {
  factory _$$AppAction_DeleteTodoImplCopyWith(
    _$AppAction_DeleteTodoImpl value,
    $Res Function(_$AppAction_DeleteTodoImpl) then,
  ) = __$$AppAction_DeleteTodoImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String id});
}

/// @nodoc
class __$$AppAction_DeleteTodoImplCopyWithImpl<$Res>
    extends _$AppActionCopyWithImpl<$Res, _$AppAction_DeleteTodoImpl>
    implements _$$AppAction_DeleteTodoImplCopyWith<$Res> {
  __$$AppAction_DeleteTodoImplCopyWithImpl(
    _$AppAction_DeleteTodoImpl _value,
    $Res Function(_$AppAction_DeleteTodoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? id = null}) {
    return _then(
      _$AppAction_DeleteTodoImpl(
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AppAction_DeleteTodoImpl extends AppAction_DeleteTodo {
  const _$AppAction_DeleteTodoImpl({required this.id}) : super._();

  @override
  final String id;

  @override
  String toString() {
    return 'AppAction.deleteTodo(id: $id)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AppAction_DeleteTodoImpl &&
            (identical(other.id, id) || other.id == id));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id);

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$AppAction_DeleteTodoImplCopyWith<_$AppAction_DeleteTodoImpl>
  get copyWith =>
      __$$AppAction_DeleteTodoImplCopyWithImpl<_$AppAction_DeleteTodoImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String title) addTodo,
    required TResult Function(String id) toggleTodo,
    required TResult Function(String id) deleteTodo,
  }) {
    return deleteTodo(id);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String title)? addTodo,
    TResult? Function(String id)? toggleTodo,
    TResult? Function(String id)? deleteTodo,
  }) {
    return deleteTodo?.call(id);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String title)? addTodo,
    TResult Function(String id)? toggleTodo,
    TResult Function(String id)? deleteTodo,
    required TResult orElse(),
  }) {
    if (deleteTodo != null) {
      return deleteTodo(id);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AppAction_AddTodo value) addTodo,
    required TResult Function(AppAction_ToggleTodo value) toggleTodo,
    required TResult Function(AppAction_DeleteTodo value) deleteTodo,
  }) {
    return deleteTodo(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AppAction_AddTodo value)? addTodo,
    TResult? Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult? Function(AppAction_DeleteTodo value)? deleteTodo,
  }) {
    return deleteTodo?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AppAction_AddTodo value)? addTodo,
    TResult Function(AppAction_ToggleTodo value)? toggleTodo,
    TResult Function(AppAction_DeleteTodo value)? deleteTodo,
    required TResult orElse(),
  }) {
    if (deleteTodo != null) {
      return deleteTodo(this);
    }
    return orElse();
  }
}

abstract class AppAction_DeleteTodo extends AppAction {
  const factory AppAction_DeleteTodo({required final String id}) =
      _$AppAction_DeleteTodoImpl;
  const AppAction_DeleteTodo._() : super._();

  String get id;

  /// Create a copy of AppAction
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$AppAction_DeleteTodoImplCopyWith<_$AppAction_DeleteTodoImpl>
  get copyWith => throw _privateConstructorUsedError;
}
