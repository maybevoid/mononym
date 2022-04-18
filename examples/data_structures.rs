#![allow(unused)]

mod equal
{
  use core::marker::PhantomData;

  pub struct Equal<Val1, Val2>(PhantomData<(Val1, Val2)>);

  use mononym::*;

  pub fn check_equal<T: Eq, Val1: HasType<T>, Val2: HasType<T>>(
    value1: &Named<Val1, T>,
    value2: &Named<Val2, T>,
  ) -> Option<Equal<Val1, Val2>>
  {
    if value1.value() == value2.value() {
      Some(Equal(PhantomData))
    } else {
      None
    }
  }
}

mod size
{
  use core::marker::PhantomData;

  use mononym::*;

  use super::sort::{
    Sorted,
    SortedFrom,
  };

  exists! {
    ExistListSize(size: usize) => ListHasSize<T>(list: Vec<T>);
  }

  proof! {
    NonEmpty<T>(list: Vec<T>);
  }

  // pub struct ListSize<ListVal, SizeVal>(PhantomData<(ListVal, SizeVal)>);

  // pub struct NonEmpty<ListVal>(PhantomData<ListVal>);

  // pub struct SizeResult<
  //   Elem,
  //   ListVal: HasType<Vec<Elem>>,
  //   SizeVal: HasType<usize>,
  // > {
  //   size: Named<SizeVal, usize>,
  //   size_proof: ListSize<ListVal, SizeVal>,
  //   non_empty_proof: Option<NonEmpty<ListVal>>,
  //   phantom: PhantomData<Elem>,
  // }

  pub fn list_size<Elem, ListVal: HasType<Vec<Elem>>>(
    seed: Seed<impl Name>,
    list: &Named<ListVal, Vec<Elem>>,
  ) -> ExistListSize<impl HasType<usize>, Elem, ListVal>
  {
    let size = list.value().len();
    new_exist_list_size(seed, size)
  }

  pub fn list_not_empty<
    Elem,
    ListVal: HasType<Vec<Elem>>,
    SizeVal: HasType<usize>,
  >(
    list_size: &Named<SizeVal, usize>,
    _list_has_size: &ListHasSize<Elem, SizeVal, ListVal>,
  ) -> Option<NonEmpty<Elem, ListVal>>
  {
    if list_size.value() == &0 {
      None
    } else {
      Some(NonEmpty::new())
    }
  }

  pub fn sorted_preserve_size<
    Elem,
    OldListVal: HasType<Vec<Elem>>,
    NewListVal: HasType<Vec<Elem>>,
    SizeVal: HasType<usize>,
  >(
    _size: ListHasSize<Elem, SizeVal, OldListVal>,
    _sorted: Sorted<NewListVal>,
    _sorted_from: SortedFrom<NewListVal, OldListVal>,
  ) -> ListHasSize<Elem, SizeVal, NewListVal>
  {
    ListHasSize::new()
  }

  pub fn sorted_preserve_non_empty<
    Elem,
    OldListVal: HasType<Vec<Elem>>,
    NewListVal: HasType<Vec<Elem>>,
  >(
    _non_empty: NonEmpty<Elem, OldListVal>,
    _sorted: Sorted<NewListVal>,
    _sorted_from: SortedFrom<NewListVal, OldListVal>,
  ) -> NonEmpty<Elem, NewListVal>
  {
    NonEmpty(PhantomData)
  }
}

mod sort
{
  use core::marker::PhantomData;

  use mononym::{
    HasType,
    Name,
    Named,
    Seed,
  };

  pub struct Sorted<ListVal>(PhantomData<ListVal>);
  pub struct SortedFrom<NewListVal, OldListVal>(
    PhantomData<(NewListVal, OldListVal)>,
  );

  pub struct SortedResult<
    Elem,
    OldListVal: HasType<Vec<Elem>>,
    NewListVal: HasType<Vec<Elem>>,
  > {
    new_list: Named<NewListVal, Vec<Elem>>,
    sorted: Sorted<NewListVal>,
    sorted_from: SortedFrom<NewListVal, OldListVal>,
  }

  pub fn sort<Elem: Ord, ListVal: HasType<Vec<Elem>>>(
    seed: Seed<impl Name>,
    list: Named<ListVal, Vec<Elem>>,
  ) -> SortedResult<Elem, ListVal, impl HasType<Vec<Elem>>>
where
  {
    let mut new_list = list.into_value();
    new_list.sort();
    let new_list = seed.new_named(new_list);

    SortedResult {
      new_list,
      sorted: Sorted(PhantomData),
      sorted_from: SortedFrom(PhantomData),
    }
  }

  pub unsafe fn sorted_axiom<ListVal>() -> Sorted<ListVal>
  {
    Sorted(PhantomData)
  }

  pub unsafe fn sorted_from_axiom<NewListVal, OldListVal>(
  ) -> SortedFrom<NewListVal, OldListVal>
  {
    SortedFrom(PhantomData)
  }
}

mod min
{
  use core::marker::PhantomData;

  use mononym::{
    HasType,
    Name,
    Named,
    Seed,
  };

  use super::{
    size::NonEmpty,
    sort::Sorted,
  };

  pub struct MinElem<ListVal, ElemVal>(PhantomData<(ListVal, ElemVal)>);

  pub struct MinResult<'a, Elem, ListVal, ElemVal: HasType<&'a Elem>>
  {
    elem: Named<ElemVal, &'a Elem>,
    min_proof: MinElem<ListVal, ElemVal>,
  }

  pub fn min<Elem, ListVal: HasType<Vec<Elem>>>(
    seed: Seed<impl Name>,
    list: &Named<ListVal, Vec<Elem>>,
    _sorted: Sorted<ListVal>,
    _non_empty: NonEmpty<Elem, ListVal>,
  ) -> MinResult<Elem, ListVal, impl HasType<&Elem>>
  {
    let elem = list.value().first().unwrap();

    MinResult {
      elem: seed.new_named(elem),
      min_proof: MinElem(PhantomData),
    }
  }
}

mod lookup
{
  use core::marker::PhantomData;
  use std::collections::BTreeMap;

  use mononym::{
    HasType,
    Name,
    Named,
    Seed,
  };

  pub struct HasKey<MapVal, KeyVal, ValueVal>(
    PhantomData<(MapVal, KeyVal, ValueVal)>,
  );

  pub struct LookupResult<
    'a,
    Value,
    MapVal,
    KeyVal,
    ValueVal: HasType<&'a Value>,
  > {
    enry_value: Named<ValueVal, &'a Value>,
    has_key_proof: HasKey<MapVal, KeyVal, ValueVal>,
  }

  pub fn lookup<
    'a,
    Key,
    Value,
    MapVal: HasType<BTreeMap<Key, Value>>,
    KeyVal: HasType<Key>,
  >(
    seed: Seed<impl Name>,
    map: &'a Named<MapVal, BTreeMap<Key, Value>>,
    key: &Named<KeyVal, Key>,
  ) -> Option<LookupResult<'a, Value, MapVal, KeyVal, impl HasType<&'a Value>>>
  where
    Key: Ord,
    Value: Clone,
  {
    map.value().get(key.value()).map(move |value| {
      let value = seed.new_named(value);

      LookupResult {
        enry_value: value,
        has_key_proof: HasKey(PhantomData),
      }
    })
  }
}

fn main() {}
