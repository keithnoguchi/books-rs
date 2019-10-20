// SPDX-License-Identifier: GPL-2.0
pub fn largest_of_five(a: [i32; 5]) -> i32 {
    let mut largest = a[0];
    for i in 1..a.len() {
        if a[i] > largest {
            largest = a[i];
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterate_integer_array_with_next() {
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
    fn largest_of_the_array() {
        struct Test {
            a: [i32; 5],
            want: i32,
        }
        let tests = [
            Test {a: [1, 2, 3, 4, 5], want: 5},
            Test {a: [5, 4, 3, 2, 1], want: 5},
            Test {a: [1, 4, 3, 2, 5], want: 5},
        ];
        largest_of_five([1, 2, 3, 4, 5]);
        for t in &tests {
            assert_eq!(largest_of_five(t.a), t.want);
        }
    }
}
