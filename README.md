# Mononym

[![Crates.io][crates-badge]][crates-url]
[![Documentation][doc-badge]][doc-url]
[![Apache licensed][license-badge]][license-url]

[crates-badge]: https://img.shields.io/crates/v/mononym.svg
[crates-url]: https://crates.io/crates/mononym
[doc-badge]: https://docs.rs/mononym/badge.svg
[doc-url]: https://docs.rs/mononym
[license-badge]: https://img.shields.io/crates/l/mononym.svg
[license-url]: https://github.com/maybevoid/mononym/blob/master/LICENSE
[actions-badge]: https://github.com/maybevoid/mononym/workflows/Cargo%20Tests/badge.svg

Mononym is a library for creating unique type-level names for each value in Rust. The core type `Named<Name, T>` represents a named value of type `T` with a unique type `Name` as its name. Mononym guarantees that there can be no two values with the same name. With that, the `Name` type serves as a unique representation of a Rust value at the type level.

Mononym enables the use of the design pattern [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf) in Rust. It provides macros that simplify the definition of [dependent pairs](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs) and proof objects in Rust. Although there is still limited support for a full dependently-typed programming in Rust, Mononym helps us move a small step toward that direction by making it possible to refer to values in types.

## Blog Posts

- [Mononym: Type-Level Named Values in Rust - Part 1: Demo and Implementation](https://maybevoid.com/blog/mononym-part-1/)

## [Implementation Details](./docs/implementation.md)

## Examples

Here are a few examples sneak peek that are currently work in progress. Apologize for the lack of documentation for the examples. There will be in-depth tutorials that go through the example code and guide the readers on how to use `mononym` to define proofs.

- [Arithmetic](./examples/number.rs)
- [Type-level Access Control](./examples/access_control.rs)
- [Proofs on Data Structures](./examples/data_structures.rs)
- [Extracting Information From List](./examples/list.rs)
