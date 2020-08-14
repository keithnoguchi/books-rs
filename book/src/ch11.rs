//! Writing Automated [Tests]
//!
//! [tests]: https://doc.rust-lang.org/book/ch11-00-testing.html

/// Simple `add_two` function to demonstrate the integration test
/// as well as unit test with Result<T, E> return value.
///
/// # Example
///
/// ```
/// use the_book::ch11::add_two;
///
/// assert_eq!(5, add_two(3));
/// ```
///
pub fn add_two(x: i32) -> i32 {
    x + 2
}

/// Another function `greeting` to demonstrate `assert` macro additional
/// arguments.
///
/// # Example
///
/// ```
/// use the_book::ch11::greeting;
///
/// let name = "Keith";
/// assert!(greeting(name).contains(name));
/// ```
pub fn greeting(name: &str) -> String {
    format!("Hi {}", name)
}

/// `Rectangle` to demonstrate the rust test framework.
///
/// # Examples
///
/// ```
/// use the_book::ch11::Rectangle;
///
/// let rect = Rectangle::new(10, 20);
/// let other = Rectangle::new(2, 19);
/// assert!(rect.can_hold(&other));
/// ```
pub struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    pub fn new(length: i32, width: i32) -> Self {
        Self { length, width }
    }
    pub fn can_hold(&self, other: &Self) -> bool {
        self.length > other.length && self.width > other.width
    }
}

/// `Guess` structure to demonstrate the `#[should_panic]` test attribute.
///
/// # Examples
///
/// use the_book::ch11::Guess;
///
/// #[test]
/// #[should_panic]
/// fn negative_value() {
///     Guess::new(-1);
/// }
///
/// #[test]
/// #[should_panic]
/// fn more_than_100() {
///     Guess::new(101);
/// }
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Create a new `Guess` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use the_book::ch11::Guess;
    ///
    /// let guess = Guess::new(55);
    /// assert_eq!(55, guess.value());
    /// ```
    ///
    /// # Panics
    ///
    /// It will panic when the provided value is negative
    /// or more than 100.
    pub fn new(value: i32) -> Self {
        if value < 0 {
            panic!("{:#?} is negative", value);
        } else if value > 100 {
            panic!("{:#?} is more than 100", value);
        }
        Self { value }
    }

    /// Return the current guessed value.
    ///
    /// # Examples
    ///
    /// ```
    /// use the_book::ch11::Guess;
    ///
    /// let guess = Guess::new(29);
    /// assert_eq!(29, guess.value());
    /// ```
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_two_ok() -> Result<(), String> {
        struct Test {
            name: &'static str,
            data: i32,
            want: i32,
        }
        let tests = [
            Test {
                name: "add two to four",
                data: 4,
                want: 6,
            },
            Test {
                name: "add two to zero",
                data: 0,
                want: 2,
            },
            Test {
                name: "add two to 1,000,000,000",
                data: 1_000_000_000,
                want: 1_000_000_002,
            },
        ];
        for t in &tests {
            match super::add_two(t.data) {
                got if got == t.want => continue,
                got => {
                    return Err(format!("{}: unexpected value: {:?} != {:?}",
                                       t.name, t.want, got));
                },
            }
        }
        Ok(())
    }
    #[test]
    fn greeting() {
        let name = "Keith";
        let got = super::greeting(name);
        assert!(got.contains(name), "{:#?} does not contains {:#?}", got, name);
    }
    #[test]
    fn rectangle_can_hold() {
        use super::Rectangle;

        struct Test {
            name: &'static str,
            data: Rectangle,
            other: Rectangle,
            want: bool,
        }
        let tests = [
            Test {
                name: "can hold",
                data: Rectangle::new(2, 3),
                other: Rectangle::new(1, 2),
                want: true,
            },
            Test {
                name: "cannot due to the same length",
                data: Rectangle::new(2, 3),
                other: Rectangle::new(2, 1),
                want: false,
            },
            Test {
                name: "cannot hold due to the same width",
                data: Rectangle::new(2, 3),
                other: Rectangle::new(1, 3),
                want: false,
            },
        ];
        for t in &tests {
            assert_eq!(t.want, t.data.can_hold(&t.other), "{}", t.name);
        }
    }
    #[test]
    #[should_panic(expected = "is negative")]
    fn guess_negative_value() {
        super::Guess::new(-1);
    }
    #[test]
    #[should_panic(expected = "is more than 100")]
    fn guess_more_than_100() {
        super::Guess::new(101);
    }
    #[test]
    fn guess_ok() {
        struct Test {
            name: &'static str,
            value: i32,
        }
        let tests = [
            Test {
                name: "zero value",
                value: 0,
            },
            Test {
                name: "one value",
                value: 1,
            },
            Test {
                name: "99 value",
                value: 99,
            },
            Test {
                name: "100 value",
                value: 100,
            },
        ];
        for t in &tests {
            let got = super::Guess::new(t.value);
            assert_eq!(t.value, got.value(), "{}", t.name);
        }
    }
}
