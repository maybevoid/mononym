Macro to define new named value coupled with proofs
in the form of dependent pairs.

Suppose we have a named `u64` integer and we want to
add 1 to the value using
[checked addition](u64::checked_add)
and return a new named `u64`. Along the way,
we also want to construct a proof that shows that the new
`u64` value is a _successor_ of the original value.

We can define our successor proof similar to the other
proofs that are defined by [`proof!`], but when
we try to define our `add_one` function, we would
hit a roadblock like follows:

```rust,ignore
mod successor {
use mononym::*;

proof! {
    IsSuccessor(succ: u64, pred: u64);
}

pub fn add_one<NumVal: HasType<u64>>(
    seed: Seed<impl Name>,
    num: &Named<NumVal, u64>,
) -> Option<(Named<impl Name, u64>, IsSuccessor<??, NumVal>)>
{
    todo!()
}
}
```

The issue with the return type of our `add_one` function above is that
the type `IsSuccessor<??, NumVal>` _depends_ on the fresh name type
`impl Name` in `Named<impl Name, 64>` to fill in the hole of `??`.
This construct is known as a
[_dependent pair_](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs)
in languages with dependent types such as Idris.

Although Rust do not have built in support for constructing dependent
pairs, we can still emulate that by wrapping the two return types
inside a new struct that we will call `ExistSuccessor`:

```rust
mod successor {
use mononym::*;
use core::marker::PhantomData;

proof! {
    IsSuccessor(succ: u64, pred: u64);
}

pub struct ExistSuccessor<
    SuccVal: HasType<u64>,
    PredVal: HasType<u64>,
> {
    successor: Named<SuccVal, u64>,
    is_successor: IsSuccessor<SuccVal, PredVal>,
}

fn new_exist_successor<
    PredVal: HasType<u64>,
>
( seed: Seed<impl Name>,
    succ: u64,
) -> ExistSuccessor<impl HasType<u64>, PredVal>
{
    ExistSuccessor {
    successor: seed.new_named(succ),
    is_successor: IsSuccessor::new(),
    }
}

pub fn add_one<NumVal: HasType<u64>>(
    seed: Seed<impl Name>,
    num: &Named<NumVal, u64>,
) -> Option<ExistSuccessor<impl HasType<u64>, NumVal>>
{
    num.value()
    .checked_add(1)
    .map(|succ| new_exist_successor(seed, succ))
}
}
```

The `ExistSuccessor` struct allows the same type parameter `SuccVal`
to be used in both `Named` and `IsSuccessor`. Using that, we can
redefine our `add_one` function to return the type
`ExistSuccessor<impl HasType<u64>, NumVal>`, which the fresh
name type `impl HasType<u64>` can be used in both places.

Although the above solution works, there are quite a lot of
boilerplate required to define just one dependent pair type.
Therefore the `exists!` macro is provided so that the same
definition above can be simplified into a single line definition
as shown below:

```rust
mod successor {
use mononym::*;
use core::marker::PhantomData;

exists! {
    ExistSuccessor(succ: u64) => IsSuccessor(pred: u64);
}

pub fn add_one<NumVal: HasType<u64>>(
    seed: Seed<impl Name>,
    num: &Named<NumVal, u64>,
) -> Option<ExistSuccessor<impl HasType<u64>, NumVal>>
{
    num.value()
    .checked_add(1)
    .map(|succ| new_exist_successor(seed, succ))
}
}
```

Note that the new version of the `successor` module is the same as
the original version. The `exists!` takes care of generating
the various struct definitions, as well as calling [`proof!`]
to generate the `IsSuccessor` proof type.

One limitation of the `exists!` macro is that the existential name
is always in the first position of the following proof type.
That is, the definition

```
# use mononym::*;
exists! { ExistSuccessor(succ: u64) => IsSuccessor(pred: u64); }
```

leads to the call to

```
# use mononym::*;
proof! { IsSuccessor(succ: u64, pred: u64); }
```

Therefore we cannot switch the position of the variables in
the proof type, such as having

```
# use mononym::*;
proof! { IsSuccessor(pred: u64, succ: u64); }
```
