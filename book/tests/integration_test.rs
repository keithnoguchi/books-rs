// SPDX-License-Identifier: GPL-2.0
use the_book;

mod common;

#[test]
fn add_two() {
    struct Test {
        name: &'static str,
        data: i32,
        want: i32,
    }
    let tests = [
        Test {
            name: "positive value",
            data: 2,
            want: 4,
        },
        Test {
            name: "zero value",
            data: 0,
            want: 2,
        },
        Test {
            name: "negative value",
            data: -2,
            want: 0,
        },
    ];
    common::setup();
    for t in &tests {
        assert_eq!(t.want, the_book::add_two(t.data), "{}", t.name);
    }
}
