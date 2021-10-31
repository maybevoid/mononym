# Mononym

Mononym is a library for creating unique type-level names for each value in Rust. The core type `Named<Name, T>` represents a named value of type `T` with a unique type `Name` as its name. Mononym guarantees that there can be no two values with the same name. With that, the `Name` type serves as a unique representation of a Rust value at the type level.

Mononym enables the use of the design pattern [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf) in Rust. It provides macros that simplify the definition of [dependent pairs](https://docs.idris-lang.org/en/latest/tutorial/typesfuns.html#dependent-pairs) and proof objects in Rust. Although there is still limited support for a full dependently-typed programming in Rust, Mononym helps us move a small step toward that direction by making it possible to refer to values in types.

## [Implementation Details](./docs/implementation.md)

## Examples

Here are a few examples sneak peek that are currently work in progress. Apologize for the lack of documentation for the examples. There will be in-depth tutorials that go through the example code and guide the readers on how to use `mononym` to define proofs.

- [Type-level Access Control](./examples/access_control.rs)
- [Proofs on Data Structures](./examples/data_structures.rs)
- [Extracting Information From List](./examples/list.rs)
