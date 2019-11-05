// SPDX-License-Identifier: GPL-2.0
use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn push_and_pop() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Vec<Option<i32>>,
        }
        let tests = [
            Test {
                name: &"single pop on empty list",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: &"three pop on empty list",
                data: vec![],
                want: vec![None, None, None],
            },
        ];
        for t in &tests {
            let mut l = List::new();
            for data in &t.data {
                l.push(data.clone());
            }
            for want in &t.want {
                let got = l.pop();
                debug_assert_eq!(want, &got, "{}", t.name);
            }
        }
    }
}
