// SPDX-License-Identifier: GPL-2.0
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    #[allow(dead_code)]
    pub fn new() -> Self {
        List { head: None }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
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
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
                name: "single pop from the empty stack",
                data: vec![],
                want: vec![None],
            },
            Test {
                name: "three pops from the empty stack",
                data: vec![],
                want: vec![None, None, None],
            },
            Test {
                name: "one push and no pop",
                data: vec![1],
                want: vec![],
            },
            Test {
                name: "one push and one pop",
                data: vec![1],
                want: vec![Some(1)],
            },
            Test {
                name: "one push and two pops",
                data: vec![1],
                want: vec![Some(1), None],
            },
            Test {
                name: "two pushes and no pop",
                data: vec![1, 2],
                want: vec![],
            },
            Test {
                name: "two pushes and one pop",
                data: vec![1, 2],
                want: vec![Some(2)],
            },
            Test {
                name: "two pushes and two pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1)],
            },
            Test {
                name: "two pushes and three pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1), None],
            },
            Test {
                name: "three pushes and no pop",
                data: vec![1, 2, 3],
                want: vec![],
            },
            Test {
                name: "three pushes and one pop",
                data: vec![1, 2, 3],
                want: vec![Some(3)],
            },
            Test {
                name: "three pushes and two pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2)],
            },
            Test {
                name: "three pushes and three pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1)],
            },
            Test {
                name: "three pushes and four pops",
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
