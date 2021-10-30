use core::marker::PhantomData;

pub trait Name: Send + Sync
{
}

pub trait HasType<T>: Name
{
}

pub struct Named<Name: HasType<Value>, Value>(Value, PhantomData<Name>);

pub struct Seed<N: Name>(PhantomData<N>);

pub struct Life<'name>(PhantomData<*mut &'name ()>);

struct SomeName<N>(PhantomData<N>);

impl<Name: HasType<Value>, Value> Named<Name, Value>
{
  pub fn value<'a>(&'a self) -> &'a Value
  {
    &self.0
  }

  pub fn into_value(self) -> Value
  {
    self.0
  }
}

impl<N: Name> Seed<N>
{
  pub fn new_named<V>(
    self,
    value: V,
  ) -> Named<impl HasType<V>, V>
  {
    unsafe_new_named(unsafe_new_name_with_type(|| {}), value)
  }

  pub fn replicate(self) -> (Seed<impl Name>, Seed<impl Name>)
  {
    (unsafe_new_seed(|| {}), unsafe_new_seed(|| {}))
  }
}

impl<F> Name for SomeName<F> where F: 'static + Send + Sync {}

impl<F, T> HasType<T> for SomeName<F> where F: 'static + Send + Sync {}

unsafe impl<'name> Send for Life<'name> {}

unsafe impl<'name> Sync for Life<'name> {}

impl<'name> Name for Life<'name> {}

impl<'name, T> HasType<T> for Life<'name> {}

pub fn with_seed<R>(cont: impl for<'name> FnOnce(Seed<Life<'name>>) -> R) -> R
{
  cont(Seed(PhantomData))
}

fn unsafe_new_name_with_type<F, T>(_: F) -> impl HasType<T>
where
  F: 'static + Send + Sync,
{
  SomeName(PhantomData::<F>)
}

fn unsafe_new_seed<F>(_: F) -> Seed<impl Name>
where
  F: 'static + Send + Sync,
{
  Seed(PhantomData::<SomeName<F>>)
}

fn unsafe_new_named<Name: HasType<Value>, Value>(
  _: Name,
  value: Value,
) -> Named<Name, Value>
{
  Named(value, PhantomData)
}

impl<N: Name> Seed<N>
{
  pub fn replicate_3(
    self
  ) -> (Seed<impl Name>, Seed<impl Name>, Seed<impl Name>)
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }

  pub fn replicate_4(
    self
  ) -> (
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
  )
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }

  pub fn replicate_5(
    self
  ) -> (
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
  )
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }

  pub fn replicate_6(
    self
  ) -> (
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
  )
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }

  pub fn replicate_7(
    self
  ) -> (
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
  )
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }

  pub fn replicate_8(
    self
  ) -> (
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
    Seed<impl Name>,
  )
  {
    (
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
      unsafe_new_seed(|| {}),
    )
  }
}
