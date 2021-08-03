# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

[Unreleased]: https://github.com/cezarmathe/qrwcell/compare/0.2.0...HEAD

## [0.2.0] - 2021-08-03

### Added

- Added this changelog.
- Added tests.
- Added CI workflow that builds, tests and checks the format and warnings
emitted by `cargo clippy` for Rust stable, beta, nightly and `1.36.0` (the
minumum supported Rust version - same as [parking_lot][parking_lot_msrv_0.2.0]).
- Added badge for the CI workflow.

### Changed

- Improve behaviour of `QrwCell::get` and `QrwCell::get_weak` for a cell that
has not been initialized yet - the cell will now panic instead of looping
forever. A panic should be easier to debug than an infinite loop.
- Changed `docs.rs` badge to the one provided by `docs.rs` itself.

### Fixed

- Fixed badges in the readme - they now link to the proper web pages instead of
linking to images.

## [0.1.0] - 2021-07-26

### Added

- quick read/write cell

[0.2.0]: https://github.com/cezarmathe/qrwcell/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/cezarmathe/qrwcell/releases/tag/0.1.0
[parking_lot_msrv_0.2.0]: https://github.com/Amanieu/parking_lot/tree/0.11.0#minimum-rust-version
