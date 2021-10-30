# Mononym

Mononym is a library for creating unique type-level names for each value in Rust. The core type `Named<Name, T>` represents a named value of type `T` with a unique type `Name` as its name. Mononym guarantees that there can be no two values with the same name. With that, the `Name` type serves as a unique representation of a Rust value at the type level.

Mononym enables the use of the design pattern [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf) in Rust. It provides macros that simplify the definition of [dependent pairs](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs) and proof objects in Rust. Although there is still limited support for a full dependently-typed programming in Rust, Mononym helps us move a small step toward that direction by making it possible to refer to values in types.

## Core Concepts

Mononym harness several features unique to Rust and have a simpler implementation of named values compared to the existing implementations in languages like Haskell.

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

To test whether the types of two values are equal, we define the function `same<T>(_: T, _: T)` that passes the compilation check if two values have the same type. When we try to compile the code, we will find that both `same(foo(), foo())` and `same(bar(), bar())` pass the compilation, but `same(foo(), bar())` would result in a type error. Similarly, the check on `same(foo(), ())` fails, as Rust treats the returned `impl Name` different than the underlying type `()`.

### Generic-dependent Uniqueness of `impl Trait`

The use of `impl Name` in return position provides the first step toward defining unique names for each value. However the above code shows an obvious issue of using `impl Name` as unique type, as the test `same(foo(), foo())` passes the compilation. This means that two calls to the same function returning `impl Trait` will return values of the same opaque type. If we want the name type to be truly unique, the test `same(foo(), foo())` should have failed.

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

In the last test, we also see that the compilation test for `same(type_name(||{}), type_name(||{}))` also fails. This is because each new closure expression in Rust produces a [unique anonymous type](https://doc.rust-lang.org/reference/types/closure.html). With this, we know that as long as we are providing different closure expressions, the returned `impl Name` would be considered different type by Rust.

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

We define a struct `SomeName<N>` that implements `Name` based on the unique type `N`. We then rename our name generator function to `unsafe_new_name` and make it return `SomeName` based on the given generic type `N`. In Mononym, we [seal](https://rust-lang.github.io/api-guidelines/future-proofing.html) the `Name` trait and keep the `SomeName` struct and `unsafe_new_name` private, so that external users cannot create new `impl Name` that may violate the uniqueness guarantee.

We can see that at this stage the name creation is still unsafe, as user can still bind the closure expression to a variable like `let f = ||{}`, and then the resulting test `same(unsafe_new_name(f), unsafe_new_name(f))` would pass. Similarly, if we define a function like `foo() -> impl Name` that calls `unsafe_new_name(||{})` inside the function, the resulting test `same(foo(), foo())` would still pass the compilation.

### Name Seed

To guarantee that every opaque type behind an `impl Name` is unique, we need to ensure that not only the inner function that returns `impl Name` is generic, but also _all_ possible functions that return `impl Name` must be generic as well.

While it is possible to ask end users to provide a unique type using `||{}` at each function definition and and call, doing so can be error prone and confusing. Instead, we want to define a _name seed_ that is a unique type itself, and use it to generate names.

We implement the `Seed` as follows:

```rust,compile_fail
use core::marker::PhantomData;

trait Name {}
struct SomeName<N>(PhantomData<N>);
impl <N> Name for SomeName<N> {}

fn unsafe_new_name<N>(_: N) -> impl Name {
  SomeName::<N>(PhantomData)
}

struct Seed<N>(PhantomData<N>);

fn unsafe_new_seed<N>(_: N) -> Seed<impl Name>
{
  Seed(PhantomData::<SomeName<N>>)
}

impl <N> Seed<N> {
  fn new_name(self) -> impl Name
  {
    unsafe_new_name(|| {})
  }

  fn replicate(self) -> (Seed<impl Name>, Seed<impl Name>)
  {
    (unsafe_new_seed(|| {}), unsafe_new_seed(|| {}))
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

Although the seed is consumed during name generation, the second method `replicate(self)` consumes the original seed, and returns two new seeds with unique names in the form of `Seed<impl Name>`. By calling `replicate` one or more times, we will be able to generate multiple names with unique types.

The `unsafe_new_seed` function creates a new seed in the form of `Seed<impl Name>`, and similar to other functions earlier, a direct call to `unsafe_new_seed` may be unsafe and return names of the same type. However we can keep the `unsafe_new_seed` function private to the Mononym crate, so that users cannot create new seeds directly. In this way, external functions would have to always accept `Seed<impl Name>` as an argument somewhere along the function calls to be able to generate new names.

By treating our `test` function as an external function, it is forced to accept a `Seed<impl Name>` in order to generate new `impl Name`s. We first use `seed.replicate()` to create two new seeds `seed1` and `seed2`. When we compile the code, we can find out that the test `same(seed1, seed2)` fails, indicating that the two replicated seeds have different types. Similarly, the test `same(seed1.new_name(), seed2.new_name())` fails because the two names are generated from different seeds. It is also not possible to do something like `same(seed.new_name(), seed.new_name())`, because the affine type system of Rust consumes the seed during name generation and to not allow the seed to be reused.
