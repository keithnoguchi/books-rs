// SPDX-License-Identifier: GPL-2.0
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
}
