//! [Smart Pointers]
//!
//! [smart pointers]: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
pub mod tree;

use std::cell::RefCell;
use std::rc::Rc;

/// [Using Box<T>] to Point to Data on the Heap
///
/// [using box<t>]: https://doc.rust-lang.org/book/ch15-01-box.html
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

/// [Rc<T>], the Reference Counted Smart Pointer
///
/// [rc<t>]: https://doc.rust-lang.org/book/ch15-04-rc.html
pub enum Graph<T> {
    Vertex(T, Vec<Rc<Graph<T>>>),
    Nil,
}

/// Creating a [Reference Cycle]
///
/// [reference cycle]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
#[derive(Debug)]
pub enum RefCellList<T> {
    Node(T, RefCell<Rc<RefCellList<T>>>),
    Null,
}

impl<T> RefCellList<T> {
    /// next returns the next node of the current node.  It returns Some<Self>
    /// or None.
    pub fn next(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Node(_, next) => Some(next),
            Self::Null => None,
        }
    }
}

/// Treating Smart Pointer Like Regular References with [the Deref trait]
///
/// [the deref trait]: https://doc.rust-lang.org/book/ch15-02-deref.html
pub struct MyBox<T>(T)
where
    T: std::fmt::Debug;

impl<T> MyBox<T>
where
    T: std::fmt::Debug,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> Drop for MyBox<T>
where
    T: std::fmt::Debug,
{
    fn drop(&mut self) {
        println!("dropping {:?}", self.0);
    }
}

impl<T> std::ops::Deref for MyBox<T>
where
    T: std::fmt::Debug,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T>
where
    T: std::fmt::Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// LimitTracker type and Messenger trait to demonstrate [RefCell<T>] usage.
///
/// [refcell<t>]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
pub struct LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

impl<'a, T> LimitTracker<'a, T>
where
    T: 'a + Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage >= 0.9 {
            self.messenger
                .send("Urgent: You've used up over 90% of your quota!");
        } else if percentage >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl super::Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }
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
    #[test]
    fn mybox_new() {
        use super::MyBox;
        let x = MyBox::new(3);
        assert_eq!(3, x.0);
    }
    #[test]
    fn mybox_dereference() {
        use super::MyBox;
        use std::ops::Deref;
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y); // multiple deref doesn't cause an issue, as it's just borrows
        assert_eq!(5, *y); // the value the smart pointer is pointing to.
        assert_eq!(5, *(y.deref())); // This is the actual call by the compiler.
    }
    #[test]
    fn mybox_mutable_dereference() {
        use super::MyBox;
        let x = 5;
        let mut y = MyBox(x);
        *y = 6; // Through DerefMut trait.
        assert_eq!(5, x);
        assert_eq!(6, *y);
    }
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
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let want = "Warning: You've used up over 75% of your quota!";
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = super::LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(75);
        assert_eq!(1, mock_messenger.sent_messages.borrow().len());
        assert_eq!(want, mock_messenger.sent_messages.borrow()[0]);
    }
}
