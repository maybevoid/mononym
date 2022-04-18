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

fn main()
{
  use less_than_eq::*;
  use mononym::*;
  use non_zero::*;
  use percentage::*;

  with_seed(|into_seed| {
    let seed = into_seed.into_seed();
    let (seed1, seed2) = replicate_seed(seed);
    let x: Named<_, u32> = new_named(seed1, 2);
    let y: Named<_, u32> = new_named(seed2, 4);

    let x_is_less_than_y: LessThanEq<_, _> =
      check_less_than_eq(&x, &y).expect("should get proof that x <= y");

    assert!(check_less_than_eq(&y, &x).is_none());

    let y_not_zero =
      check_non_zero(&y).expect("should get proof that y is non zero");

    let percent = to_percentage(&x, &y, &x_is_less_than_y, &y_not_zero);

    assert_eq!(percent, 50.0);

    println!("percentage of {}/{} is {}%", x.value(), y.value(), percent);
  })
}
