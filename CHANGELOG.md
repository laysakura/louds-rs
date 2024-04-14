# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.6.0] - 2024-04-15

### Added

- Added serde, rayon option, fixed code smell ([#12](https://github.com/laysakura/louds-rs/pull/12/))
- feature: Make cloneable ([#13](https://github.com/laysakura/louds-rs/pull/13))

## [v0.5.0] - 2024-03-11

### Added

- Index and node iterators [#10](https://github.com/laysakura/louds-rs/pull/10)

## [v0.4.0] - 2019-05-02
### Changed
- `Louds::node_num_to_index()` takes `LoudsNodeNum` instead of its reference.
- `Louds::index_to_node_num()` takes `LoudsIndex` instead of its reference.
- `Louds::child_to_parent()` takes `LoudsIndex` instead of its reference.
- `Louds::parent_to_children()` takes `LoudsNodeNum` instead of its reference.

## [v0.3.0] - 2019-05-02
### Changed
- `LoudsNodeNum` made into `Copy` tuple struct.
- `LoudsIndex` made into `Copy` tuple struct.

## [v0.2.0] - 2019-05-01
### Added
- `Louds::from::<&[bool]>()` constructor.

## [v0.1.1] - 2019-04-26
### Changed
- Removed unused `rayon` dependency.

## [v0.1.0] - 2019-04-26
Initial release.

[Unreleased]: https://github.com/laysakura/louds-rs/compare/v0.6.0...HEAD
[v0.6.0]: https://github.com/laysakura/louds-rs/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/laysakura/louds-rs/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/laysakura/louds-rs/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/laysakura/louds-rs/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/laysakura/louds-rs/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/laysakura/louds-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/laysakura/louds-rs/compare/89fad3a...v0.1.0
