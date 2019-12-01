// SPDX-License-Identifier: GPL-2.0
use std::borrow::Borrow;
use std::borrow::BorrowMut;
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

#[allow(dead_code)]
fn borrow_check<T: Borrow<str>>(want: &str, s: T) {
    assert_eq!(want, s.borrow());
}

#[allow(dead_code)]
fn borrow_mut_check<T: BorrowMut<[i32]>>(mut v: T) {
    assert_eq!(&mut [1, 2, 3], v.borrow_mut());
}

#[cfg(test)]
mod tests {
    // https://doc.rust-lang.org/alloc/borrow/enum.Cow.html
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
                name: "all positive integers",
                data: [1, 2, 3],
                want: "[1, 2, 3]",
            },
            Test {
                name: "all negative integers",
                data: [-1, -2, -3],
                want: "[1, 2, 3]",
            },
            Test {
                name: "mixed integers",
                data: [1, -2, 3],
                want: "[1, 2, 3]",
            },
        ];
        for t in &tests {
            let mut input = Cow::from(&t.data[..]);
            super::abs_all(&mut input);
            assert_eq!(t.want, format!("{:?}", input), "{}", t.name);
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
                name: "no data",
                data: vec![],
                want: "[]",
            },
            Test {
                name: "all positive integers",
                data: vec![0, 1, 2],
                want: "[0, 1, 2]",
            },
            Test {
                name: "all negative integers",
                data: vec![-9, -1, -2],
                want: "[9, 1, 2]",
            },
        ];
        for t in &tests {
            let mut input = Cow::from(&t.data);
            super::abs_all(&mut input);
            assert_eq!(t.want, format!("{:?}", input), "{}", t.name);
        }
    }
    // https://doc.rust-lang.org/alloc/borrow/trait.Borrow.html
    #[test]
    fn borrow() {
        let s = "Hello".to_string();
        super::borrow_check("Hello", s);
        let s = "Hello";
        super::borrow_check("Hello", s);
    }
    // https://doc.rust-lang.org/alloc/borrow/trait.BorrowMut.html
    #[test]
    fn borrow_mut() {
        let v = vec![1, 2, 3];
        super::borrow_mut_check(v);
        let v = [1, 2, 3];
        super::borrow_mut_check(v);
    }
    // https://doc.rust-lang.org/alloc/borrow/trait.ToOwned.html
    #[test]
    fn to_owned() {
        let want: &str = "a";
        let got: String = want.to_owned();
        assert_eq!(want, got);
        let want: &[i32] = &[1, 2];
        let got: Vec<i32> = want.to_owned();
        assert_eq!(want.len(), got.len());
        for i in 0..got.len() {
            assert_eq!(want[i], got[i]);
        }
    }
}
