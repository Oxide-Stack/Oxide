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
mixin _$CounterDemoDialogRequest {

 String get title;
/// Create a copy of CounterDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoDialogRequestCopyWith<CounterDemoDialogRequest> get copyWith => _$CounterDemoDialogRequestCopyWithImpl<CounterDemoDialogRequest>(this as CounterDemoDialogRequest, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoDialogRequest&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,title);

@override
String toString() {
  return 'CounterDemoDialogRequest(title: $title)';
}


}

/// @nodoc
abstract mixin class $CounterDemoDialogRequestCopyWith<$Res>  {
  factory $CounterDemoDialogRequestCopyWith(CounterDemoDialogRequest value, $Res Function(CounterDemoDialogRequest) _then) = _$CounterDemoDialogRequestCopyWithImpl;
@useResult
$Res call({
 String title
});




}
/// @nodoc
class _$CounterDemoDialogRequestCopyWithImpl<$Res>
    implements $CounterDemoDialogRequestCopyWith<$Res> {
  _$CounterDemoDialogRequestCopyWithImpl(this._self, this._then);

  final CounterDemoDialogRequest _self;
  final $Res Function(CounterDemoDialogRequest) _then;

/// Create a copy of CounterDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? title = null,}) {
  return _then(_self.copyWith(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterDemoDialogRequest].
extension CounterDemoDialogRequestPatterns on CounterDemoDialogRequest {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterDemoDialogRequest_Confirm value)?  confirm,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterDemoDialogRequest_Confirm() when confirm != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterDemoDialogRequest_Confirm value)  confirm,}){
final _that = this;
switch (_that) {
case CounterDemoDialogRequest_Confirm():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterDemoDialogRequest_Confirm value)?  confirm,}){
final _that = this;
switch (_that) {
case CounterDemoDialogRequest_Confirm() when confirm != null:
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
case CounterDemoDialogRequest_Confirm() when confirm != null:
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
case CounterDemoDialogRequest_Confirm():
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
case CounterDemoDialogRequest_Confirm() when confirm != null:
return confirm(_that.title);case _:
  return null;

}
}

}

/// @nodoc


class CounterDemoDialogRequest_Confirm extends CounterDemoDialogRequest {
  const CounterDemoDialogRequest_Confirm({required this.title}): super._();
  

@override final  String title;

/// Create a copy of CounterDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoDialogRequest_ConfirmCopyWith<CounterDemoDialogRequest_Confirm> get copyWith => _$CounterDemoDialogRequest_ConfirmCopyWithImpl<CounterDemoDialogRequest_Confirm>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoDialogRequest_Confirm&&(identical(other.title, title) || other.title == title));
}


@override
int get hashCode => Object.hash(runtimeType,title);

@override
String toString() {
  return 'CounterDemoDialogRequest.confirm(title: $title)';
}


}

/// @nodoc
abstract mixin class $CounterDemoDialogRequest_ConfirmCopyWith<$Res> implements $CounterDemoDialogRequestCopyWith<$Res> {
  factory $CounterDemoDialogRequest_ConfirmCopyWith(CounterDemoDialogRequest_Confirm value, $Res Function(CounterDemoDialogRequest_Confirm) _then) = _$CounterDemoDialogRequest_ConfirmCopyWithImpl;
@override @useResult
$Res call({
 String title
});




}
/// @nodoc
class _$CounterDemoDialogRequest_ConfirmCopyWithImpl<$Res>
    implements $CounterDemoDialogRequest_ConfirmCopyWith<$Res> {
  _$CounterDemoDialogRequest_ConfirmCopyWithImpl(this._self, this._then);

  final CounterDemoDialogRequest_Confirm _self;
  final $Res Function(CounterDemoDialogRequest_Confirm) _then;

/// Create a copy of CounterDemoDialogRequest
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? title = null,}) {
  return _then(CounterDemoDialogRequest_Confirm(
title: null == title ? _self.title : title // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$CounterDemoDialogResponse {

 bool get field0;
/// Create a copy of CounterDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoDialogResponseCopyWith<CounterDemoDialogResponse> get copyWith => _$CounterDemoDialogResponseCopyWithImpl<CounterDemoDialogResponse>(this as CounterDemoDialogResponse, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoDialogResponse&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'CounterDemoDialogResponse(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $CounterDemoDialogResponseCopyWith<$Res>  {
  factory $CounterDemoDialogResponseCopyWith(CounterDemoDialogResponse value, $Res Function(CounterDemoDialogResponse) _then) = _$CounterDemoDialogResponseCopyWithImpl;
@useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$CounterDemoDialogResponseCopyWithImpl<$Res>
    implements $CounterDemoDialogResponseCopyWith<$Res> {
  _$CounterDemoDialogResponseCopyWithImpl(this._self, this._then);

  final CounterDemoDialogResponse _self;
  final $Res Function(CounterDemoDialogResponse) _then;

/// Create a copy of CounterDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterDemoDialogResponse].
extension CounterDemoDialogResponsePatterns on CounterDemoDialogResponse {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterDemoDialogResponse_Confirm value)?  confirm,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterDemoDialogResponse_Confirm() when confirm != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterDemoDialogResponse_Confirm value)  confirm,}){
final _that = this;
switch (_that) {
case CounterDemoDialogResponse_Confirm():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterDemoDialogResponse_Confirm value)?  confirm,}){
final _that = this;
switch (_that) {
case CounterDemoDialogResponse_Confirm() when confirm != null:
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
case CounterDemoDialogResponse_Confirm() when confirm != null:
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
case CounterDemoDialogResponse_Confirm():
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
case CounterDemoDialogResponse_Confirm() when confirm != null:
return confirm(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class CounterDemoDialogResponse_Confirm extends CounterDemoDialogResponse {
  const CounterDemoDialogResponse_Confirm(this.field0): super._();
  

@override final  bool field0;

/// Create a copy of CounterDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoDialogResponse_ConfirmCopyWith<CounterDemoDialogResponse_Confirm> get copyWith => _$CounterDemoDialogResponse_ConfirmCopyWithImpl<CounterDemoDialogResponse_Confirm>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoDialogResponse_Confirm&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'CounterDemoDialogResponse.confirm(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $CounterDemoDialogResponse_ConfirmCopyWith<$Res> implements $CounterDemoDialogResponseCopyWith<$Res> {
  factory $CounterDemoDialogResponse_ConfirmCopyWith(CounterDemoDialogResponse_Confirm value, $Res Function(CounterDemoDialogResponse_Confirm) _then) = _$CounterDemoDialogResponse_ConfirmCopyWithImpl;
@override @useResult
$Res call({
 bool field0
});




}
/// @nodoc
class _$CounterDemoDialogResponse_ConfirmCopyWithImpl<$Res>
    implements $CounterDemoDialogResponse_ConfirmCopyWith<$Res> {
  _$CounterDemoDialogResponse_ConfirmCopyWithImpl(this._self, this._then);

  final CounterDemoDialogResponse_Confirm _self;
  final $Res Function(CounterDemoDialogResponse_Confirm) _then;

/// Create a copy of CounterDemoDialogResponse
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(CounterDemoDialogResponse_Confirm(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$CounterDemoEvent {

 String get message;
/// Create a copy of CounterDemoEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoEventCopyWith<CounterDemoEvent> get copyWith => _$CounterDemoEventCopyWithImpl<CounterDemoEvent>(this as CounterDemoEvent, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoEvent&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'CounterDemoEvent(message: $message)';
}


}

/// @nodoc
abstract mixin class $CounterDemoEventCopyWith<$Res>  {
  factory $CounterDemoEventCopyWith(CounterDemoEvent value, $Res Function(CounterDemoEvent) _then) = _$CounterDemoEventCopyWithImpl;
@useResult
$Res call({
 String message
});




}
/// @nodoc
class _$CounterDemoEventCopyWithImpl<$Res>
    implements $CounterDemoEventCopyWith<$Res> {
  _$CounterDemoEventCopyWithImpl(this._self, this._then);

  final CounterDemoEvent _self;
  final $Res Function(CounterDemoEvent) _then;

/// Create a copy of CounterDemoEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? message = null,}) {
  return _then(_self.copyWith(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterDemoEvent].
extension CounterDemoEventPatterns on CounterDemoEvent {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterDemoEvent_Notify value)?  notify,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterDemoEvent_Notify() when notify != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterDemoEvent_Notify value)  notify,}){
final _that = this;
switch (_that) {
case CounterDemoEvent_Notify():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterDemoEvent_Notify value)?  notify,}){
final _that = this;
switch (_that) {
case CounterDemoEvent_Notify() when notify != null:
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
case CounterDemoEvent_Notify() when notify != null:
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
case CounterDemoEvent_Notify():
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
case CounterDemoEvent_Notify() when notify != null:
return notify(_that.message);case _:
  return null;

}
}

}

/// @nodoc


class CounterDemoEvent_Notify extends CounterDemoEvent {
  const CounterDemoEvent_Notify({required this.message}): super._();
  

@override final  String message;

/// Create a copy of CounterDemoEvent
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoEvent_NotifyCopyWith<CounterDemoEvent_Notify> get copyWith => _$CounterDemoEvent_NotifyCopyWithImpl<CounterDemoEvent_Notify>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoEvent_Notify&&(identical(other.message, message) || other.message == message));
}


@override
int get hashCode => Object.hash(runtimeType,message);

@override
String toString() {
  return 'CounterDemoEvent.notify(message: $message)';
}


}

/// @nodoc
abstract mixin class $CounterDemoEvent_NotifyCopyWith<$Res> implements $CounterDemoEventCopyWith<$Res> {
  factory $CounterDemoEvent_NotifyCopyWith(CounterDemoEvent_Notify value, $Res Function(CounterDemoEvent_Notify) _then) = _$CounterDemoEvent_NotifyCopyWithImpl;
@override @useResult
$Res call({
 String message
});




}
/// @nodoc
class _$CounterDemoEvent_NotifyCopyWithImpl<$Res>
    implements $CounterDemoEvent_NotifyCopyWith<$Res> {
  _$CounterDemoEvent_NotifyCopyWithImpl(this._self, this._then);

  final CounterDemoEvent_Notify _self;
  final $Res Function(CounterDemoEvent_Notify) _then;

/// Create a copy of CounterDemoEvent
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? message = null,}) {
  return _then(CounterDemoEvent_Notify(
message: null == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$CounterDemoIn {

 String get text;
/// Create a copy of CounterDemoIn
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoInCopyWith<CounterDemoIn> get copyWith => _$CounterDemoInCopyWithImpl<CounterDemoIn>(this as CounterDemoIn, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoIn&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'CounterDemoIn(text: $text)';
}


}

/// @nodoc
abstract mixin class $CounterDemoInCopyWith<$Res>  {
  factory $CounterDemoInCopyWith(CounterDemoIn value, $Res Function(CounterDemoIn) _then) = _$CounterDemoInCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$CounterDemoInCopyWithImpl<$Res>
    implements $CounterDemoInCopyWith<$Res> {
  _$CounterDemoInCopyWithImpl(this._self, this._then);

  final CounterDemoIn _self;
  final $Res Function(CounterDemoIn) _then;

/// Create a copy of CounterDemoIn
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? text = null,}) {
  return _then(_self.copyWith(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterDemoIn].
extension CounterDemoInPatterns on CounterDemoIn {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterDemoIn_Receive value)?  receive,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterDemoIn_Receive() when receive != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterDemoIn_Receive value)  receive,}){
final _that = this;
switch (_that) {
case CounterDemoIn_Receive():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterDemoIn_Receive value)?  receive,}){
final _that = this;
switch (_that) {
case CounterDemoIn_Receive() when receive != null:
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
case CounterDemoIn_Receive() when receive != null:
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
case CounterDemoIn_Receive():
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
case CounterDemoIn_Receive() when receive != null:
return receive(_that.text);case _:
  return null;

}
}

}

/// @nodoc


class CounterDemoIn_Receive extends CounterDemoIn {
  const CounterDemoIn_Receive({required this.text}): super._();
  

@override final  String text;

/// Create a copy of CounterDemoIn
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoIn_ReceiveCopyWith<CounterDemoIn_Receive> get copyWith => _$CounterDemoIn_ReceiveCopyWithImpl<CounterDemoIn_Receive>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoIn_Receive&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'CounterDemoIn.receive(text: $text)';
}


}

/// @nodoc
abstract mixin class $CounterDemoIn_ReceiveCopyWith<$Res> implements $CounterDemoInCopyWith<$Res> {
  factory $CounterDemoIn_ReceiveCopyWith(CounterDemoIn_Receive value, $Res Function(CounterDemoIn_Receive) _then) = _$CounterDemoIn_ReceiveCopyWithImpl;
@override @useResult
$Res call({
 String text
});




}
/// @nodoc
class _$CounterDemoIn_ReceiveCopyWithImpl<$Res>
    implements $CounterDemoIn_ReceiveCopyWith<$Res> {
  _$CounterDemoIn_ReceiveCopyWithImpl(this._self, this._then);

  final CounterDemoIn_Receive _self;
  final $Res Function(CounterDemoIn_Receive) _then;

/// Create a copy of CounterDemoIn
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(CounterDemoIn_Receive(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$CounterDemoOut {

 String get text;
/// Create a copy of CounterDemoOut
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoOutCopyWith<CounterDemoOut> get copyWith => _$CounterDemoOutCopyWithImpl<CounterDemoOut>(this as CounterDemoOut, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoOut&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'CounterDemoOut(text: $text)';
}


}

/// @nodoc
abstract mixin class $CounterDemoOutCopyWith<$Res>  {
  factory $CounterDemoOutCopyWith(CounterDemoOut value, $Res Function(CounterDemoOut) _then) = _$CounterDemoOutCopyWithImpl;
@useResult
$Res call({
 String text
});




}
/// @nodoc
class _$CounterDemoOutCopyWithImpl<$Res>
    implements $CounterDemoOutCopyWith<$Res> {
  _$CounterDemoOutCopyWithImpl(this._self, this._then);

  final CounterDemoOut _self;
  final $Res Function(CounterDemoOut) _then;

/// Create a copy of CounterDemoOut
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? text = null,}) {
  return _then(_self.copyWith(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CounterDemoOut].
extension CounterDemoOutPatterns on CounterDemoOut {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( CounterDemoOut_Send value)?  send,required TResult orElse(),}){
final _that = this;
switch (_that) {
case CounterDemoOut_Send() when send != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( CounterDemoOut_Send value)  send,}){
final _that = this;
switch (_that) {
case CounterDemoOut_Send():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( CounterDemoOut_Send value)?  send,}){
final _that = this;
switch (_that) {
case CounterDemoOut_Send() when send != null:
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
case CounterDemoOut_Send() when send != null:
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
case CounterDemoOut_Send():
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
case CounterDemoOut_Send() when send != null:
return send(_that.text);case _:
  return null;

}
}

}

/// @nodoc


class CounterDemoOut_Send extends CounterDemoOut {
  const CounterDemoOut_Send({required this.text}): super._();
  

@override final  String text;

/// Create a copy of CounterDemoOut
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CounterDemoOut_SendCopyWith<CounterDemoOut_Send> get copyWith => _$CounterDemoOut_SendCopyWithImpl<CounterDemoOut_Send>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CounterDemoOut_Send&&(identical(other.text, text) || other.text == text));
}


@override
int get hashCode => Object.hash(runtimeType,text);

@override
String toString() {
  return 'CounterDemoOut.send(text: $text)';
}


}

/// @nodoc
abstract mixin class $CounterDemoOut_SendCopyWith<$Res> implements $CounterDemoOutCopyWith<$Res> {
  factory $CounterDemoOut_SendCopyWith(CounterDemoOut_Send value, $Res Function(CounterDemoOut_Send) _then) = _$CounterDemoOut_SendCopyWithImpl;
@override @useResult
$Res call({
 String text
});




}
/// @nodoc
class _$CounterDemoOut_SendCopyWithImpl<$Res>
    implements $CounterDemoOut_SendCopyWith<$Res> {
  _$CounterDemoOut_SendCopyWithImpl(this._self, this._then);

  final CounterDemoOut_Send _self;
  final $Res Function(CounterDemoOut_Send) _then;

/// Create a copy of CounterDemoOut
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? text = null,}) {
  return _then(CounterDemoOut_Send(
text: null == text ? _self.text : text // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
