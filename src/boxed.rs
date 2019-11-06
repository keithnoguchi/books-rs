// SPDX-License-Identifier: GPL-2.0

#[cfg(test)]
mod tests {
    #[test]
    fn stack_to_heap_i8() {
        // https://doc.rust-lang.org/alloc/boxed/index.html
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
