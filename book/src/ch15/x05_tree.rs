//! [Preventing Reference Cycles]: Turning an `Rc<T>` into a `Weak<T>`
//!
//! [preventing reference cycles]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles-turning-an-rct-into-a-weakt
//!
//! # Examples
//!
//! ```rust
//! use std::error::Error;
//! use std::rc::Rc;
//!
//! use the_book::ch15::x05_tree::Node;
//!
//! let children = vec![
//!     Rc::new(Node::new("alice")),
//!     Rc::new(Node::new("bob")),
//!     Rc::new(Node::new("chris")),
//! ];
//! let parent = Rc::new(Node::new("parent"));
//!
//! for child in children {
//!     parent.clone().add_child(child);
//! }
//! assert_eq!(1, Rc::strong_count(&parent));
//! assert_eq!(3, Rc::weak_count(&parent));
//! ```
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

/// [Node] maintains the data and the relationship with the
/// parent [Node] and the child [Node]s.
///
/// [node]: struct.Node.html
#[derive(Debug)]
pub struct Node<T>
where
    T: Debug,
{
    data: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Debug,
{
    /// `new()` method to create a leaf [Node] with the specified
    /// `data`.
    ///
    /// [node]: struct.Node.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch15::x05_tree::Node;
    ///
    /// let node = Node::new('c');
    /// assert_eq!(&'c', node.data());
    /// ```
    pub fn new(data: T) -> Self {
        Self {
            data,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
    /// `data()` method returns the reference of the data maintained
    /// in the [Node].
    ///
    /// [node]: struct.Node.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use the_book::ch15::x05_tree::Node;
    ///
    /// let node = Node::new(String::from("this is a test node"));
    /// let want = "this is a test node";
    /// assert_eq!(want, node.data());
    /// ```
    pub fn data(&self) -> &T {
        &self.data
    }
    /// `add_child()` method adds [Node] child.  It also creates
    /// a weak relationship between the child [Node] and the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::rc::Rc;
    ///
    /// use the_book::ch15::x05_tree::Node;
    ///
    /// let child = Rc::new(Node::new(5));
    /// let parent = Rc::new(Node::new(0));
    /// parent.clone().add_child(child.clone());
    /// assert_eq!(&5, child.data());
    /// assert_eq!(&0, parent.data());
    /// assert_eq!(1, Rc::strong_count(&parent));
    /// assert_eq!(1, Rc::weak_count(&parent));
    /// assert_eq!(2, Rc::strong_count(&child));
    /// ```
    pub fn add_child(self: Rc<Self>, child: Rc<Self>) {
        *child.parent.borrow_mut() = Rc::downgrade(&self.clone());
        self.children.borrow_mut().push(child);
    }
}

impl<T> Drop for Node<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("dropping {:?}", self.data());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn node_new() {
        use super::Node;

        let node = Node::new(5);
        assert_eq!(&5, node.data());
    }
    #[test]
    fn add_children() {
        use super::Node;
        use std::rc::Rc;

        let children = vec![
            Rc::new(Node::new("alice")),
            Rc::new(Node::new("bob")),
            Rc::new(Node::new("chris")),
        ];
        let parent = Rc::new(Node::new("parent"));
        for child in children {
            parent.clone().add_child(child);
        }
        assert_eq!(1, Rc::strong_count(&parent));
        assert_eq!(3, Rc::weak_count(&parent));
        assert_eq!(3, parent.children.borrow().len());
        assert_eq!(&"alice", parent.children.borrow()[0].data());
        assert_eq!(&"bob", parent.children.borrow()[1].data());
        assert_eq!(&"chris", parent.children.borrow()[2].data());
    }
}
