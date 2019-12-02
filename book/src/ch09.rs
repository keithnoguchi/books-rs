// SPDX-License-Identifier: GPL-2.0
//! Error Handling
use std::io;

/// Error represents the Error type of book crate.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    #[allow(dead_code)]
    Other,
}

impl From<io::ErrorKind> for Error {
    fn from(kind: io::ErrorKind) -> Self {
        Error::Io(io::Error::from(kind))
    }
}
