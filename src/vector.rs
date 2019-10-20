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
}
