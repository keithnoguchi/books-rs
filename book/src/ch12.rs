//! An I/O Project: Building a Command Line Program
use std::{
    env, error, fs,
    io::{self, BufRead},
};

/// Main application.
#[derive(Debug, PartialEq, Default)]
pub struct App {
    query: String,
    filename: String,
    is_case_insensitive: bool,
}

impl App {
    /// Create a new application with the passed command line arguments.
    /// The second argument, index 1, will be the query string and the
    /// third, index 2, as the filename.  It returns error if the
    /// `args`'s length is less than three.
    ///
    /// # Examples
    ///
    /// ```
    /// use the_book::ch12::App;
    ///
    /// let args: [String; 3] = [
    ///     "progname".into(),
    ///     "query string".into(),
    ///     "filename.txt".into(),
    /// ];
    ///
    /// let _app = App::new(&args).unwrap();
    /// ```
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].to_owned();
        let filename = args[2].to_owned();
        let is_case_insensitive = env::var("CASE_INSENSITIVE")
            .and_then(|var| {
                if var.is_empty() || var == "0" {
                    Err(env::VarError::NotPresent)
                } else {
                    Ok(var)
                }
            })
            .is_ok();
        Ok(Self {
            query,
            filename,
            is_case_insensitive,
        })
    }

    /// Read the each line from `config.filename` and returns `Vec<String>`
    /// which contains `config.query`.  It return error when it couldn't
    /// open a file to read or print out a error to the stderr when
    /// it encounters error while reading.
    ///
    /// # Example
    ///
    /// ```not_run
    /// use std::env;
    /// use the_book::ch12::App;
    ///
    /// let args = env::args().collect();
    /// let app = App::new(&args).unwrap();
    /// app.run();
    /// ```
    pub fn run(&self) -> Result<Vec<String>, Box<dyn error::Error>> {
        let f = io::BufReader::new(fs::File::open(&self.filename)?);
        let mut matches = vec![];
        for line in f.lines() {
            let line = line?;
            if self.search(&self.query_string(), &line) {
                matches.push(line);
            }
        }
        Ok(matches)
    }

    /// Return `true` if `line` contains `&query` string.
    ///
    /// `self.is_case_insensitive` bool variable controls
    /// the case sensitivity behavior.
    fn search(&self, query: &str, line: &str) -> bool {
        if self.is_case_insensitive {
            line.to_lowercase().contains(query)
        } else {
            line.contains(query)
        }
    }

    /// Returns the query string base on the `self.is_case_insensitive`
    /// flag.  It returns all lower cased `String` when
    /// `self.is_case_insensitive` is true.
    fn query_string(&self) -> String {
        if self.is_case_insensitive {
            self.query.to_lowercase()
        } else {
            self.query.to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_new_ok() {
        struct Test {
            name: &'static str,
            input: [String; 3],
            want: App,
        }
        let tests = [
            Test {
                name: "null query string",
                input: [
                    String::from("progname"),
                    String::from(""),
                    String::from("test.txt"),
                ],
                want: App {
                    query: "".into(),
                    filename: "test.txt".into(),
                    ..Default::default()
                },
            },
            Test {
                name: "some query string",
                input: [
                    String::from("progname"),
                    String::from("this is a test query"),
                    String::from("test.txt"),
                ],
                want: App {
                    query: "this is a test query".into(),
                    filename: "test.txt".into(),
                    ..Default::default()
                },
            },
        ];
        for t in &tests {
            let got = App::new(&t.input).unwrap();
            assert_eq!(t.want, got, "{}", t.name);
        }
    }
    #[test]
    fn app_new_err() {
        let args: [String; 2] = ["progname".into(), "some query".into()];
        let got = App::new(&args);
        assert_eq!(Err("not enough arguments"), got);
    }
    #[test]
    fn app_search_case_sensitive() {
        // set query as the second argument.
        let args: [String; 3] = ["".into(), "duct".into(), "".into()];
        let app = App::new(&args).unwrap();
        struct Test {
            content: &'static str,
            want: bool,
        }
        let tests = [
            Test {
                content: "Rust:",
                want: false,
            },
            Test {
                content: "safe, fast, productive.",
                want: true,
            },
            Test {
                content: "Pick three.",
                want: false,
            },
            Test {
                content: "Duct",
                want: false,
            },
        ];
        let query = app.query_string();
        for t in &tests {
            assert_eq!(
                t.want,
                app.search(&query, &t.content),
                "{:?} does not contain {:?}",
                t.content,
                query,
            );
        }
    }
    #[test]
    fn app_search_case_insensitive() {
        // set query as the second argument.
        let args: [String; 3] = ["".into(), "duct".into(), "".into()];
        let mut app = App::new(&args).unwrap();
        app.is_case_insensitive = true;
        struct Test {
            content: &'static str,
            want: bool,
        }
        let tests = [
            Test {
                content: "Rust:",
                want: false,
            },
            Test {
                content: "safe, fast, productive.",
                want: true,
            },
            Test {
                content: "Pick three.",
                want: false,
            },
            Test {
                content: "Duct",
                want: true,
            },
        ];
        let query = app.query_string();
        for t in &tests {
            assert_eq!(
                t.want,
                app.search(&query, &t.content),
                "{:?} does not contain {:?}",
                t.content,
                query,
            );
        }
    }
}
