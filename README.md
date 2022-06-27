[![Build Status](https://img.shields.io/github/workflow/status/monero-rs/base58m/CI/main)](https://github.com/monero-rs/base58m/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/base58m.svg)](https://crates.io/crates/base58m)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.56.1-blue)](https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html)

`base58m`, a Rust Monero Base58 `base64`-like Binary
===

This binary crate is a simple implementation of a base64-like binary for Monero base58 encoding.

```
base58m
Monero Rust Contributors
Base58 (Monero format) encode or decode FILE, or standard input, to standard output.

With no FILE, or when FILE is -, read standard input.

USAGE:
    base58m [FLAGS] [FILE]

FLAGS:
    -c, --check      Use base58 check mode
    -d, --decode     Decode data
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>    Sets the input file to use
```

## Install

If you have the rust toolchain setup on your machine simply run (use `--force` to update the binary)
```
cargo install --force base58m
```

## Releases and Changelog

See [CHANGELOG.md](CHANGELOG.md) and [RELEASING.md](RELEASING.md).

## About

This project is maintained by community members. All contributions are welcome!
