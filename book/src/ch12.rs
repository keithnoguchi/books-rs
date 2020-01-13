// SPDX-License-Identifier: GPL-2.0
//! An I/O Project: Building a Command Line Program
use super::ch09::Error;
use std::{fs, io::ErrorKind};

/// Config to capture command line arguments for the I/O project.
///
/// # Examples
/// ```
/// use the_book as book;
/// use book::ch09::Error;
/// use book::ch12::Config;
///
/// fn main() -> Result<(), Error> {
///     let args = vec![
///         String::from("crate"),
///         String::from("some query"),
///         String::from("some filename"),
///     ];
///     let config = Config::new(&args)?;
///     assert_eq!("some query", config.query());
///     assert_eq!("some filename", config.filename());
///     Ok(())
/// }
/// ```
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
pub fn run(cfg: Config) -> Result<(), Error> {
    let contents = fs::read_to_string(cfg.filename())?;
    for line in search(cfg.query(), &contents) {
        println!("{}", line);
    }
    Ok(())
}

/// search takes `query` as a first parameter and returns the line
/// in case it's in `line`.
///
/// # Examples
/// ```
/// use the_book::ch12::search;
///
/// let data = "\
/// something here,
/// and some there.";
///
/// let query = "some";
/// let want = vec!["something here,", "and some there."];
/// assert_eq!(want, search(query, data));
///
/// let query = "another";
/// let want: Vec<&str> = vec![];
/// assert_eq!(want, search(query, data));
///
/// let query = "Some";
/// let want: Vec<&str> = vec![];
/// assert_eq!(want, search(query, data));
/// ```
pub fn search<'a>(query: &str, data: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    for line in data.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
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
    #[test]
    fn search_string() {
        struct Test {
            query: &'static str,
            data: &'static str,
            want: Vec<&'static str>,
        }
        let tests = [
            Test {
                query: "",
                data: "",
                want: vec![],
            },
            Test {
                query: "line",
                data: "
This is a line.
Another line.
and another line.
",
                want: vec!["This is a line.", "Another line.", "and another line."],
            },
        ];
        for t in &tests {
            let got = search(t.query, t.data);
            assert_eq!(t.want, got);
        }
    }
}
