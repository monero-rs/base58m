// Rust Monero Base58 binary encoder/decoder
// Written in 2020 by
//   h4sh3d <h4sh3d@protonmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//

// Coding conventions
#![forbid(unsafe_code)]

use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncRead;
use tokio::io::AsyncWriteExt;

use futures_util::pin_mut;
use futures_util::stream::StreamExt;

use base58_monero::{
    base58, decode_stream, decode_stream_check, encode_stream, encode_stream_check,
};
use clap::{clap_app, crate_version};
use thiserror::Error;

/// Possible errors when reading inputs and encoding/decoding base58 and base58-check strings.
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid base58 encoding/decoding.
    #[error("Invalid base58 encoding/decoding: {0}")]
    InvalidEncode(#[from] base58::Error),
    /// Invalid IO operation.
    #[error("Invalid IO operation: {0}")]
    InvalidIo(#[from] io::Error),
}

/// Helper result type used for all operations.
type Result<T> = std::result::Result<T, Error>;

/// Defines the mode of operation to apply on the data stream.
#[derive(Debug, Clone, Copy)]
enum Mode {
    /// Encode the stream in base58 Monero format.
    Encode,
    /// Decode the stream from a base58 Monero format.
    Decode,
    /// Encode the stream in base58 Monero format with an appended checksum.
    CheckEncode,
    /// Decode the stream from a base58 Monero format with an appended checksum.
    CheckDecode,
}

/// Base58 encode or decode the `FILE` passed in parameter, or if no `FILE` or if `= '-'` the
/// standard input is used.
///
/// Two flags are defined:
///
///  * `-c` or `--check` for using the checksum mode in base58
///  * `-d` or `--decode` for decoding a streaming, default is encoding without checksum
///
#[tokio::main]
async fn main() -> Result<()> {
    let mut out = io::stdout();

    let matches = clap_app!(base58m =>
        (version: crate_version!())
        (author: "h4sh3d <h4sh3d@protonmail.com>")
        (about: "Base58 (Monero format) encode or decode FILE, or standard input, to standard output.\n\nWith no FILE, or when FILE is -, read standard input.")
        (@arg FILE: "Sets the input file to use")
        (@arg decode: -d --decode "Decode data")
        (@arg check: -c --check "Use base58 check mode")
    ).get_matches();

    let mode = if matches.is_present("decode") {
        if matches.is_present("check") {
            Mode::CheckDecode
        } else {
            Mode::Decode
        }
    } else if matches.is_present("check") {
        Mode::CheckEncode
    } else {
        Mode::Encode
    };

    let input: Box<dyn AsyncRead + Unpin> = match matches.value_of("FILE") {
        Some(file) => match file {
            "-" => Box::new(io::stdin()),
            _ => Box::new(File::open(file).await?),
        },
        None => Box::new(io::stdin()),
    };

    match mode {
        Mode::Encode => {
            let s = encode_stream(input);
            pin_mut!(s);

            while let Some(value) = s.next().await {
                print!("{}", value?);
            }
        }
        Mode::CheckEncode => {
            let s = encode_stream_check(input);
            pin_mut!(s);

            while let Some(value) = s.next().await {
                print!("{}", value?);
            }
        }
        Mode::Decode => {
            let s = decode_stream(input);
            pin_mut!(s);

            while let Some(value) = s.next().await {
                out.write_u8(value?).await?;
            }
        }
        Mode::CheckDecode => {
            let s = decode_stream_check(input);
            pin_mut!(s);

            while let Some(value) = s.next().await {
                out.write_u8(value?).await?;
            }
        }
    }

    Ok(())
}
