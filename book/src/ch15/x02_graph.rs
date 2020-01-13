//! [Rc<T>], the Reference Counted Smart Pointer
//!
//! [rc<t>]: https://doc.rust-lang.org/book/ch15-04-rc.html
use std::rc::Rc;

pub enum Graph<T> {
    Vertex(T, Vec<Rc<Graph<T>>>),
    Nil,
}

#[cfg(test)]
mod tests {
    #[test]
    fn graph() {
        use super::Graph::{Nil, Vertex};
        use std::rc::Rc;
        let wants_b = [1, 3, 4, 5];
        let wants_c = [2, 3, 4, 5];
        let mut a = Rc::new(Nil);
        a = Rc::new(Vertex(5, vec![a]));
        a = Rc::new(Vertex(4, vec![a]));
        a = Rc::new(Vertex(3, vec![a]));
        assert_eq!(1, Rc::strong_count(&a));
        let b = Vertex(1, vec![a.clone()]);
        assert_eq!(2, Rc::strong_count(&a));
        let c = Vertex(2, vec![a.clone()]);
        assert_eq!(3, Rc::strong_count(&a));
        {
            let d = a.clone();
            assert_eq!(4, Rc::strong_count(&a));
            match *d {
                Nil => panic!("unexpected Nil"),
                Vertex(val, _) => assert_eq!(3, val),
            }
        }
        assert_eq!(3, Rc::strong_count(&a));
        let mut got = &b;
        for want in &wants_b {
            got = match got {
                Nil => panic!("unexpected Nil"),
                Vertex(val, next) => {
                    assert_eq!(want, val);
                    &next[0]
                }
            }
        }
        if let Vertex(val, _) = got {
            panic!("unexpected value in b: {}", val);
        }
        got = &c;
        for want in &wants_c {
            got = match got {
                Nil => panic!("unexpected Nil in c"),
                Vertex(val, next) => {
                    assert_eq!(want, val);
                    &next[0]
                }
            }
        }
        if let Vertex(val, _) = got {
            panic!("unexpected value in c: {}", val);
        }
    }
}
