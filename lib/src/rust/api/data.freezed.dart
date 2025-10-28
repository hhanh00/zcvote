// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'data.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$CandidateChoice {

 String get choice;
/// Create a copy of CandidateChoice
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$CandidateChoiceCopyWith<CandidateChoice> get copyWith => _$CandidateChoiceCopyWithImpl<CandidateChoice>(this as CandidateChoice, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is CandidateChoice&&(identical(other.choice, choice) || other.choice == choice));
}


@override
int get hashCode => Object.hash(runtimeType,choice);

@override
String toString() {
  return 'CandidateChoice(choice: $choice)';
}


}

/// @nodoc
abstract mixin class $CandidateChoiceCopyWith<$Res>  {
  factory $CandidateChoiceCopyWith(CandidateChoice value, $Res Function(CandidateChoice) _then) = _$CandidateChoiceCopyWithImpl;
@useResult
$Res call({
 String choice
});




}
/// @nodoc
class _$CandidateChoiceCopyWithImpl<$Res>
    implements $CandidateChoiceCopyWith<$Res> {
  _$CandidateChoiceCopyWithImpl(this._self, this._then);

  final CandidateChoice _self;
  final $Res Function(CandidateChoice) _then;

/// Create a copy of CandidateChoice
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? choice = null,}) {
  return _then(_self.copyWith(
choice: null == choice ? _self.choice : choice // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [CandidateChoice].
extension CandidateChoicePatterns on CandidateChoice {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _CandidateChoice value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _CandidateChoice() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _CandidateChoice value)  $default,){
final _that = this;
switch (_that) {
case _CandidateChoice():
return $default(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _CandidateChoice value)?  $default,){
final _that = this;
switch (_that) {
case _CandidateChoice() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String choice)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _CandidateChoice() when $default != null:
return $default(_that.choice);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String choice)  $default,) {final _that = this;
switch (_that) {
case _CandidateChoice():
return $default(_that.choice);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String choice)?  $default,) {final _that = this;
switch (_that) {
case _CandidateChoice() when $default != null:
return $default(_that.choice);case _:
  return null;

}
}

}

/// @nodoc


class _CandidateChoice implements CandidateChoice {
  const _CandidateChoice({required this.choice});
  

@override final  String choice;

/// Create a copy of CandidateChoice
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$CandidateChoiceCopyWith<_CandidateChoice> get copyWith => __$CandidateChoiceCopyWithImpl<_CandidateChoice>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _CandidateChoice&&(identical(other.choice, choice) || other.choice == choice));
}


@override
int get hashCode => Object.hash(runtimeType,choice);

@override
String toString() {
  return 'CandidateChoice(choice: $choice)';
}


}

/// @nodoc
abstract mixin class _$CandidateChoiceCopyWith<$Res> implements $CandidateChoiceCopyWith<$Res> {
  factory _$CandidateChoiceCopyWith(_CandidateChoice value, $Res Function(_CandidateChoice) _then) = __$CandidateChoiceCopyWithImpl;
@override @useResult
$Res call({
 String choice
});




}
/// @nodoc
class __$CandidateChoiceCopyWithImpl<$Res>
    implements _$CandidateChoiceCopyWith<$Res> {
  __$CandidateChoiceCopyWithImpl(this._self, this._then);

  final _CandidateChoice _self;
  final $Res Function(_CandidateChoice) _then;

/// Create a copy of CandidateChoice
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? choice = null,}) {
  return _then(_CandidateChoice(
choice: null == choice ? _self.choice : choice // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$Election {

 String get name; String? get seed; int get startHeight; int get endHeight; List<Question> get questions; bool get signatureRequired; bool get locked;
/// Create a copy of Election
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ElectionCopyWith<Election> get copyWith => _$ElectionCopyWithImpl<Election>(this as Election, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Election&&(identical(other.name, name) || other.name == name)&&(identical(other.seed, seed) || other.seed == seed)&&(identical(other.startHeight, startHeight) || other.startHeight == startHeight)&&(identical(other.endHeight, endHeight) || other.endHeight == endHeight)&&const DeepCollectionEquality().equals(other.questions, questions)&&(identical(other.signatureRequired, signatureRequired) || other.signatureRequired == signatureRequired)&&(identical(other.locked, locked) || other.locked == locked));
}


@override
int get hashCode => Object.hash(runtimeType,name,seed,startHeight,endHeight,const DeepCollectionEquality().hash(questions),signatureRequired,locked);

@override
String toString() {
  return 'Election(name: $name, seed: $seed, startHeight: $startHeight, endHeight: $endHeight, questions: $questions, signatureRequired: $signatureRequired, locked: $locked)';
}


}

/// @nodoc
abstract mixin class $ElectionCopyWith<$Res>  {
  factory $ElectionCopyWith(Election value, $Res Function(Election) _then) = _$ElectionCopyWithImpl;
@useResult
$Res call({
 String name, String? seed, int startHeight, int endHeight, List<Question> questions, bool signatureRequired, bool locked
});




}
/// @nodoc
class _$ElectionCopyWithImpl<$Res>
    implements $ElectionCopyWith<$Res> {
  _$ElectionCopyWithImpl(this._self, this._then);

  final Election _self;
  final $Res Function(Election) _then;

/// Create a copy of Election
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? name = null,Object? seed = freezed,Object? startHeight = null,Object? endHeight = null,Object? questions = null,Object? signatureRequired = null,Object? locked = null,}) {
  return _then(_self.copyWith(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,seed: freezed == seed ? _self.seed : seed // ignore: cast_nullable_to_non_nullable
as String?,startHeight: null == startHeight ? _self.startHeight : startHeight // ignore: cast_nullable_to_non_nullable
as int,endHeight: null == endHeight ? _self.endHeight : endHeight // ignore: cast_nullable_to_non_nullable
as int,questions: null == questions ? _self.questions : questions // ignore: cast_nullable_to_non_nullable
as List<Question>,signatureRequired: null == signatureRequired ? _self.signatureRequired : signatureRequired // ignore: cast_nullable_to_non_nullable
as bool,locked: null == locked ? _self.locked : locked // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}

}


/// Adds pattern-matching-related methods to [Election].
extension ElectionPatterns on Election {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _Election value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _Election() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _Election value)  $default,){
final _that = this;
switch (_that) {
case _Election():
return $default(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _Election value)?  $default,){
final _that = this;
switch (_that) {
case _Election() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String name,  String? seed,  int startHeight,  int endHeight,  List<Question> questions,  bool signatureRequired,  bool locked)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Election() when $default != null:
return $default(_that.name,_that.seed,_that.startHeight,_that.endHeight,_that.questions,_that.signatureRequired,_that.locked);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String name,  String? seed,  int startHeight,  int endHeight,  List<Question> questions,  bool signatureRequired,  bool locked)  $default,) {final _that = this;
switch (_that) {
case _Election():
return $default(_that.name,_that.seed,_that.startHeight,_that.endHeight,_that.questions,_that.signatureRequired,_that.locked);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String name,  String? seed,  int startHeight,  int endHeight,  List<Question> questions,  bool signatureRequired,  bool locked)?  $default,) {final _that = this;
switch (_that) {
case _Election() when $default != null:
return $default(_that.name,_that.seed,_that.startHeight,_that.endHeight,_that.questions,_that.signatureRequired,_that.locked);case _:
  return null;

}
}

}

/// @nodoc


class _Election implements Election {
  const _Election({required this.name, this.seed, required this.startHeight, required this.endHeight, required final  List<Question> questions, required this.signatureRequired, required this.locked}): _questions = questions;
  

@override final  String name;
@override final  String? seed;
@override final  int startHeight;
@override final  int endHeight;
 final  List<Question> _questions;
@override List<Question> get questions {
  if (_questions is EqualUnmodifiableListView) return _questions;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_questions);
}

@override final  bool signatureRequired;
@override final  bool locked;

/// Create a copy of Election
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$ElectionCopyWith<_Election> get copyWith => __$ElectionCopyWithImpl<_Election>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Election&&(identical(other.name, name) || other.name == name)&&(identical(other.seed, seed) || other.seed == seed)&&(identical(other.startHeight, startHeight) || other.startHeight == startHeight)&&(identical(other.endHeight, endHeight) || other.endHeight == endHeight)&&const DeepCollectionEquality().equals(other._questions, _questions)&&(identical(other.signatureRequired, signatureRequired) || other.signatureRequired == signatureRequired)&&(identical(other.locked, locked) || other.locked == locked));
}


@override
int get hashCode => Object.hash(runtimeType,name,seed,startHeight,endHeight,const DeepCollectionEquality().hash(_questions),signatureRequired,locked);

@override
String toString() {
  return 'Election(name: $name, seed: $seed, startHeight: $startHeight, endHeight: $endHeight, questions: $questions, signatureRequired: $signatureRequired, locked: $locked)';
}


}

/// @nodoc
abstract mixin class _$ElectionCopyWith<$Res> implements $ElectionCopyWith<$Res> {
  factory _$ElectionCopyWith(_Election value, $Res Function(_Election) _then) = __$ElectionCopyWithImpl;
@override @useResult
$Res call({
 String name, String? seed, int startHeight, int endHeight, List<Question> questions, bool signatureRequired, bool locked
});




}
/// @nodoc
class __$ElectionCopyWithImpl<$Res>
    implements _$ElectionCopyWith<$Res> {
  __$ElectionCopyWithImpl(this._self, this._then);

  final _Election _self;
  final $Res Function(_Election) _then;

/// Create a copy of Election
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? name = null,Object? seed = freezed,Object? startHeight = null,Object? endHeight = null,Object? questions = null,Object? signatureRequired = null,Object? locked = null,}) {
  return _then(_Election(
name: null == name ? _self.name : name // ignore: cast_nullable_to_non_nullable
as String,seed: freezed == seed ? _self.seed : seed // ignore: cast_nullable_to_non_nullable
as String?,startHeight: null == startHeight ? _self.startHeight : startHeight // ignore: cast_nullable_to_non_nullable
as int,endHeight: null == endHeight ? _self.endHeight : endHeight // ignore: cast_nullable_to_non_nullable
as int,questions: null == questions ? _self._questions : questions // ignore: cast_nullable_to_non_nullable
as List<Question>,signatureRequired: null == signatureRequired ? _self.signatureRequired : signatureRequired // ignore: cast_nullable_to_non_nullable
as bool,locked: null == locked ? _self.locked : locked // ignore: cast_nullable_to_non_nullable
as bool,
  ));
}


}

/// @nodoc
mixin _$Question {

 String get question; List<CandidateChoice> get choices;
/// Create a copy of Question
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$QuestionCopyWith<Question> get copyWith => _$QuestionCopyWithImpl<Question>(this as Question, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Question&&(identical(other.question, question) || other.question == question)&&const DeepCollectionEquality().equals(other.choices, choices));
}


@override
int get hashCode => Object.hash(runtimeType,question,const DeepCollectionEquality().hash(choices));

@override
String toString() {
  return 'Question(question: $question, choices: $choices)';
}


}

/// @nodoc
abstract mixin class $QuestionCopyWith<$Res>  {
  factory $QuestionCopyWith(Question value, $Res Function(Question) _then) = _$QuestionCopyWithImpl;
@useResult
$Res call({
 String question, List<CandidateChoice> choices
});




}
/// @nodoc
class _$QuestionCopyWithImpl<$Res>
    implements $QuestionCopyWith<$Res> {
  _$QuestionCopyWithImpl(this._self, this._then);

  final Question _self;
  final $Res Function(Question) _then;

/// Create a copy of Question
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? question = null,Object? choices = null,}) {
  return _then(_self.copyWith(
question: null == question ? _self.question : question // ignore: cast_nullable_to_non_nullable
as String,choices: null == choices ? _self.choices : choices // ignore: cast_nullable_to_non_nullable
as List<CandidateChoice>,
  ));
}

}


/// Adds pattern-matching-related methods to [Question].
extension QuestionPatterns on Question {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _Question value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _Question() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _Question value)  $default,){
final _that = this;
switch (_that) {
case _Question():
return $default(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _Question value)?  $default,){
final _that = this;
switch (_that) {
case _Question() when $default != null:
return $default(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String question,  List<CandidateChoice> choices)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _Question() when $default != null:
return $default(_that.question,_that.choices);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String question,  List<CandidateChoice> choices)  $default,) {final _that = this;
switch (_that) {
case _Question():
return $default(_that.question,_that.choices);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String question,  List<CandidateChoice> choices)?  $default,) {final _that = this;
switch (_that) {
case _Question() when $default != null:
return $default(_that.question,_that.choices);case _:
  return null;

}
}

}

/// @nodoc


class _Question implements Question {
  const _Question({required this.question, required final  List<CandidateChoice> choices}): _choices = choices;
  

@override final  String question;
 final  List<CandidateChoice> _choices;
@override List<CandidateChoice> get choices {
  if (_choices is EqualUnmodifiableListView) return _choices;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableListView(_choices);
}


/// Create a copy of Question
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$QuestionCopyWith<_Question> get copyWith => __$QuestionCopyWithImpl<_Question>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _Question&&(identical(other.question, question) || other.question == question)&&const DeepCollectionEquality().equals(other._choices, _choices));
}


@override
int get hashCode => Object.hash(runtimeType,question,const DeepCollectionEquality().hash(_choices));

@override
String toString() {
  return 'Question(question: $question, choices: $choices)';
}


}

/// @nodoc
abstract mixin class _$QuestionCopyWith<$Res> implements $QuestionCopyWith<$Res> {
  factory _$QuestionCopyWith(_Question value, $Res Function(_Question) _then) = __$QuestionCopyWithImpl;
@override @useResult
$Res call({
 String question, List<CandidateChoice> choices
});




}
/// @nodoc
class __$QuestionCopyWithImpl<$Res>
    implements _$QuestionCopyWith<$Res> {
  __$QuestionCopyWithImpl(this._self, this._then);

  final _Question _self;
  final $Res Function(_Question) _then;

/// Create a copy of Question
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? question = null,Object? choices = null,}) {
  return _then(_Question(
question: null == question ? _self.question : question // ignore: cast_nullable_to_non_nullable
as String,choices: null == choices ? _self._choices : choices // ignore: cast_nullable_to_non_nullable
as List<CandidateChoice>,
  ));
}


}

// dart format on
