pub use paste::paste;

/**
 Macro to define new named value coupled with proofs
 in the form of dependent pairs.

 Suppose we have a named `u64` integer and we want to
 add 1 to the value using
 [checked addition](u64::checked_add)
 and return a new named `u64`. Along the way,
 we also want to construct a proof that shows that the new
 `u64` value is a _successor_ of the original value.

 We can define our successor proof similar to the other
 proofs that are defined by [`proof!`], but when
 we try to define our `add_one` function, we would
 hit a roadblock like follows:

 ```rust,ignore
 mod successor {
   use mononym::*;

   proof! {
     IsSuccessor(succ: u64, pred: u64);
   }

   pub fn add_one<NumVal: HasType<u64>>(
     seed: Seed<impl Name>,
     num: &Named<NumVal, u64>,
   ) -> Option<(Named<impl Name, u64>, IsSuccessor<??, NumVal>)>
   {
     todo!()
   }
 }
 ```

 The issue with the return type of our `add_one` function above is that
 the type `IsSuccessor<??, NumVal>` _depends_ on the fresh name type
 `impl Name` in `Named<impl Name, 64>` to fill in the hole of `??`.
 This construct is known as a
 [_dependent pair_](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs)
 in languages with dependent types such as Idris.

 Although Rust do not have built in support for constructing dependent
 pairs, we can still emulate that by wrapping the two return types
 inside a new struct that we will call `ExistSuccessor`:

 ```rust
 mod successor {
   use mononym::*;
   use core::marker::PhantomData;

   proof! {
     IsSuccessor(succ: u64, pred: u64);
   }

   pub struct ExistSuccessor<
     SuccVal: HasType<u64>,
     PredVal: HasType<u64>,
   > {
     successor: Named<SuccVal, u64>,
     is_successor: IsSuccessor<SuccVal, PredVal>,
   }

   fn new_exist_successor<
     PredVal: HasType<u64>,
   >
   ( seed: Seed<impl Name>,
     succ: u64,
   ) -> ExistSuccessor<impl HasType<u64>, PredVal>
   {
     ExistSuccessor {
       successor: seed.new_named(succ),
       is_successor: IsSuccessor::new(),
     }
   }

   pub fn add_one<NumVal: HasType<u64>>(
     seed: Seed<impl Name>,
     num: &Named<NumVal, u64>,
   ) -> Option<ExistSuccessor<impl HasType<u64>, NumVal>>
   {
     num.value()
       .checked_add(1)
       .map(|succ| new_exist_successor(seed, succ))
   }
 }
 ```

 The `ExistSuccessor` struct allows the same type parameter `SuccVal`
 to be used in both `Named` and `IsSuccessor`. Using that, we can
 redefine our `add_one` function to return the type
 `ExistSuccessor<impl HasType<u64>, NumVal>`, which the fresh
 name type `impl HasType<u64>` can be used in both places.

 Although the above solution works, there are quite a lot of
 boilerplate required to define just one dependent pair type.
 Therefore the `exists!` macro is provided so that the same
 definition above can be simplified into a single line definition
 as shown below:

 ```rust
 mod successor {
   use mononym::*;
   use core::marker::PhantomData;

   exists! {
     ExistSuccessor(succ: u64) => IsSuccessor(pred: u64);
   }

   pub fn add_one<NumVal: HasType<u64>>(
     seed: Seed<impl Name>,
     num: &Named<NumVal, u64>,
   ) -> Option<ExistSuccessor<impl HasType<u64>, NumVal>>
   {
     num.value()
       .checked_add(1)
       .map(|succ| new_exist_successor(seed, succ))
   }
 }
 ```

 Note that the new version of the `successor` module is the same as
 the original version. The `exists!` takes care of generating
 the various struct definitions, as well as calling [`proof!`]
 to generate the `IsSuccessor` proof type.

 One limitation of the `exists!` macro is that the existential name
 is always in the first position of the following proof type.
 That is, the definition

 ```
 # use mononym::*;
 exists! { ExistSuccessor(succ: u64) => IsSuccessor(pred: u64); }
 ```

 leads to the call to

 ```
 # use mononym::*;
 proof! { IsSuccessor(succ: u64, pred: u64); }
 ```

 Therefore we cannot switch the position of the variables in
 the proof type, such as having

 ```
 # use mononym::*;
 proof! { IsSuccessor(pred: u64, succ: u64); }
 ```

*/
#[macro_export]
macro_rules! exists {
  ( $(
      $exists:ident
      ( $name:ident : $type:ty ) =>
      $proof:ident
      $( < $( $proof_param:ident ),+ $(,) ? > )?
      ( $( $suchthat:ident $( : $suchtype:ty )? ),* $(,)? );
    )*

  ) => {
    $(
      $crate::exists_single! {
        $exists
        ( $name : $type ) =>
        $proof
        $( < $( $proof_param ),* > )*
        ( $( $suchthat $( : $suchtype )* ),* );
      }
    )*
  }
}

#[macro_export]
macro_rules! exists_single {
  ( $exists:ident
    ( $name:ident : $type:ty ) =>
    $proof:ident
    $( < $( $proof_param:ident ),+ $(,) ? > )?
    ( $( $suchthat:ident $( : $suchtype:ty )? ),* $(,)? )
    $(;)?
  ) => {
    $crate::proof_single! {
      $proof
      $( < $( $proof_param ),* > )?
      ( $name : $type, $( $suchthat $( : $suchtype )? ),* )
    }

    $crate::macros::paste! {
      pub struct [< $exists:camel >]
      <
        [< $name:camel Val >] : $crate::HasType<$type>,
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >] $( : $crate::HasType<$suchtype> )?  ),*
      >
      {
        pub [< $name:snake >] : $crate::Named<
          [< $name:camel Val >],
          $type
        >,
        pub [< $proof:snake >] :
          [< $proof:camel >]
          <
            $( $( $proof_param, )* )?
            [< $name:camel Val >],
            $( [< $suchthat:camel Val >] ),*
          >,
      }

      fn [< new_ $exists:snake >]
      <
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >] $( : $crate::HasType<$suchtype> )?  ),*
      >
      (
        seed : $crate::Seed<impl $crate::Name>,
        [< $name:snake >] : $type,
      ) ->
        [< $exists:camel >]
        < impl $crate::HasType<$type>,
          $( $( $proof_param, )* )?
          $( [< $suchthat:camel Val >] ),*
        >
      {
        [< $exists:camel >] {
          [< $name:snake >]: $crate::Seed::new_named(seed, [< $name:snake >]),
          [< $proof:snake >] : [< $proof:camel >] ( ::core::marker::PhantomData )
        }
      }
    }
  }
}

/**
 Macro to help define a new proof type.

 Suppose we want to write a module that tests whether a
 named `i64` is greater or equal to 0, and return a proof
 of it. We could define the proof type manually as follows:

 ```rust
 mod natural {
   use mononym::*;
   use core::marker::PhantomData;

   pub struct IsNatural<NumVal: HasType<i64>>(PhantomData<NumVal>);

   impl <NumVal: HasType<i64>> IsNatural<NumVal> {
     fn new() -> Self {
       Self(PhantomData)
     }
   }

   pub fn is_natural<NumVal: HasType<i64>>(
     num: &Named<NumVal, i64>,
   ) -> Option<IsNatural<NumVal>>
   {
     if *num.value() >= 0 {
       Some(IsNatural::new())
     } else {
       None
     }
   }
 }
 ```

 We define `IsNatural` as a proof type for a named `i64` value
 with the name `NumVal` that shows that the number is greater
 or equal to 0. The type has a private
 [`PhantomData`](core::marker::PhantomData) body, as the
 existence of the proof value alone is sufficient. It provides
 a private `new()` method that allows functions to construct
 the proof object.

 The function `is_natural` then accepts a named number value
 with the name `NumVal`, and only constructs the
 proof `IsNatural<NumVal>` using `IsNatural::new()`
 if the condition is satisfied.

 As we define more proof types, the need to manually define
 the proof structs and methods can become tedious. The
 `proof!` macro simplifies the definition such as
 above so that we can write our code as follows:

 ```rust
 mod natural {
   use mononym::*;

   proof! {
     IsNatural(num: i64);
   }

   pub fn is_natural<NumVal: HasType<i64>>(
     num: &Named<NumVal, i64>,
   ) -> Option<IsNatural<NumVal>>
   {
     if *num.value() >= 0 {
       Some(IsNatural::new())
     } else {
       None
     }
   }
 }
 ```

 Note that the new version of the `natural` module is the same as
 the original version. `proof!` takes care of generating
 the struct definition and the private `new` method, so that
 we do not need to keep repeating the same boilerplate definition.
*/
#[macro_export]
macro_rules! proof {
  ( $(
      $proof:ident
      $( < $( $proof_param:ident ),+ $(,) ? > )?
      ( $( $suchthat:ident $( : $suchtype:ty )? ),* $(,)? );
    )+
  ) => {
    $(
      $crate::proof_single! {
        $proof
        $( < $( $proof_param ),* > )?
        ( $( $suchthat $( : $suchtype )? ),* );
      }
    )*
  }
}

#[macro_export]
macro_rules! proof_single {
  ( $proof:ident
    $( < $( $proof_param:ident ),+ $(,) ? > )?
    ( $( $suchthat:ident $( : $suchtype:ty )? ),* $(,)? )
    $(;)?
  ) => {
    $crate::macros::paste! {
      pub struct [< $proof:camel >] <
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >] $( : $crate::HasType<$suchtype> )?  ),*
      >
      (
        ::core::marker::PhantomData<(
          $( $( $proof_param, )* )?
          $( [< $suchthat:camel Val >] ),*
        )>
      );

      impl
      <
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >] $( : $crate::HasType<$suchtype> )?  ),*
      >
      [< $proof:camel >]
      <
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >]  ),*
      >
      {
        fn new () -> Self
        {
          [< $proof:camel >] (
            ::core::marker::PhantomData
          )
        }
      }
    }
  }
}
