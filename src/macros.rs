pub use paste::paste;

#[macro_export]
macro_rules! exists {
  ( $exists:ident
    ( $name:ident : $type:ty ) =>
    $proof:ident
    $( < $( $proof_param:ident ),+ $(,) ? > )?
    ( $( $suchthat:ident $( : $suchtype:ty )? ),* $(,)? )
  ) => {
    $crate::macros::paste! {
      pub struct [< $proof:camel >] <
        [< $name:camel Val >] : $crate::HasType<$type>,
        $( $( $proof_param, )* )?
        $( [< $suchthat:camel Val >] $( : $crate::HasType<$suchtype> )?  ),*
      >
      (
        ::core::marker::PhantomData<(
          [< $name:camel Val >],
          $( $( $proof_param, )* )?
          $( [< $suchthat:camel Val >] ),*
        )>
      );

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
          < [< $name:camel Val >],
            $( $( $proof_param, )* )?
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
