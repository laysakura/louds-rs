# louds-rs

High performance LOUDS (Level-Order Unary Degree Sequence) library.

[Master API Docs](https://laysakura.github.io/louds-rs/louds_rs/)
|
[Released API Docs](https://docs.rs/crate/louds-rs)
|
[Benchmark Results](https://laysakura.github.io/louds-rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/louds-rs/blob/master/CHANGELOG.md)

[![Build Status](https://travis-ci.com/laysakura/louds-rs.svg?branch=master)](https://travis-ci.com/laysakura/louds-rs)
[![Crates.io](https://img.shields.io/crates/v/louds-rs.svg)](https://crates.io/crates/louds-rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/louds-rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/louds-rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/louds-rs/blob/master/LICENSE-APACHE)

## Quickstart

To use louds-rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
louds-rs = "0.1"  # NOTE: Replace to latest minor version.
```

### Usage Overview
```rust
use louds_rs::{Louds, LoudsIndex, LoudsNodeNum};

// Construct from LBS.
let s = "10_1110_10_0_1110_0_0_10_110_0_0_0";
let louds = Louds::from(s);

// LoudsNodeNum <-> LoudsIndex
let node8 = LoudsNodeNum::new(8);
let index11 = louds.node_num_to_index(&node8);
assert_eq!(louds.index_to_node_num(&index11), node8);

// Search for children.
assert_eq!(louds.parent_to_children(&node8), vec!(LoudsIndex::new(17), LoudsIndex::new(18)));

// Search for parent.
assert_eq!(louds.child_to_parent(&index11), LoudsNodeNum::new(4));
```

## Features
(TBD)

### Complexity
(TBD)

## Versions
louds-rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

## Rust Version Supports

louds-rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Contributing

Any kind of pull requests are appreciated.

### Guidelines

- `README.md` is generated from `$ cargo readme` command. Do not manually update `README.md` but edit `src/lib.rs` and then `$ cargo readme > README.md`.
- Travis CI automatically does the following commit & push to your pull-requests:
    - `$ cargo readme > README.md`
    - `$ cargo fmt --all`

## License

MIT OR Apache-2.0
