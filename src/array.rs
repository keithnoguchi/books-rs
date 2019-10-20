// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
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
        let t = Test {a: [1, 2, 3, 4, 5], want: 5};
        println!("array={:?}, want={}", t.a, t.want);
    }
}
