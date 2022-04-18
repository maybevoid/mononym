pub use paste::paste;

#[doc = include_str!("../docs/exists_macro.md")]
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
        seed : impl $crate::Seed,
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

#[doc = include_str!("../docs/proof_macro.md")]
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
