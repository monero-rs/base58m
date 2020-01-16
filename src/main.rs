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

use std::{error, fmt};

use base58_monero::{
    base58, decode_stream, decode_stream_check, encode_stream, encode_stream_check,
};
use clap::clap_app;

/// Possible errors when reading inputs and encoding/decoding base58 and base58-check strings.
pub enum Error {
    /// Invalid base58 encoding/decoding
    InvalidEncode(base58::Error),
    /// Invalid IO operation
    InvalidIo(io::Error),
}

impl From<base58::Error> for Error {
    fn from(e: base58::Error) -> Error {
        Error::InvalidEncode(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::InvalidIo(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidEncode(e) => write!(f, "invalid encoding/decoding operation, {}", e),
            Error::InvalidIo(e) => write!(f, "invalid IO operation, {}", e),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::InvalidEncode(e) => Some(e),
            Error::InvalidIo(e) => Some(e),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Defines the mode of operation.
#[derive(Debug)]
enum Mode {
    /// Encode the stream
    Encode,
    /// Decode the stream
    Decode,
    /// Encode the stream with checksum
    CheckEncode,
    /// Decode the stream with checksum
    CheckDecode,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut out = io::stdout();

    let matches = clap_app!(base58m =>
        (version: "0.1.0")
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
