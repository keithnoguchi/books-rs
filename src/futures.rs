// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod test {
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/macro.join.html
    #[test]
    fn join() {
        use futures::executor::block_on;
        use futures::join;
        struct Test {
            name: &'static str,
            a: i32,
            b: i32,
            want: (i32, i32),
        }
        let tests = [
            Test {
                name: "pass 1 and 2",
                a: 1,
                b: 2,
                want: (1, 2),
            },
            Test {
                name: "pass 9 and 1000",
                a: 9,
                b: 1000,
                want: (9, 1000),
            },
        ];
        for t in &tests {
            let a = async { t.a };
            let b = async { t.b };
            block_on(async {
                debug_assert_eq!(join!(a, b), t.want, "{}", t.name);
            })
        }
    }
}
