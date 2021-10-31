
Macro to help define a new proof type.

Suppose we want to write a module that tests whether a
named `i64` is greater or equal to 0, and return a proof
of it. We could define the proof type manually as follows:

```rust
mod natural {
use mononym::*;
use core::marker::PhantomData;

pub struct IsNatural<NumVal: HasType<i64>>(PhantomData<NumVal>);

impl <NumVal: HasType<i64>> IsNatural<NumVal> {
    fn new() -> Self {
    Self(PhantomData)
    }
}

pub fn is_natural<NumVal: HasType<i64>>(
    num: &Named<NumVal, i64>,
) -> Option<IsNatural<NumVal>>
{
    if *num.value() >= 0 {
    Some(IsNatural::new())
    } else {
    None
    }
}
}
```

We define `IsNatural` as a proof type for a named `i64` value
with the name `NumVal` that shows that the number is greater
or equal to 0. The type has a private
[`PhantomData`](core::marker::PhantomData) body, as the
existence of the proof value alone is sufficient. It provides
a private `new()` method that allows functions to construct
the proof object.

The function `is_natural` then accepts a named number value
with the name `NumVal`, and only constructs the
proof `IsNatural<NumVal>` using `IsNatural::new()`
if the condition is satisfied.

As we define more proof types, the need to manually define
the proof structs and methods can become tedious. The
`proof!` macro simplifies the definition such as
above so that we can write our code as follows:

```rust
mod natural {
use mononym::*;

proof! {
    IsNatural(num: i64);
}

pub fn is_natural<NumVal: HasType<i64>>(
    num: &Named<NumVal, i64>,
) -> Option<IsNatural<NumVal>>
{
    if *num.value() >= 0 {
    Some(IsNatural::new())
    } else {
    None
    }
}
}
```

Note that the new version of the `natural` module is the same as
the original version. `proof!` takes care of generating
the struct definition and the private `new` method, so that
we do not need to keep repeating the same boilerplate definition.
