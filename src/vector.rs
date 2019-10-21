// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn next() {
        let a = vec!['a', 'b', 'c', 'd', 'e'];
        let mut i = a.iter();
        assert_eq!(i.next(), Some(&'a'));
        assert_eq!(i.next(), Some(&'b'));
        assert_eq!(i.next(), Some(&'c'));
        assert_eq!(i.next(), Some(&'d'));
        assert_eq!(i.next(), Some(&'e'));
        for _ in 1..1000 {
            assert_eq!(i.next(), None);
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
                name: &"empty vector size",
                data: vec![],
                want: 0,
            },
            Test {
                name: &"one entry vector size",
                data: vec![1],
                want: 1,
            },
            Test {
                name: &"two entries vector size",
                data: vec![1, 2],
                want: 2,
            },
            Test {
                name: &"three entries vector size",
                data: vec![1, 2, 3],
                want: 3,
            },
        ];
        for t in &tests {
            debug_assert_eq!(t.data.len(), t.want, "{}", t.want);
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
                name: &"get index 0 from the empty vector",
                data: vec![],
                want: vec![(0, None)],
            },
            Test {
                name: &"get index 0, 1, 3, 4 from the empty vector",
                data: vec![],
                want: vec![(0, None),(1, None),(3, None),(4, None)],
            },
            Test {
                name: &"get index 0 from the three entries vector",
                data: vec![1, 2, 3],
                want: vec![(0, Some(&1))],
            },
            Test {
                name: &"get index 0, 1, 3, 4 from the three entries vector",
                data: vec![1, 2, 3],
                want: vec![(0, Some(&1)),(1, Some(&2)),(3, None),(4, None)],
            },
        ];
        for t in &tests {
            for (i, &want) in t.want.iter().enumerate() {
                let got = t.data.get(want.0);
                debug_assert_eq!(got, want.1, "{}", t.name);
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
                name: &"push to the empty vector",
                data: vec![],
                push: vec!['a', 'b', 'c'],
                want: vec!['a', 'b', 'c'],
            },
            Test {
                name: &"push to the existing vector",
                data: vec!['a', 'b', 'c'],
                push: vec!['d', 'e', 'f'],
                want: vec!['a', 'b', 'c', 'd', 'e', 'f'],
            },
            Test {
                name: &"push the duplicate data",
                data: vec!['a', 'b', 'c'],
                push: vec!['a', 'b', 'c'],
                want: vec!['a', 'b', 'c', 'a', 'b', 'c'],
            },
        ];
        for t in &mut tests {
            for a in t.push.iter() {
                t.data.push(*a);
            }
            debug_assert_eq!(t.data, t.want, "{}", t.name);
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
                name: &"1 pop from the empty vector",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: &"4 pops from the empty vector",
                data: vec![],
                want: vec![None, None, None, None],
            },
            Test {
                name: &"1 pop from the one entry vector",
                data: vec![1],
                want: vec![Some(1)],
            },
            Test {
                name: &"4 pops from the one entry vector",
                data: vec![1],
                want: vec![Some(1), None, None, None],
            },
            Test {
                name: &"1 pop from the four entries vector",
                data: vec![1, 2, 3, 4],
                want: vec![Some(4)],
            },
            Test {
                name: &"4 pops from the four entries vector",
                data: vec![1, 2, 3, 4],
                want: vec![Some(4), Some(3), Some(2), Some(1)],
            },
        ];
        for t in &mut tests {
            for want in &t.want {
                let got = t.data.pop();
                debug_assert_eq!(&got, want, "{}", t.name);
            }
        }
    }
}
