//! Creating a [Reference Cycle]
//!
//! [reference cycle]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum CycleList<T> {
    Node(T, RefCell<Rc<Self>>),
    Null,
}

impl<T> CycleList<T> {
    /// next returns the next node of the current node.
    /// It returns `Some<Self>` or `None`.
    pub fn next(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
            Self::Node(_, next) => Some(next),
            Self::Null => None,
        }
    }
}
