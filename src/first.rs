// SPDX-License-Identifier: GPL-2.0
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
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
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
    pub fn pop_node(&mut self) -> Link {
        mem::replace(&mut self.head, Link::Empty)
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.pop_node();
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
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
                name: &"single pop on the empty list",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: &"three pops on the empty list",
                data: vec![],
                want: vec![None, None, None],
            },
            Test {
                name: &"one push and no pop",
                data: vec![1],
                want: vec![],
            },
            Test {
                name: &"one push and one pop",
                data: vec![1],
                want: vec![Some(1)],
            },
            Test {
                name: &"one push and two pops",
                data: vec![1],
                want: vec![Some(1), None],
            },
            Test {
                name: &"two pushes and no pop",
                data: vec![1, 2],
                want: vec![],
            },
            Test {
                name: &"two pushes and one pop",
                data: vec![1, 2],
                want: vec![Some(2)],
            },
            Test {
                name: &"two pushes and two pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1)],
            },
            Test {
                name: &"two pushes and three pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1), None],
            },
            Test {
                name: &"three pushes and no pop",
                data: vec![1, 2, 3],
                want: vec![],
            },
            Test {
                name: &"three pushes and one pop",
                data: vec![1, 2, 3],
                want: vec![Some(3)],
            },
            Test {
                name: &"three pushes and two pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2)],
            },
            Test {
                name: &"three pushes and three pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1)],
            },
            Test {
                name: &"three pushes and four pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1), None],
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
