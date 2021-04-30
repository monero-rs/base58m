[![Build Status](https://travis-ci.com/monero-rs/base58m-rs.svg?branch=master)](https://travis-ci.com/monero-rs/base58m-rs) [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/) [![Crates.io](https://img.shields.io/crates/v/base58m.svg)](https://crates.io/crates/base58m) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

`base58m`, a Rust Monero Base58 `base64`-like Binary
===

This binary crate is a simple implementation of a base64-like binary for Monero base58 encoding.

```
base58m 0.2.0
h4sh3d <h4sh3d@protonmail.com>
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

About
===

This project is maintained by community members. All contributions are welcome!
