//! Singly linked-list
use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

type RefNode<T> = Rc<RefCell<Node<T>>>;

pub struct List<T> {
    head: Option<RefNode<T>>,
}

impl<T: Clone> List<T> {
    /// Create a doubly linked-list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append new value.
    pub fn append(&mut self, data: T) {
        if let Some(tail) = self.tail() {
            tail.borrow_mut().next = Some(Rc::new(RefCell::new(Node { data, next: None })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node { data, next: None })));
        }
    }

    /// Returns the iterator.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().cloned(),
        }
    }

    /// Get the tail of the list.
    fn tail(&self) -> Option<RefNode<T>> {
        if let Some(node) = self.head.as_ref().cloned() {
            if node.borrow().next.is_none() {
                return Some(node);
            } else {
                return Node::tail(&node);
            }
        }
        None
    }
}

impl<T: Clone> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "[")?;
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(f, "{}", n.borrow().data)?;
            node = n.borrow().next.clone();
            if node.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

pub struct Iter<T> {
    next: Option<RefNode<T>>,
}

impl<'a, T> Iterator for Iter<T> {
    type Item = RefNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next.as_ref().cloned() {
            self.next = node.borrow().next.clone();
            return Some(node);
        }
        None
    }
}

pub struct Node<T> {
    data: T,
    next: Option<RefNode<T>>,
}

impl<T> Node<T> {
    /// Returns the inner value.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Update data.
    pub fn set(&mut self, data: T) {
        self.data = data;
    }

    /// Returns the tail of the current linked-list.
    fn tail(node: &RefNode<T>) -> Option<RefNode<T>> {
        if let Some(next) = node.borrow().next.as_ref().cloned() {
            return Self::tail(&next);
        }
        Some(node.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn append() {
        let want = vec![5, 6, 7];
        let mut list = List::new();
        list.append(5);
        list.append(6);
        list.append(7);
        for (i, got) in list.iter().enumerate() {
            assert_eq!(want[i], *got.borrow().data());
        }
        // increment by one.
        for got in list.iter() {
            let data = *got.borrow().data();
            got.borrow_mut().set(data + 1);
        }
        for (i, got) in list.iter().enumerate() {
            assert_eq!(want[i] + 1, *got.borrow().data());
        }
    }
    #[test]
    fn append_update() {
        let want = vec![6, 7, 8];
        let mut list = List::new();
        list.append(5);
        list.append(6);
        list.append(7);
        for got in list.iter() {
            let data = *got.borrow().data();
            got.borrow_mut().set(data + 1);
        }
        for (i, got) in list.iter().enumerate() {
            assert_eq!(want[i], *got.borrow().data());
        }
    }
}
