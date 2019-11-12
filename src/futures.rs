// SPDX-License-Identifier: GPL-2.0
use std::future;

#[allow(dead_code)]
async fn foo(x: u8) -> u8 {
    x
}

#[allow(dead_code)]
fn bar() -> impl future::Future<Output = u8> {
    async {
        let y = 5u8;
        let x: u8 = foo(y).await;
        x + 5
    }
}

#[cfg(test)]
mod test {
    // https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
    #[test]
    fn async_bar_foo() {
        assert_eq!(10, futures::executor::block_on(super::bar()));
    }
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/macro.join.html
    #[test]
    fn join() {
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
            use futures::executor::block_on;
            use futures::join;
            let a = async { t.a };
            let b = async { t.b };
            block_on(async {
                debug_assert_eq!(t.want, join!(a, b), "{}", t.name);
            })
        }
    }
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/macro.select.html
    #[test]
    fn select() {
        struct Test {
            name: &'static str,
            a: i32,
            b: i32,
            want: i32,
        }
        let tests = [
            Test {
                name: "wait for 4 + 1 = 5",
                a: 4,
                b: 1,
                want: 5,
            },
            Test {
                name: "wait for 0 + 3 = 3",
                a: 0,
                b: 3,
                want: 3,
            },
        ];
        for t in &tests {
            use futures::executor::block_on;
            use futures::future;
            use futures::select;
            let mut a = future::ready(t.a);
            let mut b = future::pending::<()>();

            block_on(async {
                let got = select! {
                    got = a => got + t.b,
                    _ = b => 0,
                };
                debug_assert_eq!(t.want, got, "{}", t.name);
            })
        }
    }
}
