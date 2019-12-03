// SPDX-License-Identifier: GPL-2.0
//! An I/O Project: Building a Command Line Program
use super::Error;
use std::{fs, io::ErrorKind};

/// Config encapsulates the filename to parse.
#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, Error> {
        if args.len() < 3 {
            Err(Error::from(ErrorKind::InvalidInput))
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Self { query, filename })
        }
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

/// run reads a file provided by Config.filename() and returns
/// the contents throught Result<String, Error>.
pub fn run(cfg: Config) -> Result<String, Error> {
    let contents = fs::read_to_string(cfg.filename())?;
    Ok(contents)
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
                args: vec![],
                want: Err(Error::Io(io::Error::from(io::ErrorKind::InvalidInput))),
            },
            Test {
                args: vec![String::from("no filename")],
                want: Err(Error::Io(io::Error::from(io::ErrorKind::InvalidInput))),
            },
            Test {
                args: vec![String::from("with query"), String::from("query")],
                want: Err(Error::Io(io::Error::from(io::ErrorKind::InvalidInput))),
            },
            Test {
                args: vec![
                    String::from("with query and filename"),
                    String::from("query"),
                    String::from("filename"),
                ],
                want: Ok(Config {
                    query: String::from("query"),
                    filename: String::from("filename"),
                }),
            },
            Test {
                args: vec![
                    String::from("with more than query and filename"),
                    String::from("query"),
                    String::from("filename"),
                    String::from("another argument"),
                ],
                want: Ok(Config {
                    query: String::from("query"),
                    filename: String::from("filename"),
                }),
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
    #[test]
    fn config_query() {
        struct Test {
            config: Config,
            want: &'static str,
        }
        let tests = [
            Test {
                config: Config {
                    query: String::from("some query"),
                    filename: String::from("some file"),
                },
                want: "some query",
            },
            Test {
                config: Config {
                    query: String::from(""),
                    filename: String::from("some filename"),
                },
                want: "",
            },
        ];
        for t in &tests {
            assert_eq!(t.want, t.config.query());
        }
    }
    #[test]
    fn config_filename() {
        struct Test {
            config: Config,
            want: &'static str,
        }
        let tests = [
            Test {
                config: Config {
                    query: String::from("some query"),
                    filename: String::from("some file"),
                },
                want: "some file",
            },
            Test {
                config: Config {
                    query: String::from("some query"),
                    filename: String::from(""),
                },
                want: "",
            },
        ];
        for t in &tests {
            assert_eq!(t.want, t.config.filename());
        }
    }
}
