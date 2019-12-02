// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn get() {
        let mut map = HashMap::new();
        map.insert(1, String::from("one"));
        map.insert(2, String::from("two"));
        map.insert(3, String::from("three"));
        map.insert(4, String::from("four"));
        map.insert(5, String::from("five"));
        struct Test {
            name: &'static str,
            key: usize,
            want: &'static str,
        };
        let tests = [
            Test {
                name: "value for 1",
                key: 1,
                want: "one",
            },
            Test {
                name: "value for 2",
                key: 2,
                want: "two",
            },
            Test {
                name: "value for 3",
                key: 3,
                want: "three",
            },
            Test {
                name: "value for 4",
                key: 4,
                want: "four",
            },
            Test {
                name: "value for 5",
                key: 5,
                want: "five",
            },
        ];
        for t in &tests {
            let got = map.get(&t.key);
            assert_eq!(t.want, got.unwrap(), "{}", t.name);
        }
    }
}
