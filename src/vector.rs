// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn iterate_char_vector_with_next() {
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
    fn vector_size_with_len() {
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
    fn push_on_char_vector() {
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
}
