# Mononym

Mononym is a library for creating unique type-level names for each value in Rust. The core type `Named<Name, T>` represents a named value of type `T` with a unique type `Name` as its name. Mononym guarantees that there can be no two values with the same name. With that, the `Name` type serves as a unique representation of a Rust value at the type level.

Mononym enables the use of the design pattern [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf) in Rust. It provides macros that simplify the definition of [dependent pairs](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs) and proof objects in Rust. Although there is still limited support for a full dependently-typed programming in Rust, Mononym helps us move a small step toward that direction by making it possible to refer to values in types.

## Implementation Details

Mononym harness several features unique to Rust and have a simpler implementation of named values compared to existing implementations in languages like Haskell.

### `impl Trait` as Opaque Types

At its core Mononym makes use of the idea that the use of `impl Trait` in return position produces a new [abstract type](https://doc.rust-lang.org/reference/types/impl-trait.html#abstract-return-types) that is unique to that function.

Consider the following example:

```rust,compile_fail
trait Name {}
impl Name for () {}

fn foo() -> impl Name {}
fn bar() -> impl Name {}

fn same<T>(_: T, _: T) {}
same(foo(), foo());
same(bar(), bar());
same(foo(), bar()); // error
same(foo(), ()); // error
same(bar(), ()); // error
```

We define a dummy trait `Name` that is implemented only for `()`. We then define two functions `foo()` and `bar()` that return `impl Name` by returning `()`. Although both `foo()` and `bar()` are in effect returning the same concrete type `()`, they are considered different types by the Rust compiler.

To test whether the types of two values are equal, we define the function `same<T>(_: T, _: T)` that pass the compilation check if two values have the same type. When we try to compile the code, we will find that both `same(foo(), foo())` and `same(bar(), bar())` pass the compilation, but `same(foo(), bar())` would result in a type error. Similarly, the check on `same(foo(), ())` fails, as Rust treats the returned `impl Name` different than the underlying type `()`.

### Generic-dependent Uniqueness of `impl Trait`

The use of `impl Name` in return position provides the first step toward defining unique names for each value. However the above code shows an obvious issue of using returned `impl Name` as unique type, as the test `same(foo(), foo())` pass the compilation. This means that two calls to the same function returning `impl Trait` will return values of the same opaque type. If we want the name type to be truly unique, the test `same(foo(), foo())` should have failed.

We can try to force the opaque type returned in a function returning `impl Name` unique by making the function _generic_:

```rust,compile_fail
trait Name {}
impl Name for () {}

fn type_name<T>(_: T) -> impl Name {}

fn same<T>(_: T, _: T) {}
same(type_name(()), type_name(()));
same(type_name(0_u64), type_name(0_u64));
same(type_name(()), type_name(0_u64)); // error
same(type_name(0_u32), type_name(0_u64)); // error
same(type_name(||{}), type_name(||{})) // error
```

In the above example, we define a `type_name` function that accepts a dummy value of any generic type, and return an `impl Name`. From there we can see that when `type_name` is called with the same type, the returned `impl Name` is considered the same type. As a result, the tests to `same(type_name(()), type_name(()))` and `same(type_name(0_u64), type_name(0_u64))` pass the compilation, but tests such as `same(type_name(()), type_name(0_u64))` fail the compilation.

### Closure Expressions as Unique Type

In the last test, we also see that the compilation test for `same(type_name(||{}), type_name(||{}))` has failed. This is because each new closure expression in Rust produces a [unique anonymous type](https://doc.rust-lang.org/reference/types/closure.html). With this, we know that as long as we are providing different closure expressions, the returned `impl Name` would be considered different type by Rust.

Moving one step further, we can instead define the implementation for `Name` using the anonymous closure type:

```rust,compile_fail
use core::marker::PhantomData;

trait Name {}
struct SomeName<N>(PhantomData<N>);
impl <N> Name for SomeName<N> {}

fn unsafe_new_name<N>(_: N) -> impl Name {
    SomeName::<N>(PhantomData)
}

fn same<T>(_: T, _: T) {}

let f = ||{};
same(unsafe_new_name(f), unsafe_new_name(f));

fn foo() -> impl Name {
    unsafe_new_name(||{})
}
same(foo(), foo())

same(unsafe_new_name(||{}), unsafe_new_name(||{})); // error
```

We define a struct `SomeName<N>` that implements `Name` based on the unique type `N`. We then rename our name generator function to `unsafe_new_name` and make it return `SomeName` based on the given generic type `N`. We keep the `SomeName` struct and `unsafe_new_name` private, so that external users cannot create new `impl Name` that may violate the uniqueness guarantee.

We can see that at this stage the name creation is still unsafe, as user can still bind the closure expression to a variable like `let f = ||{}`, and then the resulting test `same(unsafe_new_name(f), unsafe_new_name(f))` would pass. Similarly, if we define a function like `foo() -> impl Name` that calls `unsafe_new_name(||{})` inside the function, the resulting test `same(foo(), foo())` would still pass the compilation.

### Name Seed

To guarantee that every opaque type behind an `impl Name` is unique, we need to ensure that not only the inner function that returns `impl Name` is generic, but also _all_ possible functions that return `impl Name` must be generic as well.

While it is possible to ask end users to provide a unique type using `||{}` at each function definition and and call, doing so can be error prone and confusing. Instead, we want to define a _name seed_ that is a unique type itself, and use it to generate new names.

We implement the `Seed` type as follows:

```rust,compile_fail
use core::marker::PhantomData;

pub trait Name {}
struct SomeName<N>(PhantomData<N>);
impl <N> Name for SomeName<N> {}

pub struct Seed<N>(PhantomData<N>);

impl <N: Name> Seed<N> {
  pub fn new_name(self) -> impl Name
  {
    SomeName::<N>(PhantomData)
  }

  pub fn replicate(self) -> (Seed<impl Name>, Seed<impl Name>)
  {
    (Seed(PhantomData::<N>), Seed(PhantomData::<N>))
  }
}

fn test(seed: Seed<impl Name>) {
  fn same<T>(_: T, _: T) {}
  let (seed1, seed2) = seed.replicate();
  same(seed1, seed2); // error
  same(seed1.new_name(), seed2.new_name()); // error
}
```

The type `Seed<N>` is parameterized by a name `N` and provides two methods. The first method `new_name(self)` _consumes_ the seed and returns a new `impl Name` (notice the lack of `&` in `self`). Since the seed is consumed when generating the name, the same seed cannot be used to generate another new name of the same type.

Although the seed is consumed during name generation, the second method `replicate(self)` consumes the original seed, and returns two new seeds with unique names in the form of `Seed<impl Name>`. Notice that the two `Seed<impl Name>` returned by `replicate` are considered different types by Rust, even when they have the same underlying concrete type `Seed<N>`. By calling `replicate` one or more times, we will be able to generate multiple names with unique types.

Since there is no public constructor function to create new `Seed` value, the only way external users can create new seed is by replicating existing seeds. In this way, external functions would have to always accept `Seed<impl Name>` as an argument somewhere along the function calls to be able to generate new names.

By treating our `test` function as an external function, it is forced to accept a `Seed<impl Name>` in order to generate new `impl Name`s. We first use `seed.replicate()` to create two new seeds `seed1` and `seed2`. When we compile the code, we can find out that the test `same(seed1, seed2)` fails, indicating that the two replicated seeds have different types. Similarly, the test `same(seed1.new_name(), seed2.new_name())` fails because the two names are generated from different seeds. It is also not possible to do something like `same(seed.new_name(), seed.new_name())`, because the affine type system of Rust consumes the seed during name generation and to not allow the seed to be reused.

### Named Values

The `Seed` type we defined earlier provides a `new_name` method that returns unique `impl Name`. While having a unique name is not very useful on its own, it can be used to define a `Named<Name, T>` struct to assign unique names to a given value of type `T`. The `Named` struct is defined and used as follows:

```rust,compile_fail
use core::marker::PhantomData;

pub struct Named<Name, T>(T, PhantomData<Name>);

impl <Name, T> Named<Name, T> {
  pub fn value(&self) -> &T { &self.0 }
  pub fn into_value(self) -> T { self.0 }
}

pub trait Name {}
pub struct Seed<N>(PhantomData<N>);

impl <N: Name> Seed<N> {
  pub fn new_named<T>(self, value: T) -> Named<impl Name, T>
  {
    Named::<N, _>(value, PhantomData)
  }

   pub fn replicate(self) -> (Seed<impl Name>, Seed<impl Name>)
  {
    (Seed(PhantomData::<N>), Seed(PhantomData::<N>))
  }
}

fn test(seed: Seed<impl Name>) {
  fn same<T>(_: T, _: T) {}
  let (seed1, seed2) = seed.replicate();
  same(seed1.new_named(1), seed2.new_named(1)); // error
}
```

The struct `Named<Name, T>` is essentially a newtype wrapper around `T`, with the underlying value kept private. The `Named` type provides two public methods, `value` for getting a reference to the underlying value, and `into_value` to convert the named value to the underlying value.

The `Seed` type now provides a `new_named` method that accepts an owned value of type `T`, and returns a `Named<impl Name, T>`. Because the `impl Name` is nested inside `Named`, we can guarantee that the new name given to the value is unique, provided that the `Seed` type is unique.

Similar to earlier, we can test that two named values indeed have different names by writing a `test` function that accepts a `Seed<impl Name>`. After replicating the seed, we can verify that the test `same(seed1.new_named(1), seed2.new_named(1))` fails with error during compilation. This shows that the `Named<impl Name, i32>` vallues returned by the two calls to `new_name` are indeed unique.

We can think of the type `Named<Name, T>` as being a _singleton type_, that is, a type with only one possible value. With Rust's affine type system, the singleton guarantee is even stronger that we can never have two Rust values of type `Named<Name, T>` with the same `Name` type.

### Unique Lifetime with Higher Ranked Trait Bounds

Our setup for generating uniquely named values is mostly complete, provided we are able to hand over the first unique `Seed` value to the main function to start generating new names. But we cannot simply expose a function like `fn new_seed() -> Seed<impl Name>`, as we know that two calls to the same function will return two values of the same type, thereby making them non-unique.

We know that in languages like Haskell, it is possible to generate unique types by using continuation-passing-style with _higher-ranked_ continuations. While Rust do not currently support higher-ranked types, it instead supports [_higher-ranked trait bounds_](https://rustc-dev-guide.rust-lang.org/traits/hrtb.html) (HRTB) which can be used in similar way.

```rust,compile_fail
use core::marker::PhantomData;

pub trait Name {}
pub struct Life<'name>(PhantomData<*mut &'name ()>);
impl<'name> Name for Life<'name> {}

pub fn with_seed<R>(
  cont: impl for<'name> FnOnce(Seed<Life<'name>>) -> R
) -> R {
  cont(Seed(PhantomData))
}

pub struct Named<Name, T>(T, PhantomData<Name>);

impl <Name, T> Named<Name, T> {
  pub fn value(&self) -> &T { &self.0 }
  pub fn into_value(self) -> T { self.0 }
}

pub struct Seed<N>(PhantomData<N>);

impl <N: Name> Seed<N> {
  pub fn new_named<T>(self, value: T) -> Named<impl Name, T>
  {
    Named::<N, _>(value, PhantomData)
  }

   pub fn replicate(self) -> (Seed<impl Name>, Seed<impl Name>)
  {
    (Seed(PhantomData::<N>), Seed(PhantomData::<N>))
  }
}

fn same<T>(_: T, _: T) {}

with_seed(|seed| {
  let (seed1, seed2) = seed.replicate();
  same(seed1, seed2); // error
  same(seed1.new_named(1), seed2.new_named(1)); // error
});

with_seed(|seed1| {
  with_seed(|seed2| {
    same(seed1, seed2); // error
    same(seed1.new_named(1), seed2.new_named(1)); // error
  });
});
```

We first come out with a different way of generating unique types, using the `Life` type that is parameterized by a _unique lifetime_. The struct is defined as `struct Life<'name>(PhantomData<*mut &'name ()>)`. The inner type `PhantomData<*mut &'name ()>` makes Rust treats `Life<'name>` as if it is a raw pointer of type `*mut &'name ()`. We specifically make it so that Rust treats `'name` as an _invariant_ phantom lifetime. This means that if we have two types `Life<'name1>` and `Life<'name2>`, Rust would consider them as different types even if there are partial overlaps such as `'name1: 'name2`.

Using `Life`, we now simplify the problem of generating unique names to generating unique lifetimes. We then define the `with_seed` function, which accepts a continuation with a _higher-ranked trait bound_ `impl for<'name> FnOnce(Seed<Life<'name>>) -> R`. The `for<'name>` part forces the contnuation closure to work with _all_ possible lifetimes. As a result, we can guarantee that the type `Life<'name>` is always unique inside the closure. By using `Life<'name>` as the unique type inside `Seed<Life<'name>>`, we ensure that the seed type given to the continuation closure is also unique.

We can now repeat the same test we had earlier, but now with the tests running inside the closure given to `with_seed`. We can verify that after replicating the seed, the tests `same(seed1, seed2)` and `same(seed1.new_named(1), seed2.new_named(1))` still fail with compilation error, indicating that the names generated are different.

We can also repeat the same test with two nested calls to `with_seed`, thereby getting two separate fresh seeds `seed1` and `seed2`. Thanks to the magic of HRTB and invariant phantom lifetime, we can verify that even in this case the test `same(seed1, seed2)` still fails, indicating that Rust is treating the two underlying lifetimes differently. Similarly, the test `same(seed1.new_named(1), seed2.new_named(1))` also fails, indicating that names generated by two different seeds are indeed different.

The techniques of using phantom lifetimes as names and using HRTB to generate new lifetimes is first explored in [GhostCell](http://plv.mpi-sws.org/rustbelt/ghostcell/). With that, we close the loop of unique name generation by requiring the top level program to generate the first seed using `with_seed`. Furthermore, since multiple calls to `with_seed` is safe, this means that there is no way for external users to do unsafe construction of two seeds or named values of the same type. From this, we can safely reason that assuming unsafe Rust is not used, whenever we get a value of type `Named<Name, T>`, the type `Name` is always uniquely assigned to the underlying value.
