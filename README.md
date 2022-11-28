# SortableJS-rs

This crate provides rusty bindings to SortableJS.

The documentation mostly lives with [the official SortableJS documentation](https://github.com/SortableJS/Sortable).

Just adding this crate as a dependency should be enough to get everything working when using [trunk](https://trunkrs.dev/). Just be careful to keep alive the return value of `Sortable::apply`, or you will get JavaScript exceptions.

You can find example usage of SortableJS from a pure-Rust WASM application in the `examples/` directory.
