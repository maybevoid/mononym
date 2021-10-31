#![no_std]

/*!
 Mononym is a library for creating unique type-level names for each value
 in Rust. The core type `Named<Name, T>` represents a named value of type
 `T` with a unique type `Name` as its name. Mononym guarantees that there
 can be no two values with the same name. With that, the `Name` type
 serves as a unique representation of a Rust value at the type level.

 Mononym enables the use of the design pattern
 [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf) in Rust.
 It provides macros that simplify the definition of
 [dependent pairs](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs)
 and proof objects in Rust. Although there is still limited support for
 a full dependently-typed programming in Rust, Mononym helps us move a
 small step toward that direction by making it possible to refer to
 values in types.

 See [`docs::implementation`] for how `mononym` implements unique name
 generation in Rust.
*/

pub mod named;

pub mod proof;

pub mod macros;

pub use named::{
  with_seed,
  HasType,
  Life,
  Name,
  Named,
  Seed,
};

#[cfg(doc)]
pub mod docs
{
  /*!
   This is a rustdoc-only module providing in depth documentation
   on the library.
  */

  pub mod implementation
  {
    #![doc = include_str!("../docs/implementation.md")]
  }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod test;
