use crate::named::*;

crate::proof! {
    IsEqual<T>(first: T, second: T);
}

pub fn check_equal<T: Eq, FirstVal: HasType<T>, SecondVal: HasType<T>>(
  first: &Named<FirstVal, T>,
  second: &Named<SecondVal, T>,
) -> Option<IsEqual<T, FirstVal, SecondVal>>
{
  if first.value() == second.value() {
    Some(IsEqual::new())
  } else {
    None
  }
}

pub fn equal_commutative<T: Eq, FirstVal: HasType<T>, SecondVal: HasType<T>>(
  _is_equal: IsEqual<T, FirstVal, SecondVal>
) -> IsEqual<T, SecondVal, FirstVal>
{
  IsEqual::new()
}
