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
louds-rs = "0.1"
```

### Usage Overview
(TBD)


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
- Beta version
- Nightly build

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
