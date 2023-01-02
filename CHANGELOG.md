# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) as described in [The Cargo Book](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field).

## [Unreleased]

### Changed

- Update dependencies `futures-utils`, `tokio`, and `thiserror`

## [1.0.0] - 2022-06-27

### Changed

- Bump clap from 2.33.3 to 3.2.6
- Bump base58-monero from 0.3.2 to 1.0.0
- Bump MSRV from 1.45.2 to 1.56.1
- Bump tokio from 1.13.0 to 1.19.2
- Bump shared workflows to v2
- Bump thiserror from 1.0.30 to 1.0.31
- Bump async-stream from 0.3.2 to 0.3.3
- Bump futures-util from 0.3.17 to 0.3.21

## [0.2.2] - 2021-11-15

### Changed

- Bump base58-monero from `0.3.1` to `0.3.2` ([#34](https://github.com/monero-rs/base58m/pull/34))
- Bump tokio from `1.12.0` to `1.13.0` ([#30](https://github.com/monero-rs/base58m/pull/30))

## [0.2.1] - 2021-10-13

### Added

- Add test jobs for basic encode/decode with and without check mode
- New automated release system based on monero-rs/workflows
- Documentation in README

### Changed

- Update `base58-monero` to `0.3.1`
- Update `futures-util` to `0.3.17`
- Update `async-stream` to `0.3.2`

## [0.2.0] - 2021-04-30

### Added

- Add `dependabot` and switch to GitHub Actions for the CI

### Changed

- Update `base58-monero` to `0.3.0`
- Update `tokio` to `1`
- Rename project from `base58m-rs` to `base58m`
- Improve `Error` implementation
- Improve documentation

### Fixed

- Clap now uses the `crate_version!()` macro to display the binary version

## [0.1.1] - 2020-01-16

### Changed

- Documentation and file tracking for crates.io releases

## [0.1.0] - 2020-01-09

### Added

- `base58m` binary with support for encoding/decoding Monero base58 data with checksum support

[Unreleased]: https://github.com/monero-rs/base58m/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/monero-rs/base58m/compare/v0.2.2...v1.0.0
[0.2.2]: https://github.com/monero-rs/base58m/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/monero-rs/base58m/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/monero-rs/base58m/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/monero-rs/base58m/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/monero-rs/base58m/compare/1909d92fd48441c88e758c00f18c5aad23b0ac39...v0.1.0
