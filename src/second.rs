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
        enum Test {
            I32(Type<i32>),
            F32(Type<f32>),
            Str(Type<String>),
        };
        struct Type<T> {
            name: &'static str,
            data: Vec<T>,
            want: Vec<Option<T>>,
        }
        let tests = vec![
            Test::I32(Type::<i32> {
                name: "single pop from the empty i32 stack",
                data: vec![],
                want: vec![None],
            }),
            Test::I32(Type::<i32> {
                name: "three pops from the empty i32 stack",
                data: vec![],
                want: vec![None, None, None],
            }),
            Test::I32(Type::<i32> {
                name: "one push and no pop on i32 stack",
                data: vec![1],
                want: vec![],
            }),
            Test::I32(Type::<i32> {
                name: "one push and one pop on i32 stack",
                data: vec![1],
                want: vec![Some(1)],
            }),
            Test::I32(Type::<i32> {
                name: "one push and two pops on i32 stack",
                data: vec![1],
                want: vec![Some(1), None],
            }),
            Test::I32(Type::<i32> {
                name: "two pushes and no pop on i32 stack",
                data: vec![1, 2],
                want: vec![],
            }),
            Test::I32(Type::<i32> {
                name: "two pushes and one pop on i32 stack",
                data: vec![1, 2],
                want: vec![Some(2)],
            }),
            Test::I32(Type::<i32> {
                name: "two pushes and two pops on i32 stack",
                data: vec![1, 2],
                want: vec![Some(2), Some(1)],
            }),
            Test::I32(Type::<i32> {
                name: "two pushes and three pops on i32 stack",
                data: vec![1, 2],
                want: vec![Some(2), Some(1), None],
            }),
            Test::I32(Type::<i32> {
                name: "three pushes and no pop on i32 stack",
                data: vec![1, 2, 3],
                want: vec![],
            }),
            Test::I32(Type::<i32> {
                name: "three pushes and one pop on i32 stack",
                data: vec![1, 2, 3],
                want: vec![Some(3)],
            }),
            Test::I32(Type::<i32> {
                name: "three pushes and two pops on i32 stack",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2)],
            }),
            Test::I32(Type::<i32> {
                name: "three pushes and three pops on i32 stack",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1)],
            }),
            Test::I32(Type::<i32> {
                name: "three pushes and four pops i32 stack",
                data: vec![1, 2, 3],
                want: vec![Some(3), Some(2), Some(1), None],
            }),
            Test::F32(Type::<f32> {
                name: "single pop from the empty f32 stack",
                data: vec![],
                want: vec![None],
            }),
            Test::F32(Type::<f32> {
                name: "three pops from the empty f32 stack",
                data: vec![],
                want: vec![None, None, None],
            }),
            Test::F32(Type::<f32> {
                name: "one push and no pop on f32 stack",
                data: vec![1.1],
                want: vec![],
            }),
            Test::F32(Type::<f32> {
                name: "one push and one pop on f32 stack",
                data: vec![1.1],
                want: vec![Some(1.1)],
            }),
            Test::F32(Type::<f32> {
                name: "one push and two pops on f32 stack",
                data: vec![1.1],
                want: vec![Some(1.1), None],
            }),
            Test::F32(Type::<f32> {
                name: "two pushes and no pop on f32 stack",
                data: vec![1.1, 2.1],
                want: vec![],
            }),
            Test::F32(Type::<f32> {
                name: "two pushes and one pop on f32 stack",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1)],
            }),
            Test::F32(Type::<f32> {
                name: "two pushes and two pops on f32 stack",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1), Some(1.1)],
            }),
            Test::F32(Type::<f32> {
                name: "two pushes and three pops on f32 stack",
                data: vec![1.1, 2.1],
                want: vec![Some(2.1), Some(1.1), None],
            }),
            Test::F32(Type::<f32> {
                name: "three pushes and no pop on f32 stack",
                data: vec![1.1, 2.1, 3.1],
                want: vec![],
            }),
            Test::F32(Type::<f32> {
                name: "three pushes and one pop on f32 stack",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1)],
            }),
            Test::F32(Type::<f32> {
                name: "three pushes and two pops on f32 stack",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1)],
            }),
            Test::F32(Type::<f32> {
                name: "three pushes and three pops on f32 stack",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1), Some(1.1)],
            }),
            Test::F32(Type::<f32> {
                name: "three pushes and four pops on f32 stack",
                data: vec![1.1, 2.1, 3.1],
                want: vec![Some(3.1), Some(2.1), Some(1.1), None],
            }),
            Test::Str(Type::<String> {
                name: "single pop from the empty String stack",
                data: vec![],
                want: vec![None],
            }),
            Test::Str(Type::<String> {
                name: "three pops from the empty String stack",
                data: vec![],
                want: vec![None, None, None],
            }),
            Test::Str(Type::<String> {
                name: "one push and no pop on String stack",
                data: vec!["I".to_string()],
                want: vec![],
            }),
            Test::Str(Type::<String> {
                name: "one push and one pop on String stack",
                data: vec!["I".to_string()],
                want: vec![Some("I".to_string())],
            }),
            Test::Str(Type::<String> {
                name: "one push and two pops on String stack",
                data: vec!["I".to_string()],
                want: vec![Some("I".to_string()), None],
            }),
            Test::Str(Type::<String> {
                name: "two pushes and no pop on String stack",
                data: vec!["I".to_string(), "II".to_string()],
                want: vec![],
            }),
            Test::Str(Type::<String> {
                name: "two pushes and one pop on String stack",
                data: vec!["I".to_string(), "II".to_string()],
                want: vec![Some("II".to_string())],
            }),
            Test::Str(Type::<String> {
                name: "two pushes and two pops on String stack",
                data: vec!["I".to_string(), "II".to_string()],
                want: vec![Some("II".to_string()), Some("I".to_string())],
            }),
            Test::Str(Type::<String> {
                name: "two pushes and three pops on String stack",
                data: vec!["I".to_string(), "II".to_string()],
                want: vec![Some("II".to_string()), Some("I".to_string()), None],
            }),
            Test::Str(Type::<String> {
                name: "three pushes and no pop on String stack",
                data: vec!["I".to_string(), "II".to_string(), "III".to_string()],
                want: vec![],
            }),
            Test::Str(Type::<String> {
                name: "three pushes and one pop on String stack",
                data: vec!["I".to_string(), "II".to_string(), "III".to_string()],
                want: vec![Some("III".to_string())],
            }),
            Test::Str(Type::<String> {
                name: "three pushes and two pops on String stack",
                data: vec!["I".to_string(), "II".to_string(), "III".to_string()],
                want: vec![Some("III".to_string()), Some("II".to_string())],
            }),
            Test::Str(Type::<String> {
                name: "three pushes and three pops on String stack",
                data: vec!["I".to_string(), "II".to_string(), "III".to_string()],
                want: vec![
                    Some("III".to_string()),
                    Some("II".to_string()),
                    Some("I".to_string()),
                ],
            }),
            Test::Str(Type::<String> {
                name: "three pushes and four pops on String stack",
                data: vec!["I".to_string(), "II".to_string(), "III".to_string()],
                want: vec![
                    Some("III".to_string()),
                    Some("II".to_string()),
                    Some("I".to_string()),
                    None,
                ],
            }),
        ];
        for t in &tests {
            match t {
                Test::I32(t) => {
                    let mut list = List::<i32>::new();
                    for data in &t.data {
                        list.push(data.clone());
                    }
                    for want in &t.want {
                        let got = list.pop();
                        debug_assert_eq!(want, &got, "{}", t.name);
                    }
                }
                Test::F32(t) => {
                    let mut list = List::<f32>::new();
                    for data in &t.data {
                        list.push(data.clone());
                    }
                    for want in &t.want {
                        let got = list.pop();
                        debug_assert_eq!(want, &got, "{}", t.name);
                    }
                }
                Test::Str(t) => {
                    let mut list = List::<String>::new();
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
