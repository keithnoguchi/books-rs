// SPDX-License-Identifier: GPL-2.0
pub fn largest(a: &[i32; 5]) -> &i32 {
    let mut largest = &a[0];
    for i in a.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    #[test]
    fn next() {
        let a = [1, 2, 3, 4, 5];
        let mut i = a.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&4));
        assert_eq!(i.next(), Some(&5));
        // you can call next() forever.
        for _ in 0..1000 {
            assert_eq!(i.next(), None);
        }
    }
    #[test]
    fn slice() {
        struct Test {
            name: &'static str,
            data: [i32; 5],
            range: (usize, usize),
            want: Vec<i32>,
        };
        let tests = [
            Test {
                name: &"first entry only",
                data: [1, 2, 3, 4, 5],
                range: (0, 1),
                want: vec![1],
            },
            Test {
                name: &"first and second entries",
                data: [1, 2, 3, 4, 5],
                range: (0, 2),
                want: vec![1, 2],
            },
            Test {
                name: &"last entry",
                data: [1, 2, 3, 4, 5],
                range: (4, 5),
                want: vec![5],
            },
            Test {
                name: &"last two entries",
                data: [1, 2, 3, 4, 5],
                range: (3, 5),
                want: vec![4, 5],
            },
            Test {
                name: &"all entries",
                data: [1, 2, 3, 4, 5],
                range: (0, 5),
                want: vec![1, 2, 3, 4, 5],
            },
        ];
        for t in &tests {
            let got = &t.data[t.range.0..t.range.1];
            for (i, want) in t.want.iter().enumerate() {
                debug_assert_eq!(&got[i], want, "{}", t.name);
            }
        }
    }
    #[test]
    fn largest() {
        struct Test {
            a: [i32; 5],
            want: i32,
        }
        let tests = [
            Test {
                a: [1, 2, 3, 4, 5],
                want: 5,
            },
            Test {
                a: [5, 4, 3, 2, 1],
                want: 5,
            },
            Test {
                a: [1, 4, 3, 2, 5],
                want: 5,
            },
        ];
        for t in &tests {
            assert_eq!(super::largest(&t.a), &t.want);
        }
    }
}
