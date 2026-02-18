// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'channels.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ApiBrowserDemoDialogRequest {

 String get title;
/// Create a copy of ApiBrowserDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoDialogRequestCopyWith<ApiBrowserDemoDialogRequest> get copyWith => _$ApiBrowserDemoDialogRequestCopyWithImpl<ApiBrowserDemoDialogRequest>(this as ApiBrowserDemoDialogRequest, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoDialogRequest&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,title);

@override
String toString() {
  return 'ApiBrowserDemoDialogRequest(title: $title)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoDialogRequestCopyWith<$Res>  {
  factory $ApiBrowserDemoDialogRequestCopyWith(ApiBrowserDemoDialogRequest value, $Res Function(ApiBrowserDemoDialogRequest) _then) = _$ApiBrowserDemoDialogRequestCopyWithImpl;
@useResult
$Res call({
 String title
});




}
/// @nodoc
class _$ApiBrowserDemoDialogRequestCopyWithImpl<$Res>
    implements $ApiBrowserDemoDialogRequestCopyWith<$Res> {
  _$ApiBrowserDemoDialogRequestCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoDialogRequest _self;
  final $Res Function(ApiBrowserDemoDialogRequest) _then;

/// Create a copy of ApiBrowserDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? title = null,}) {
  return _then(_self.copyWith(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiBrowserDemoDialogRequest].
extension ApiBrowserDemoDialogRequestPatterns on ApiBrowserDemoDialogRequest {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiBrowserDemoDialogRequest_Confirm value)?  confirm,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm() when confirm != null:
return confirm(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiBrowserDemoDialogRequest_Confirm value)  confirm,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm():
return confirm(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiBrowserDemoDialogRequest_Confirm value)?  confirm,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm() when confirm != null:
return confirm(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String title)?  confirm,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm() when confirm != null:
return confirm(_that.title);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String title)  confirm,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm():
return confirm(_that.title);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String title)?  confirm,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogRequest_Confirm() when confirm != null:
return confirm(_that.title);case _:
  return null;

}
}

}

/// @nodoc


class ApiBrowserDemoDialogRequest_Confirm extends ApiBrowserDemoDialogRequest {
  const ApiBrowserDemoDialogRequest_Confirm({required this.title}): super._();
  

@override final  String title;

/// Create a copy of ApiBrowserDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoDialogRequest_ConfirmCopyWith<ApiBrowserDemoDialogRequest_Confirm> get copyWith => _$ApiBrowserDemoDialogRequest_ConfirmCopyWithImpl<ApiBrowserDemoDialogRequest_Confirm>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoDialogRequest_Confirm&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,title);

@override
String toString() {
  return 'ApiBrowserDemoDialogRequest.confirm(title: $title)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoDialogRequest_ConfirmCopyWith<$Res> implements $ApiBrowserDemoDialogRequestCopyWith<$Res> {
  factory $ApiBrowserDemoDialogRequest_ConfirmCopyWith(ApiBrowserDemoDialogRequest_Confirm value, $Res Function(ApiBrowserDemoDialogRequest_Confirm) _then) = _$ApiBrowserDemoDialogRequest_ConfirmCopyWithImpl;
@override @useResult
$Res call({
 String title
});




}
/// @nodoc
class _$ApiBrowserDemoDialogRequest_ConfirmCopyWithImpl<$Res>
    implements $ApiBrowserDemoDialogRequest_ConfirmCopyWith<$Res> {
  _$ApiBrowserDemoDialogRequest_ConfirmCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoDialogRequest_Confirm _self;
  final $Res Function(ApiBrowserDemoDialogRequest_Confirm) _then;

/// Create a copy of ApiBrowserDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? title = null,}) {
  return _then(ApiBrowserDemoDialogRequest_Confirm(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$ApiBrowserDemoDialogResponse {

 bool get field0;
/// Create a copy of ApiBrowserDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoDialogResponseCopyWith<ApiBrowserDemoDialogResponse> get copyWith => _$ApiBrowserDemoDialogResponseCopyWithImpl<ApiBrowserDemoDialogResponse>(this as ApiBrowserDemoDialogResponse, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoDialogResponse&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ApiBrowserDemoDialogResponse(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoDialogResponseCopyWith<$Res>  {
  factory $ApiBrowserDemoDialogResponseCopyWith(ApiBrowserDemoDialogResponse value, $Res Function(ApiBrowserDemoDialogResponse) _then) = _$ApiBrowserDemoDialogResponseCopyWithImpl;
@useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$ApiBrowserDemoDialogResponseCopyWithImpl<$Res>
    implements $ApiBrowserDemoDialogResponseCopyWith<$Res> {
  _$ApiBrowserDemoDialogResponseCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoDialogResponse _self;
  final $Res Function(ApiBrowserDemoDialogResponse) _then;

/// Create a copy of ApiBrowserDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiBrowserDemoDialogResponse].
extension ApiBrowserDemoDialogResponsePatterns on ApiBrowserDemoDialogResponse {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiBrowserDemoDialogResponse_Confirm value)?  confirm,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm() when confirm != null:
return confirm(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiBrowserDemoDialogResponse_Confirm value)  confirm,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm():
return confirm(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiBrowserDemoDialogResponse_Confirm value)?  confirm,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm() when confirm != null:
return confirm(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( bool field0)?  confirm,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm() when confirm != null:
return confirm(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( bool field0)  confirm,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm():
return confirm(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( bool field0)?  confirm,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoDialogResponse_Confirm() when confirm != null:
return confirm(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ApiBrowserDemoDialogResponse_Confirm extends ApiBrowserDemoDialogResponse {
  const ApiBrowserDemoDialogResponse_Confirm(this.field0): super._();
  

@override final  bool field0;

/// Create a copy of ApiBrowserDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoDialogResponse_ConfirmCopyWith<ApiBrowserDemoDialogResponse_Confirm> get copyWith => _$ApiBrowserDemoDialogResponse_ConfirmCopyWithImpl<ApiBrowserDemoDialogResponse_Confirm>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoDialogResponse_Confirm&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ApiBrowserDemoDialogResponse.confirm(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoDialogResponse_ConfirmCopyWith<$Res> implements $ApiBrowserDemoDialogResponseCopyWith<$Res> {
  factory $ApiBrowserDemoDialogResponse_ConfirmCopyWith(ApiBrowserDemoDialogResponse_Confirm value, $Res Function(ApiBrowserDemoDialogResponse_Confirm) _then) = _$ApiBrowserDemoDialogResponse_ConfirmCopyWithImpl;
@override @useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$ApiBrowserDemoDialogResponse_ConfirmCopyWithImpl<$Res>
    implements $ApiBrowserDemoDialogResponse_ConfirmCopyWith<$Res> {
  _$ApiBrowserDemoDialogResponse_ConfirmCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoDialogResponse_Confirm _self;
  final $Res Function(ApiBrowserDemoDialogResponse_Confirm) _then;

/// Create a copy of ApiBrowserDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ApiBrowserDemoDialogResponse_Confirm(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$ApiBrowserDemoEvent {

 String get message;
/// Create a copy of ApiBrowserDemoEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoEventCopyWith<ApiBrowserDemoEvent> get copyWith => _$ApiBrowserDemoEventCopyWithImpl<ApiBrowserDemoEvent>(this as ApiBrowserDemoEvent, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoEvent&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'ApiBrowserDemoEvent(message: $message)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoEventCopyWith<$Res>  {
  factory $ApiBrowserDemoEventCopyWith(ApiBrowserDemoEvent value, $Res Function(ApiBrowserDemoEvent) _then) = _$ApiBrowserDemoEventCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$ApiBrowserDemoEventCopyWithImpl<$Res>
    implements $ApiBrowserDemoEventCopyWith<$Res> {
  _$ApiBrowserDemoEventCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoEvent _self;
  final $Res Function(ApiBrowserDemoEvent) _then;

/// Create a copy of ApiBrowserDemoEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? message = null,}) {
  return _then(_self.copyWith(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiBrowserDemoEvent].
extension ApiBrowserDemoEventPatterns on ApiBrowserDemoEvent {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiBrowserDemoEvent_Notify value)?  notify,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify() when notify != null:
return notify(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiBrowserDemoEvent_Notify value)  notify,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify():
return notify(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiBrowserDemoEvent_Notify value)?  notify,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify() when notify != null:
return notify(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String message)?  notify,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify() when notify != null:
return notify(_that.message);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String message)  notify,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify():
return notify(_that.message);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String message)?  notify,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoEvent_Notify() when notify != null:
return notify(_that.message);case _:
  return null;

}
}

}

/// @nodoc


class ApiBrowserDemoEvent_Notify extends ApiBrowserDemoEvent {
  const ApiBrowserDemoEvent_Notify({required this.message}): super._();
  

@override final  String message;

/// Create a copy of ApiBrowserDemoEvent
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoEvent_NotifyCopyWith<ApiBrowserDemoEvent_Notify> get copyWith => _$ApiBrowserDemoEvent_NotifyCopyWithImpl<ApiBrowserDemoEvent_Notify>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoEvent_Notify&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'ApiBrowserDemoEvent.notify(message: $message)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoEvent_NotifyCopyWith<$Res> implements $ApiBrowserDemoEventCopyWith<$Res> {
  factory $ApiBrowserDemoEvent_NotifyCopyWith(ApiBrowserDemoEvent_Notify value, $Res Function(ApiBrowserDemoEvent_Notify) _then) = _$ApiBrowserDemoEvent_NotifyCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$ApiBrowserDemoEvent_NotifyCopyWithImpl<$Res>
    implements $ApiBrowserDemoEvent_NotifyCopyWith<$Res> {
  _$ApiBrowserDemoEvent_NotifyCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoEvent_Notify _self;
  final $Res Function(ApiBrowserDemoEvent_Notify) _then;

/// Create a copy of ApiBrowserDemoEvent
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(ApiBrowserDemoEvent_Notify(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$ApiBrowserDemoIn {

 String get text;
/// Create a copy of ApiBrowserDemoIn
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoInCopyWith<ApiBrowserDemoIn> get copyWith => _$ApiBrowserDemoInCopyWithImpl<ApiBrowserDemoIn>(this as ApiBrowserDemoIn, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoIn&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'ApiBrowserDemoIn(text: $text)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoInCopyWith<$Res>  {
  factory $ApiBrowserDemoInCopyWith(ApiBrowserDemoIn value, $Res Function(ApiBrowserDemoIn) _then) = _$ApiBrowserDemoInCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$ApiBrowserDemoInCopyWithImpl<$Res>
    implements $ApiBrowserDemoInCopyWith<$Res> {
  _$ApiBrowserDemoInCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoIn _self;
  final $Res Function(ApiBrowserDemoIn) _then;

/// Create a copy of ApiBrowserDemoIn
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? text = null,}) {
  return _then(_self.copyWith(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiBrowserDemoIn].
extension ApiBrowserDemoInPatterns on ApiBrowserDemoIn {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiBrowserDemoIn_Receive value)?  receive,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive() when receive != null:
return receive(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiBrowserDemoIn_Receive value)  receive,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive():
return receive(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiBrowserDemoIn_Receive value)?  receive,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive() when receive != null:
return receive(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String text)?  receive,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive() when receive != null:
return receive(_that.text);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String text)  receive,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive():
return receive(_that.text);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String text)?  receive,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoIn_Receive() when receive != null:
return receive(_that.text);case _:
  return null;

}
}

}

/// @nodoc


class ApiBrowserDemoIn_Receive extends ApiBrowserDemoIn {
  const ApiBrowserDemoIn_Receive({required this.text}): super._();
  

@override final  String text;

/// Create a copy of ApiBrowserDemoIn
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoIn_ReceiveCopyWith<ApiBrowserDemoIn_Receive> get copyWith => _$ApiBrowserDemoIn_ReceiveCopyWithImpl<ApiBrowserDemoIn_Receive>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoIn_Receive&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'ApiBrowserDemoIn.receive(text: $text)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoIn_ReceiveCopyWith<$Res> implements $ApiBrowserDemoInCopyWith<$Res> {
  factory $ApiBrowserDemoIn_ReceiveCopyWith(ApiBrowserDemoIn_Receive value, $Res Function(ApiBrowserDemoIn_Receive) _then) = _$ApiBrowserDemoIn_ReceiveCopyWithImpl;
@override @useResult
$Res call({
 String text
});




}
/// @nodoc
class _$ApiBrowserDemoIn_ReceiveCopyWithImpl<$Res>
    implements $ApiBrowserDemoIn_ReceiveCopyWith<$Res> {
  _$ApiBrowserDemoIn_ReceiveCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoIn_Receive _self;
  final $Res Function(ApiBrowserDemoIn_Receive) _then;

/// Create a copy of ApiBrowserDemoIn
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(ApiBrowserDemoIn_Receive(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$ApiBrowserDemoOut {

 String get text;
/// Create a copy of ApiBrowserDemoOut
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoOutCopyWith<ApiBrowserDemoOut> get copyWith => _$ApiBrowserDemoOutCopyWithImpl<ApiBrowserDemoOut>(this as ApiBrowserDemoOut, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoOut&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'ApiBrowserDemoOut(text: $text)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoOutCopyWith<$Res>  {
  factory $ApiBrowserDemoOutCopyWith(ApiBrowserDemoOut value, $Res Function(ApiBrowserDemoOut) _then) = _$ApiBrowserDemoOutCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$ApiBrowserDemoOutCopyWithImpl<$Res>
    implements $ApiBrowserDemoOutCopyWith<$Res> {
  _$ApiBrowserDemoOutCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoOut _self;
  final $Res Function(ApiBrowserDemoOut) _then;

/// Create a copy of ApiBrowserDemoOut
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? text = null,}) {
  return _then(_self.copyWith(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [ApiBrowserDemoOut].
extension ApiBrowserDemoOutPatterns on ApiBrowserDemoOut {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ApiBrowserDemoOut_Send value)?  send,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send() when send != null:
return send(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ApiBrowserDemoOut_Send value)  send,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send():
return send(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ApiBrowserDemoOut_Send value)?  send,}){
final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send() when send != null:
return send(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String text)?  send,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send() when send != null:
return send(_that.text);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String text)  send,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send():
return send(_that.text);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String text)?  send,}) {final _that = this;
switch (_that) {
case ApiBrowserDemoOut_Send() when send != null:
return send(_that.text);case _:
  return null;

}
}

}

/// @nodoc


class ApiBrowserDemoOut_Send extends ApiBrowserDemoOut {
  const ApiBrowserDemoOut_Send({required this.text}): super._();
  

@override final  String text;

/// Create a copy of ApiBrowserDemoOut
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ApiBrowserDemoOut_SendCopyWith<ApiBrowserDemoOut_Send> get copyWith => _$ApiBrowserDemoOut_SendCopyWithImpl<ApiBrowserDemoOut_Send>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ApiBrowserDemoOut_Send&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'ApiBrowserDemoOut.send(text: $text)';
}


}

/// @nodoc
abstract mixin class $ApiBrowserDemoOut_SendCopyWith<$Res> implements $ApiBrowserDemoOutCopyWith<$Res> {
  factory $ApiBrowserDemoOut_SendCopyWith(ApiBrowserDemoOut_Send value, $Res Function(ApiBrowserDemoOut_Send) _then) = _$ApiBrowserDemoOut_SendCopyWithImpl;
@override @useResult
$Res call({
 String text
});




}
/// @nodoc
class _$ApiBrowserDemoOut_SendCopyWithImpl<$Res>
    implements $ApiBrowserDemoOut_SendCopyWith<$Res> {
  _$ApiBrowserDemoOut_SendCopyWithImpl(this._self, this._then);

  final ApiBrowserDemoOut_Send _self;
  final $Res Function(ApiBrowserDemoOut_Send) _then;

/// Create a copy of ApiBrowserDemoOut
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(ApiBrowserDemoOut_Send(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
