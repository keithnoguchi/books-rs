// SPDX-License-Identifier: GPL-2.0
// https://doc.rust-lang.org/alloc/boxed/index.html
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    #[test]
    fn recursive_list() {
        use super::List;
        struct Test {
            name: &'static str,
            list: List<u32>,
            want: &'static str,
        };
        let tests = [
            Test {
                name: &"nil entry",
                list: List::Nil,
                want: &"Nil",
            },
            Test {
                name: &"one list entry",
                list: List::Cons(1, Box::new(List::Nil)),
                want: &"Cons(1, Nil)",
            },
            Test {
                name: &"two list entries",
                list: List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))),
                want: &"Cons(1, Cons(2, Nil))",
            },
            Test {
                name: &"three list entries",
                list: List::Cons(
                    1,
                    Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
                ),
                want: &"Cons(1, Cons(2, Cons(3, Nil)))",
            },
        ];
        for t in &tests {
            debug_assert_eq!(t.want, format!("{:?}", t.list), "{}", t.name);
        }
    }
    #[test]
    fn stack_to_heap() {
        struct Test {
            name: &'static str,
            data: i8,
            want: i8,
        }
        let tests = [
            Test {
                name: &"zero",
                data: 0,
                want: 0,
            },
            Test {
                name: &"number eight",
                data: 8,
                want: 8,
            },
            Test {
                name: &"minus one",
                data: -1,
                want: -1,
            },
        ];
        for t in &tests {
            let boxed: Box<i8> = Box::new(t.data);
            debug_assert_eq!(t.want, *boxed, "{}", t.name);
        }
    }
}
