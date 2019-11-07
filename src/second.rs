// SPDX-License-Identifier: GPL-2.0
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        List { head: None }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
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
        enum TestType {
            I32(Test<i32>),
            F32(Test<f32>),
        };
        struct Test<T> {
            name: &'static str,
            data: Vec<T>,
            want: Vec<Option<T>>,
        }
        let tests = vec![
            TestType::I32(Test::<i32> {
                name: "single pop from the empty stack",
                data: vec![],
                want: vec![None],
            }),
            TestType::I32(Test::<i32> {
                name: "three pops from the empty stack",
                data: vec![],
                want: vec![None, None, None],
            }),
            TestType::I32(Test::<i32> {
                name: "one push and no pop",
                data: vec![1],
                want: vec![],
            }),
            TestType::I32(Test::<i32> {
                name: "one push and one pop",
                data: vec![1],
                want: vec![Some(1)],
            }),
            TestType::I32(Test::<i32> {
                name: "one push and two pops",
                data: vec![1],
                want: vec![Some(1), None],
            }),
            TestType::I32(Test::<i32> {
                name: "two pushes and no pop",
                data: vec![1, 2],
                want: vec![],
            }),
            TestType::I32(Test::<i32> {
                name: "two pushes and one pop",
                data: vec![1, 2],
                want: vec![Some(2)],
            }),
            TestType::I32(Test::<i32> {
                name: "two pushes and two pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1)],
            }),
            TestType::I32(Test::<i32> {
                name: "two pushes and three pops",
                data: vec![1, 2],
                want: vec![Some(2), Some(1), None],
            }),
            TestType::I32(Test::<i32> {
                name: "three pushes and no pop",
                data: vec![1, 2, 3],
                want: vec![],
            }),
            TestType::I32(Test::<i32> {
                name: "three pushes and one pop",
                data: vec![1, 2, 3],
                want: vec![Some(3)],
            }),
            TestType::I32(Test::<i32> {
                name: "three pushes and two pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2)],
            }),
            TestType::I32(Test::<i32> {
                name: "three pushes and three pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1)],
            }),
            TestType::I32(Test::<i32> {
                name: "three pushes and four pops",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1), None],
            }),
            TestType::F32(Test::<f32> {
                name: "single pop from the empty stack",
                data: vec![],
                want: vec![None],
            }),
            TestType::F32(Test::<f32> {
                name: "three pops from the empty stack",
                data: vec![],
                want: vec![None, None, None],
            }),
            TestType::F32(Test::<f32> {
                name: "one push and no pop",
                data: vec![1.1],
                want: vec![],
            }),
            TestType::F32(Test::<f32> {
                name: "one push and one pop",
                data: vec![1.1],
                want: vec![Some(1.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "one push and two pops",
                data: vec![1.1],
                want: vec![Some(1.1), None],
            }),
            TestType::F32(Test::<f32> {
                name: "two pushes and no pop",
                data: vec![1.1, 2.1],
                want: vec![],
            }),
            TestType::F32(Test::<f32> {
                name: "two pushes and one pop",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "two pushes and two pops",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1), Some(1.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "two pushes and three pops",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1), Some(1.1), None],
            }),
            TestType::F32(Test::<f32> {
                name: "three pushes and no pop",
                data: vec![1.1, 2.1, 3.1],
                want: vec![],
            }),
            TestType::F32(Test::<f32> {
                name: "three pushes and one pop",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "three pushes and two pops",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "three pushes and three pops",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1), Some(1.1)],
            }),
            TestType::F32(Test::<f32> {
                name: "three pushes and four pops",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1), Some(1.1), None],
            }),
        ];
        for t in &tests {
            match t {
                TestType::I32(t) => {
                    let mut list = List::<i32>::new();
                    for data in &t.data {
                        list.push(data.clone());
                    }
                    for want in &t.want {
                        let got = list.pop();
                        debug_assert_eq!(want, &got, "{}", t.name);
                    }
                }
                TestType::F32(t) => {
                    let mut list = List::<f32>::new();
                    for data in &t.data {
                        list.push(data.clone());
                    }
                    for want in &t.want {
                        let got = list.pop();
                        debug_assert_eq!(want, &got, "{}", t.name);
                    }
                }
            };
        }
    }
}
