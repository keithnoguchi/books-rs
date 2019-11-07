// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/alloc/borrow/enum.Cow.html
use std::borrow::Cow;

#[allow(dead_code)]
fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    #[test]
    fn abs_all_array() {
        struct Test {
            name: &'static str,
            data: [i32; 3],
            want: &'static str,
        };
        let tests = [
            Test {
                name: &"all positive integers",
                data: [1, 2, 3],
                want: &"[1, 2, 3]",
            },
            Test {
                name: &"all negative integers",
                data: [-1, -2, -3],
                want: &"[1, 2, 3]",
            },
            Test {
                name: &"mixed integers",
                data: [1, -2, 3],
                want: &"[1, 2, 3]",
            },
        ];
        for t in &tests {
            let mut input = Cow::from(&t.data[..]);
            super::abs_all(&mut input);
            debug_assert_eq!(t.want, format!("{:?}", input), "{}", t.name);
        }
    }
    #[test]
    fn abs_all_vector() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: &'static str,
        };
        let tests = [
            Test {
                name: &"no data",
                data: vec![],
                want: &"[]",
            },
            Test {
                name: &"all positive integers",
                data: vec![0, 1, 2],
                want: &"[0, 1, 2]",
            },
            Test {
                name: &"all negative integers",
                data: vec![-9, -1, -2],
                want: &"[9, 1, 2]",
            },
        ];
        for t in &tests {
            let mut input = Cow::from(&t.data);
            super::abs_all(&mut input);
            debug_assert_eq!(t.want, format!("{:?}", input), "{}", t.name);
        }
    }
}
