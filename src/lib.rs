#![no_std]
#![doc = include_str!("../README.md")]

mod named;

pub mod proof;

pub mod macros;

pub use named::{
  with_seed,
  HasType,
  Name,
  Named,
  Seed,
};

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod test;
