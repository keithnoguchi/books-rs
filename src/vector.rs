// SPDX-License-Identifier: GPL-2.0
use std::{cmp, fmt, io};

#[derive(Debug)]
enum Error {
    Io(io::Error),
    #[allow(dead_code)]
    Other,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(ref err) => err.fmt(f),
            Error::Other => write!(f, "other error"),
        }
    }
}

impl From<io::ErrorKind> for Error {
    fn from(kind: io::ErrorKind) -> Self {
        Error::Io(io::Error::from(kind))
    }
}

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::Io(ref err) => {
                if let Error::Io(ref other) = other {
                    other.kind() == err.kind()
                } else {
                    false
                }
            }
            Error::Other => {
                if let Error::Other = other {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[allow(dead_code)]
fn largest<T: PartialOrd + Clone>(list: &[T]) -> Result<T, Error> {
    if let Some(mut largest) = list.get(0) {
        for value in list {
            if value > largest {
                largest = value
            }
        }
        return Ok(largest.clone());
    }
    Err(Error::from(io::ErrorKind::InvalidInput))
}

#[allow(dead_code)]
fn largest_iter<T: PartialOrd + Clone>(list: &[T]) -> Result<T, Error> {
    let mut i = list.iter();
    if let Some(mut largest) = i.next() {
        loop {
            if let Some(value) = i.next() {
                if value > largest {
                    largest = value;
                }
                continue;
            }
            return Ok(largest.clone());
        }
    }
    Err(Error::from(io::ErrorKind::InvalidInput))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next() {
        let a = vec!['a', 'b', 'c', 'd', 'e'];
        let mut i = a.iter();
        assert_eq!(Some(&'a'), i.next());
        assert_eq!(Some(&'b'), i.next());
        assert_eq!(Some(&'c'), i.next());
        assert_eq!(Some(&'d'), i.next());
        assert_eq!(Some(&'e'), i.next());
        for _ in 1..1000 {
            assert_eq!(None, i.next());
        }
    }
    #[test]
    fn len() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: usize,
        };
        let tests = [
            Test {
                name: "empty vector size",
                data: vec![],
                want: 0,
            },
            Test {
                name: "one entry vector size",
                data: vec![1],
                want: 1,
            },
            Test {
                name: "two entries vector size",
                data: vec![1, 2],
                want: 2,
            },
            Test {
                name: "three entries vector size",
                data: vec![1, 2, 3],
                want: 3,
            },
        ];
        for t in &tests {
            debug_assert_eq!(t.want, t.data.len(), "{}", t.name);
        }
    }
    #[test]
    fn get() {
        struct Test {
            name: &'static str,
            data: Vec<i64>,
            want: Vec<(usize, Option<&'static i64>)>,
        };
        let tests = [
            Test {
                name: "get index 0 from the empty vector",
                data: vec![],
                want: vec![(0, None)],
            },
            Test {
                name: "get index 0, 1, 3, 4 from the empty vector",
                data: vec![],
                want: vec![(0, None), (1, None), (3, None), (4, None)],
            },
            Test {
                name: "get index 0 from the three entries vector",
                data: vec![1, 2, 3],
                want: vec![(0, Some(&1))],
            },
            Test {
                name: "get index 0, 1, 3, 4 from the three entries vector",
                data: vec![1, 2, 3],
                want: vec![(0, Some(&1)), (1, Some(&2)), (3, None), (4, None)],
            },
        ];
        for t in &tests {
            for want in t.want.iter() {
                let got = t.data.get(want.0);
                debug_assert_eq!(want.1, got, "{}", t.name);
            }
        }
    }
    #[test]
    fn push() {
        struct Test {
            name: &'static str,
            data: Vec<char>,
            push: Vec<char>,
            want: Vec<char>,
        }
        let mut tests = [
            Test {
                name: "push to the empty vector",
                data: vec![],
                push: vec!['a', 'b', 'c'],
                want: vec!['a', 'b', 'c'],
            },
            Test {
                name: "push to the existing vector",
                data: vec!['a', 'b', 'c'],
                push: vec!['d', 'e', 'f'],
                want: vec!['a', 'b', 'c', 'd', 'e', 'f'],
            },
            Test {
                name: "push the duplicate data",
                data: vec!['a', 'b', 'c'],
                push: vec!['a', 'b', 'c'],
                want: vec!['a', 'b', 'c', 'a', 'b', 'c'],
            },
        ];
        for t in &mut tests {
            for a in t.push.iter() {
                t.data.push(*a);
            }
            debug_assert_eq!(t.want, t.data, "{}", t.name);
        }
    }
    #[test]
    fn pop() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Vec<Option<i32>>,
        };
        let mut tests = [
            Test {
                name: "1 pop from the empty vector",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: "4 pops from the empty vector",
                data: vec![],
                want: vec![None, None, None, None],
            },
            Test {
                name: "1 pop from the one entry vector",
                data: vec![1],
                want: vec![Some(1)],
            },
            Test {
                name: "4 pops from the one entry vector",
                data: vec![1],
                want: vec![Some(1), None, None, None],
            },
            Test {
                name: "1 pop from the four entries vector",
                data: vec![1, 2, 3, 4],
                want: vec![Some(4)],
            },
            Test {
                name: "4 pops from the four entries vector",
                data: vec![1, 2, 3, 4],
                want: vec![Some(4), Some(3), Some(2), Some(1)],
            },
        ];
        for t in &mut tests {
            for want in &t.want {
                let got = t.data.pop();
                debug_assert_eq!(want, &got, "{}", t.name);
            }
        }
    }
    #[test]
    fn largest_ok_i32() {
        const NAME: &str = "largest_ok_i32";
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: i32,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec![1],
                want: 1,
            },
            Test {
                name: "ascending two elements vector",
                data: vec![1, 2],
                want: 2,
            },
            Test {
                name: "descending two elements vector",
                data: vec![2, 1],
                want: 2,
            },
            Test {
                name: "ascending five elements vector",
                data: vec![1, 2, 3, 4, 5],
                want: 5,
            },
            Test {
                name: "decending five elements vector",
                data: vec![5, 4, 3, 2, 1],
                want: 5,
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![1, 5, 3, 2, 4],
                want: 5,
            },
        ];
        for t in &tests {
            match largest::<i32>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_err_i32() {
        const NAME: &str = "largest_err_i32";
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Error,
        }
        let tests = [Test {
            name: "empty i32 vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest::<i32>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_ok_f32() {
        const NAME: &str = "largest_ok_f32";
        struct Test {
            name: &'static str,
            data: Vec<f32>,
            want: f32,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec![1.1],
                want: 1.1,
            },
            Test {
                name: "ascending two elements vector",
                data: vec![1.1, 2.1],
                want: 2.1,
            },
            Test {
                name: "descending two elements vector",
                data: vec![2.9, 1.1],
                want: 2.9,
            },
            Test {
                name: "ascending five elements vector",
                data: vec![1.1, 2.1, 3.1, 4.1, 5.1],
                want: 5.1,
            },
            Test {
                name: "decending five elements vector",
                data: vec![5.9, 4.9, 3.9, 2.9, 1.9],
                want: 5.9,
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![1.5, 5.5, 3.5, 2.5, 4.5],
                want: 5.5,
            },
        ];
        for t in &tests {
            match largest::<f32>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_err_f32() {
        const NAME: &str = "largest_err_f32";
        struct Test {
            name: &'static str,
            data: Vec<f32>,
            want: Error,
        }
        let tests = [Test {
            name: "empty f32 vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest::<f32>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_ok_char() {
        const NAME: &str = "largest_ok_char";
        struct Test {
            name: &'static str,
            data: Vec<char>,
            want: char,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec!['a'],
                want: 'a',
            },
            Test {
                name: "ascending two elements vector",
                data: vec!['a', 'b'],
                want: 'b',
            },
            Test {
                name: "descending two elements vector",
                data: vec!['b', 'a'],
                want: 'b',
            },
            Test {
                name: "ascending five elements vector",
                data: vec!['a', 'b', 'c', 'd', 'e'],
                want: 'e',
            },
            Test {
                name: "decending five elements vector",
                data: vec!['e', 'd', 'c', 'b', 'a'],
                want: 'e',
            },
            Test {
                name: "unsorted five elements vector",
                data: vec!['a', 'e', 'b', 'c', 'd'],
                want: 'e',
            },
        ];
        for t in &tests {
            match largest::<char>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_err_char() {
        const NAME: &str = "largest_err_char";
        struct Test {
            name: &'static str,
            data: Vec<char>,
            want: Error,
        }
        let tests = [Test {
            name: "empty char vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest::<char>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_ok_string() {
        const NAME: &str = "largest_ok_string";
        struct Test {
            name: &'static str,
            data: Vec<String>,
            want: String,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec!["a".to_string()],
                want: "a".to_string(),
            },
            Test {
                name: "ascending two elements vector",
                data: vec!["a".to_string(), "b".to_string()],
                want: "b".to_string(),
            },
            Test {
                name: "descending two elements vector",
                data: vec!["b".to_string(), "a".to_string()],
                want: "b".to_string(),
            },
            Test {
                name: "ascending five elements vector",
                data: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
                want: "e".to_string(),
            },
            Test {
                name: "decending five elements vector",
                data: vec![
                    "e".to_string(),
                    "d".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "a".to_string(),
                ],
                want: "e".to_string(),
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![
                    "a".to_string(),
                    "e".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ],
                want: "e".to_string(),
            },
        ];
        for t in &tests {
            match largest::<String>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_err_string() {
        const NAME: &str = "largest_err_string";
        struct Test {
            name: &'static str,
            data: Vec<String>,
            want: Error,
        }
        let tests = [Test {
            name: "empty string vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest::<String>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_ok_i32() {
        const NAME: &str = "largest_iter_ok_i32";
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: i32,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec![1],
                want: 1,
            },
            Test {
                name: "ascending two elements vector",
                data: vec![1, 2],
                want: 2,
            },
            Test {
                name: "descending two elements vector",
                data: vec![2, 1],
                want: 2,
            },
            Test {
                name: "ascending five elements vector",
                data: vec![1, 2, 3, 4, 5],
                want: 5,
            },
            Test {
                name: "decending five elements vector",
                data: vec![5, 4, 3, 2, 1],
                want: 5,
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![1, 5, 3, 2, 4],
                want: 5,
            },
        ];
        for t in &tests {
            match largest_iter::<i32>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_err_i32() {
        const NAME: &str = "largest_iter_err_i32";
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Error,
        }
        let tests = [Test {
            name: "empty i32 vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest_iter::<i32>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_ok_f32() {
        const NAME: &str = "largest_iter_ok_f32";
        struct Test {
            name: &'static str,
            data: Vec<f32>,
            want: f32,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec![1.1],
                want: 1.1,
            },
            Test {
                name: "ascending two elements vector",
                data: vec![1.1, 2.1],
                want: 2.1,
            },
            Test {
                name: "descending two elements vector",
                data: vec![2.9, 1.1],
                want: 2.9,
            },
            Test {
                name: "ascending five elements vector",
                data: vec![1.1, 2.1, 3.1, 4.1, 5.1],
                want: 5.1,
            },
            Test {
                name: "decending five elements vector",
                data: vec![5.9, 4.9, 3.9, 2.9, 1.9],
                want: 5.9,
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![1.5, 5.5, 3.5, 2.5, 4.5],
                want: 5.5,
            },
        ];
        for t in &tests {
            match largest_iter::<f32>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_err_f32() {
        const NAME: &str = "largest_iter_err_f32";
        struct Test {
            name: &'static str,
            data: Vec<f32>,
            want: Error,
        }
        let tests = [Test {
            name: "empty f32 vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest_iter::<f32>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_ok_char() {
        const NAME: &str = "largest_iter_ok_char";
        struct Test {
            name: &'static str,
            data: Vec<char>,
            want: char,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec!['a'],
                want: 'a',
            },
            Test {
                name: "ascending two elements vector",
                data: vec!['a', 'b'],
                want: 'b',
            },
            Test {
                name: "descending two elements vector",
                data: vec!['b', 'a'],
                want: 'b',
            },
            Test {
                name: "ascending five elements vector",
                data: vec!['a', 'b', 'c', 'd', 'e'],
                want: 'e',
            },
            Test {
                name: "decending five elements vector",
                data: vec!['e', 'd', 'c', 'b', 'a'],
                want: 'e',
            },
            Test {
                name: "unsorted five elements vector",
                data: vec!['a', 'e', 'b', 'c', 'd'],
                want: 'e',
            },
        ];
        for t in &tests {
            match largest_iter::<char>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_err_char() {
        const NAME: &str = "largest_iter_err_char";
        struct Test {
            name: &'static str,
            data: Vec<char>,
            want: Error,
        }
        let tests = [Test {
            name: "empty char vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest_iter::<char>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_ok_string() {
        const NAME: &str = "largest_iter_ok_string";
        struct Test {
            name: &'static str,
            data: Vec<String>,
            want: String,
        }
        let tests = [
            Test {
                name: "single element vector",
                data: vec!["a".to_string()],
                want: "a".to_string(),
            },
            Test {
                name: "ascending two elements vector",
                data: vec!["a".to_string(), "b".to_string()],
                want: "b".to_string(),
            },
            Test {
                name: "descending two elements vector",
                data: vec!["b".to_string(), "a".to_string()],
                want: "b".to_string(),
            },
            Test {
                name: "ascending five elements vector",
                data: vec![
                    "a".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                    "e".to_string(),
                ],
                want: "e".to_string(),
            },
            Test {
                name: "decending five elements vector",
                data: vec![
                    "e".to_string(),
                    "d".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "a".to_string(),
                ],
                want: "e".to_string(),
            },
            Test {
                name: "unsorted five elements vector",
                data: vec![
                    "a".to_string(),
                    "e".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "d".to_string(),
                ],
                want: "e".to_string(),
            },
        ];
        for t in &tests {
            match largest_iter::<String>(&t.data) {
                Err(err) => {
                    let msg = format!("{}({}): {}", NAME, t.name, err);
                    panic!("{}", msg);
                }
                Ok(got) => {
                    debug_assert_eq!(t.want, got, "{}({})", NAME, t.name);
                }
            }
        }
    }
    #[test]
    fn largest_iter_err_string() {
        const NAME: &str = "largest_iter_err_string";
        struct Test {
            name: &'static str,
            data: Vec<String>,
            want: Error,
        }
        let tests = [Test {
            name: "empty string vector",
            data: vec![],
            want: Error::from(std::io::ErrorKind::InvalidInput),
        }];
        for t in &tests {
            match largest_iter::<String>(&t.data) {
                Ok(_) => {
                    let msg = format!("{}({}): unexpected success", NAME, t.name);
                    panic!(msg)
                }
                Err(err) => {
                    debug_assert_eq!(t.want, err, "{}({})", NAME, t.name);
                }
            }
        }
    }
}
