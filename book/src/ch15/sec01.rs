//! Using [Box<T>] to Point to Data on the Heap
//!
//! [box<t>]: https://doc.rust-lang.org/book/ch15-01-box.html
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    #[test]
    fn list() {
        use super::List::{Cons, Nil};
        let wants = [11, 12, 13, 14];
        let mut list = Nil;
        list = Cons(14, Box::new(list));
        list = Cons(13, Box::new(list));
        list = Cons(12, Box::new(list));
        list = Cons(11, Box::new(list));
        let mut got = &list;
        for want in &wants {
            got = match got {
                Nil => panic!("unexpected premature Nil node"),
                Cons(got, next) => {
                    assert_eq!(want, got);
                    next
                }
            };
        }
        if let Cons(..) = got {
            panic!("unexpected non Nil tail");
        }
    }
    #[test]
    fn reference_dereference() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    #[test]
    fn box_dereference() {
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *y);
    }
}
