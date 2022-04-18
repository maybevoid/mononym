## Demo

As a quick demo, Mononym can be used to construct named values and
create proofs that relate multiple modules as follows:

```rust
pub mod less_than_eq
{
  use mononym::*;

  proof! {
    LessThanEq(x: u32, y: u32);
  }

  pub fn check_less_than_eq<XVal: HasType<u32>, YVal: HasType<u32>>(
    x: &Named<XVal, u32>,
    y: &Named<YVal, u32>,
  ) -> Option<LessThanEq<XVal, YVal>>
  {
    if x.value() <= y.value() {
      Some(LessThanEq::new())
    } else {
      None
    }
  }
}

use less_than_eq::*;
use mononym::*;

with_seed(|seed| {
  let (seed1, seed2) = replicate_seed(seed.into_seed());
  let x: Named<_, u32> = new_named(seed1, 2);
  let y: Named<_, u32> = new_named(seed2, 4);

  let x_is_less_than_y: LessThanEq<_, _> =
    check_less_than_eq(&x, &y).expect("should get proof that x <= y");

  assert!(check_less_than_eq(&y, &x).is_none());
});
```

In the first part of the program, we define a `less_than_eq` module that defines a `LessThanEq` proof type using the [`proof!`] macro provided by Mononym. The function `check_less_than_eq` is then defined to accept two _named values_ `x` and `y`, with the type names `XVal` and `YVal` representing the values `x` and `y` at the type level. In the function body, it checks that if `x` (`XVal`) is indeed less than or equal to `y` (`YVal`), it would return the proof in the form of `LessThanEq<XVal, YVal>`.

In the second part of the program, we start our test by calling the [`with_seed`](crate::with_seed) function with a continuation closure. The `with_seed` function generates a fresh name seed type which is then given to the closure as the `seed` variable. We then call `seed.replicate()` to create two new copies of seed, because it is an _affine_ value in Rust that can be used at most once.

Each seed value can be used once to generate a named value. We then assign the variable `x` to `seed1.new_named(2)`, which has the type `Named<_, u32>`. Mononym provides the [`Named`](crate::Named) type to represent Rust values with unique name types. In this case, the `_` is used in the first position of the `Named` type for `x`, because the `new_named` method returns a `Named` type with an opaque type as the name.

Similarly, the variable `y` is assigned to `seed2.new_named(4)`. Note that even though `y` also have the type `Named<_, u32>`, it is in fact a different type than the type of `x`. With Mononym guaranteeing the uniqueness of name types, we can conceptually refer to the name type of `x` being `XVal`, and the name type of `y` being `YVal` which is different from `XVal`.

We call `check_less_than_eq(&x, &y)`, and expect the function to return a proof that `x` with a value of 2 is indeed less than `y` with a value of 4. Similarly, if we call `check_less_than_eq(&y, &x)`, we expect `None` to be returned and no proof should exist for 2 being greater than 4.

Note that unlike in fully formalized languages like Coq, Mononym do not check that the proof `LessThan<XVal, YVal>` can really only be constructed if and only if `x` <= `y`. It is up to the implementer of functions such as `check_less_than` to ensure that the construction of proofs match the underlying invariant.

Nevertheless, proofs that are constructed using mononym are useful for encoding pre- and post-conditions for functions so that they can be composed in a declarative way. For example, we can define another `non_zero` module that produce proof that a number is non-zero, and together we can create a `percentage` module that converts the division of two numbers into percentage form if and only if x <= y and y is not zero:

```rust
# mod example {
# pub mod less_than_eq
# {
#   use mononym::*;
#
#   proof! {
#     LessThanEq(x: u32, y: u32);
#   }
#
#   pub fn check_less_than_eq<XVal: HasType<u32>, YVal: HasType<u32>>(
#     x: &Named<XVal, u32>,
#     y: &Named<YVal, u32>,
#   ) -> Option<LessThanEq<XVal, YVal>>
#   {
#     if x.value() <= y.value() {
#       Some(LessThanEq::new())
#     } else {
#       None
#     }
#   }
# }
#
pub mod non_zero
{
  use mononym::*;

  proof! {
    NonZero(num: u32);
  }

  pub fn check_non_zero<NumVal: HasType<u32>>(
    x: &Named<NumVal, u32>
  ) -> Option<NonZero<NumVal>>
  {
    if *x.value() != 0 {
      Some(NonZero::new())
    } else {
      None
    }
  }
}

pub mod percentage
{
  use mononym::*;

  use super::{
    less_than_eq::LessThanEq,
    non_zero::NonZero,
  };

  pub fn to_percentage<
    NumeratorVal: HasType<u32>,
    DenominatorVal: HasType<u32>,
  >(
    x: &Named<NumeratorVal, u32>,
    y: &Named<DenominatorVal, u32>,
    _numerator_lte_denom: &LessThanEq<NumeratorVal, DenominatorVal>,
    _denom_not_zero: &NonZero<DenominatorVal>,
  ) -> f64
  {
    let x: f64 = (*x.value()).into();
    let y: f64 = (*y.value()).into();
    x / y * 100.0
  }
}

# }

use example::*;
use less_than_eq::*;
use mononym::*;
use non_zero::*;
use percentage::*;

with_seed(|seed| {
  let (seed1, seed2) = replicate_seed(seed.into_seed());
  let x: Named<_, u32> = new_named(seed1, 2);
  let y: Named<_, u32> = new_named(seed2, 4);

  let x_is_less_than_y: LessThanEq<_, _> =
    check_less_than_eq(&x, &y).expect("should get proof that x <= y");

  let y_not_zero =
    check_non_zero(&y).expect("should get proof that y is non zero");

  let percent = to_percentage(&x, &y, &x_is_less_than_y, &y_not_zero);

  println!("percentage of {}/{} is {}%", x.value(), y.value(), percent);
})
```

Similar to the `less_than_eq` module, the `check_non_zero` function in the `non_zero` module checks and may return a proof `NonZero<NumVal>` for a value being non-zero.

The `percentage` module then defines the `to_percentage` function to make use of both the proofs of `LessThanEq<XVal, YVal>` and `NonZero<YVal>` before returning a `f64` percentage value. Assuming that the proofs in `less_than_eq` and `non_zero` are constructed properly, we can guarantee that the `f64` returned by `to_percentage` is always between 0 and 100.

Using Mononym, the `to_percentage` function can be defined as a _total_ function, as in it does not need to return an `Option<f64>` or `Result<f64, Error>` to handle the cases where either the numerator or denominator values are invalid.

Although we cannot use Rust to formally prove that `to_percentage` will always return a valid percentage value between 0 and 100, Mononym can help reduce significantly the surface area of code that can potentially violate such invariant.

Since we know that the proofs `LessThanEq` and `NonZero` can only be produced by `less_than_eq` and `non_zero`, we do not need to worry about any other potential cases that `to_percentage` can be given invalid arguments, no matter how large and how complex our code base become.

We can also use Mononym together with techniques such as [property based testing](https://github.com/BurntSushi/quickcheck) to further ensure that the behavior of `to_percentage` is correct. In such property-based test, the random generator can attempt to randomly generate named values, and call `to_percentage` only when valid proofs can be constructed using `less_than_eq` and `non_zero`. This would significantly reduce the number of test cases needed, as compared to a brute force generator that have to test the function with the cartesian product of `u32` * `u32`. (Note that integration with testing framework is still a future work planned for Mononym)
