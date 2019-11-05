// SPDX-License-Identifier: GPL-2.0
use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;
    #[test]
    fn push_and_pop() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Vec<Option<i32>>,
        }
        let tests = [
            Test {
                name: &"single pop from the empty stack",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: &"three pops from the empty stack",
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
