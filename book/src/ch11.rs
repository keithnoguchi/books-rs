// SPDX-License-Identifier: GPL-2.0
//! Writing Automated Tests

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = book::add_two(arg);
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn new(length: u32, width: u32) -> Self {
        Self { length, width }
    }
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.length <= self.length && other.width <= self.width
    }
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    #[allow(dead_code)]
    fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("the value should be in 1..101");
        }
        Self { value }
    }
    #[allow(dead_code)]
    fn value(&self) -> i32 {
        self.value
    }
}

struct Guess2(i32);

impl Guess2 {
    #[allow(dead_code)]
    fn new(value: i32) -> Result<Self, String> {
        if value < 1 || value > 100 {
            Err(String::from("invalid value"))
        } else {
            Ok(Self(value))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rectangle_can_hold() {
        use super::Rectangle;
        const NAME: &str = "rectangle_can_hold";
        struct Test {
            name: &'static str,
            r1: Rectangle,
            r2: Rectangle,
            want: bool,
        }
        let tests = [
            Test {
                name: "r1 can hold r2",
                r1: Rectangle::new(2, 2),
                r2: Rectangle::new(2, 1),
                want: true,
            },
            Test {
                name: "r1 cannot hold r2",
                r1: Rectangle::new(2, 1),
                r2: Rectangle::new(2, 2),
                want: false,
            },
            Test {
                name: "r1 and r2 are equal, hence r1 can hold r2",
                r1: Rectangle::new(2, 2),
                r2: Rectangle::new(2, 2),
                want: true,
            },
        ];
        for t in &tests {
            assert_eq!(t.want, t.r1.can_hold(&t.r2), "{}: {}", NAME, t.name);
        }
    }
    #[test]
    fn greeting_contains() {
        const NAME: &str = "greeting_contains";
        let tests = [
            String::from("Alice"),
            String::from("Bob"),
            String::from("Keith"),
        ];
        for t in &tests {
            assert!(super::greeting(t).contains(t), "{}: {}", NAME, t);
        }
    }
    #[test]
    #[should_panic(expected = "the value should be in 1..101")]
    fn panic_guess_with_invalid_value() {
        let tests = [32i32, 99, -1];
        for t in &tests {
            super::Guess::new(*t);
        }
    }
    #[test]
    fn guess2_return_ok() -> Result<(), String> {
        let tests = [32i32, 99, 100, 1];
        for t in &tests {
            super::Guess2::new(*t)?;
        }
        Ok(())
    }
    #[test]
    fn guess2_return_err() {
        let tests = [-1i32, 101, 200, 0];
        for t in &tests {
            if let Ok(_) = super::Guess2::new(*t) {
                let msg = format!("{}: unexpected success", t);
                panic!(msg);
            }
        }
    }
}
