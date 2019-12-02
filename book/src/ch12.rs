// SPDX-License-Identifier: GPL-2.0
//! An I/O Project: Building a Command Line Program
use super::Error;
use std::io::ErrorKind;

/// Config contains the filename to parse.
#[derive(Debug, PartialEq)]
pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Error> {
        if args.len() < 2 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            let filename = args[1].clone();
            Ok(Self { filename })
        }
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    #[test]
    fn config_new() {
        struct Test {
            args: Vec<String>,
            want: Result<Config, Error>,
        }
        let tests = [
            Test {
                args: vec![String::from("with filename"), String::from("filename")],
                want: Ok(Config {
                    filename: String::from("filename"),
                }),
            },
            Test {
                args: vec![String::from("no filename")],
                want: Err(Error::Io(io::Error::from(io::ErrorKind::InvalidInput))),
            },
        ];
        for t in &tests {
            match Config::new(&t.args) {
                Ok(got) => {
                    if let Ok(want) = &t.want {
                        assert_eq!(want, &got);
                    } else {
                        panic!("unexpected success");
                    }
                }
                Err(got) => {
                    if let Err(want) = &t.want {
                        assert_eq!(want, &got);
                    } else {
                        panic!("unexpected error");
                    }
                }
            }
        }
    }
}
