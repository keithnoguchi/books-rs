//! [Error] Handling
//!
//! [error]: https://doc.rust-lang.org/book/ch09-00-error-handling.html
use std::io;
use std::ops::Deref;

#[derive(Debug)]
pub struct Guess(i32);

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("value should be greater than 1 and less than 100, inclusive");
        }
        Self(value)
    }
}

impl Deref for Guess {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Error represents the Error type of book crate.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Other,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<io::ErrorKind> for Error {
    fn from(kind: io::ErrorKind) -> Self {
        Error::Io(io::Error::from(kind))
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match self {
            Error::Io(err) => {
                if let Error::Io(other) = other {
                    other.kind() == err.kind()
                } else {
                    false
                }
            }
            Error::Other => {
                if let Error::Other = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error_from_io_error() {
        const NAME: &str = "error_from_io_error";
        use std::io::{self, ErrorKind};
        struct Test {
            name: &'static str,
            kind: ErrorKind,
            want: Error,
        }
        let tests = [
            Test {
                name: "NotFound",
                kind: ErrorKind::NotFound,
                want: Error::Io(io::Error::from(ErrorKind::NotFound)),
            },
            Test {
                name: "PermissionDenied",
                kind: ErrorKind::PermissionDenied,
                want: Error::Io(io::Error::from(ErrorKind::PermissionDenied)),
            },
            Test {
                name: "ConnectionRefused",
                kind: ErrorKind::ConnectionRefused,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionRefused)),
            },
            Test {
                name: "ConnectionReset",
                kind: ErrorKind::ConnectionReset,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionReset)),
            },
            Test {
                name: "ConnectionAborted",
                kind: ErrorKind::ConnectionAborted,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionAborted)),
            },
            Test {
                name: "NotConnected",
                kind: ErrorKind::NotConnected,
                want: Error::Io(io::Error::from(ErrorKind::NotConnected)),
            },
            Test {
                name: "AddrInUse",
                kind: ErrorKind::AddrInUse,
                want: Error::Io(io::Error::from(ErrorKind::AddrInUse)),
            },
            Test {
                name: "AddrNotAvailable",
                kind: ErrorKind::AddrNotAvailable,
                want: Error::Io(io::Error::from(ErrorKind::AddrNotAvailable)),
            },
            Test {
                name: "BrokenPipe",
                kind: ErrorKind::BrokenPipe,
                want: Error::Io(io::Error::from(ErrorKind::BrokenPipe)),
            },
            Test {
                name: "WouldBlock",
                kind: ErrorKind::WouldBlock,
                want: Error::Io(io::Error::from(ErrorKind::WouldBlock)),
            },
            Test {
                name: "AlreadyExists",
                kind: ErrorKind::AlreadyExists,
                want: Error::Io(io::Error::from(ErrorKind::AlreadyExists)),
            },
            Test {
                name: "InvalidInput",
                kind: ErrorKind::InvalidInput,
                want: Error::Io(io::Error::from(ErrorKind::InvalidInput)),
            },
            Test {
                name: "InvalidData",
                kind: ErrorKind::InvalidData,
                want: Error::Io(io::Error::from(ErrorKind::InvalidData)),
            },
            Test {
                name: "TimedOut",
                kind: ErrorKind::TimedOut,
                want: Error::Io(io::Error::from(ErrorKind::TimedOut)),
            },
            Test {
                name: "WriteZero",
                kind: ErrorKind::WriteZero,
                want: Error::Io(io::Error::from(ErrorKind::WriteZero)),
            },
            Test {
                name: "Interrupted",
                kind: ErrorKind::Interrupted,
                want: Error::Io(io::Error::from(ErrorKind::Interrupted)),
            },
            Test {
                name: "Other",
                kind: ErrorKind::Other,
                want: Error::Io(io::Error::from(ErrorKind::Other)),
            },
            Test {
                name: "UnexpectedEof",
                kind: ErrorKind::UnexpectedEof,
                want: Error::Io(io::Error::from(ErrorKind::UnexpectedEof)),
            },
        ];
        for t in &tests {
            let got = Error::from(io::Error::from(t.kind));
            assert_eq!(t.want, got, "{}: {}", NAME, t.name);
        }
    }
    #[test]
    fn error_from_io_error_kind() {
        const NAME: &str = "error_from_io_error_kind";
        use std::io::{self, ErrorKind};
        struct Test {
            name: &'static str,
            kind: ErrorKind,
            want: Error,
        }
        let tests = [
            Test {
                name: "NotFound",
                kind: ErrorKind::NotFound,
                want: Error::Io(io::Error::from(ErrorKind::NotFound)),
            },
            Test {
                name: "PermissionDenied",
                kind: ErrorKind::PermissionDenied,
                want: Error::Io(io::Error::from(ErrorKind::PermissionDenied)),
            },
            Test {
                name: "ConnectionRefused",
                kind: ErrorKind::ConnectionRefused,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionRefused)),
            },
            Test {
                name: "ConnectionReset",
                kind: ErrorKind::ConnectionReset,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionReset)),
            },
            Test {
                name: "ConnectionAborted",
                kind: ErrorKind::ConnectionAborted,
                want: Error::Io(io::Error::from(ErrorKind::ConnectionAborted)),
            },
            Test {
                name: "NotConnected",
                kind: ErrorKind::NotConnected,
                want: Error::Io(io::Error::from(ErrorKind::NotConnected)),
            },
            Test {
                name: "AddrInUse",
                kind: ErrorKind::AddrInUse,
                want: Error::Io(io::Error::from(ErrorKind::AddrInUse)),
            },
            Test {
                name: "AddrNotAvailable",
                kind: ErrorKind::AddrNotAvailable,
                want: Error::Io(io::Error::from(ErrorKind::AddrNotAvailable)),
            },
            Test {
                name: "BrokenPipe",
                kind: ErrorKind::BrokenPipe,
                want: Error::Io(io::Error::from(ErrorKind::BrokenPipe)),
            },
            Test {
                name: "WouldBlock",
                kind: ErrorKind::WouldBlock,
                want: Error::Io(io::Error::from(ErrorKind::WouldBlock)),
            },
            Test {
                name: "AlreadyExists",
                kind: ErrorKind::AlreadyExists,
                want: Error::Io(io::Error::from(ErrorKind::AlreadyExists)),
            },
            Test {
                name: "InvalidInput",
                kind: ErrorKind::InvalidInput,
                want: Error::Io(io::Error::from(ErrorKind::InvalidInput)),
            },
            Test {
                name: "InvalidData",
                kind: ErrorKind::InvalidData,
                want: Error::Io(io::Error::from(ErrorKind::InvalidData)),
            },
            Test {
                name: "TimedOut",
                kind: ErrorKind::TimedOut,
                want: Error::Io(io::Error::from(ErrorKind::TimedOut)),
            },
            Test {
                name: "WriteZero",
                kind: ErrorKind::WriteZero,
                want: Error::Io(io::Error::from(ErrorKind::WriteZero)),
            },
            Test {
                name: "Interrupted",
                kind: ErrorKind::Interrupted,
                want: Error::Io(io::Error::from(ErrorKind::Interrupted)),
            },
            Test {
                name: "Other",
                kind: ErrorKind::Other,
                want: Error::Io(io::Error::from(ErrorKind::Other)),
            },
            Test {
                name: "UnexpectedEof",
                kind: ErrorKind::UnexpectedEof,
                want: Error::Io(io::Error::from(ErrorKind::UnexpectedEof)),
            },
        ];
        for t in &tests {
            let got = Error::from(t.kind);
            assert_eq!(t.want, got, "{}: {}", NAME, t.name);
        }
    }
}
