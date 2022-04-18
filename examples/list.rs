mod list_size
{
  use mononym::*;

  // Emulates the following dependent pair in Idris:
  //  (t: Type) -> (list: Vec t) -> (size: usize ** ListHasSize size list)
  exists! {
    ExistSize(size: usize) => ListHasSize<T>(list: Vec<T>);
  }

  pub fn list_size<T, ListVal: HasType<Vec<T>>>(
    seed: impl Seed,
    list: &Named<ListVal, Vec<T>>,
  ) -> ExistSize<impl HasType<usize>, T, ListVal>
  {
    let size = list.value().len();
    new_exist_size(seed, size)
  }
}

mod list_positive
{
  use mononym::*;

  exists! {
    ExistPositives(count: usize) => ListHasPositives(list: Vec<i64>);
  }

  pub fn count_positive_integers<ListVal: HasType<Vec<i64>>>(
    seed: impl Seed,
    list: &Named<ListVal, Vec<i64>>,
  ) -> ExistPositives<impl HasType<usize>, ListVal>
  {
    let count = list.value().iter().filter(|x| **x > 0).count();

    new_exist_positives(seed, count)
  }
}

mod greater_half
{
  use mononym::*;

  proof! {
    GreaterThanHalf(numerator: usize, denominator: usize);
  }

  pub fn greater_than_half<
    NumeratorVal: HasType<usize>,
    DenominatorVal: HasType<usize>,
  >(
    x: &Named<NumeratorVal, usize>,
    y: &Named<DenominatorVal, usize>,
  ) -> Option<GreaterThanHalf<NumeratorVal, DenominatorVal>>
  {
    if *x.value() * 2 > *y.value() {
      Some(GreaterThanHalf::new())
    } else {
      None
    }
  }
}

mod greater_than_half_positive
{
  use mononym::*;

  use super::{
    greater_half::GreaterThanHalf,
    list_positive::ListHasPositives,
    list_size::ListHasSize,
  };

  proof! {
    GreaterThanHalfPositive(list: Vec<i64>);
  }

  pub fn greater_than_half_positive<
    ListVal: HasType<Vec<i64>>,
    TotalCountVal: HasType<usize>,
    PositiveCountVal: HasType<usize>,
  >(
    _has_size: &ListHasSize<i64, TotalCountVal, ListVal>,
    _has_positives: &ListHasPositives<PositiveCountVal, ListVal>,
    _greater_than_half: &GreaterThanHalf<PositiveCountVal, TotalCountVal>,
  ) -> GreaterThanHalfPositive<ListVal>
  {
    GreaterThanHalfPositive::new()
  }
}

mod greater_than_half_positive_dynamic
{
  use mononym::*;

  use super::{
    greater_half::greater_than_half,
    greater_than_half_positive::{
      greater_than_half_positive,
      GreaterThanHalfPositive,
    },
    list_positive::count_positive_integers,
    list_size::list_size,
  };

  pub fn maybe_greater_than_half_positive<ListVal: HasType<Vec<i64>>>(
    seed: impl Seed,
    data: &Named<ListVal, Vec<i64>>,
  ) -> Option<GreaterThanHalfPositive<ListVal>>
  {
    let (seed1, seed2) = seed.replicate();
    let size = list_size(seed1, data);
    let positives = count_positive_integers(seed2, data);

    let greater_half = greater_than_half(&positives.count, &size.size);

    greater_half.map(|greater_half| {
      greater_than_half_positive(
        &size.list_has_size,
        &positives.list_has_positives,
        &greater_half,
      )
    })
  }
}

mod process_static
{
  use mononym::*;

  use super::greater_than_half_positive::GreaterThanHalfPositive;

  pub fn process_data<ListVal: HasType<Vec<i64>>>(
    _: &Named<ListVal, Vec<i64>>,
    _greater_than_half_positive: &GreaterThanHalfPositive<ListVal>,
  ) -> i64
  {
    0
  }
}

mod process_data_dynamic
{
  use mononym::*;

  use super::{
    greater_than_half_positive_dynamic::maybe_greater_than_half_positive,
    process_static::process_data as process_data_static,
  };

  #[derive(Debug, Eq, PartialEq)]
  pub enum Error
  {
    LessThanHalfPositive,
  }

  pub fn process_data(data: Vec<i64>) -> Result<i64, Error>
  {
    with_seed(move |life| {
      let (seed1, seed2) = life.into_seed().replicate();

      let data = seed1.new_named(data);

      let proof = maybe_greater_than_half_positive(seed2, &data)
        .ok_or(Error::LessThanHalfPositive)?;

      let res = process_data_static(&data, &proof);

      Ok(res)
    })
  }
}

fn main()
{
  use self::process_data_dynamic::{
    process_data,
    Error,
  };

  let data = vec![3, 2, 1, 0, -1];

  process_data(data).unwrap();

  let data = vec![3, 2, 1, 0, -1, -2];

  let res = process_data(data);

  assert_eq!(res, Err(Error::LessThanHalfPositive));
}
