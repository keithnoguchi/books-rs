// SPDX-License-Identifier: GPL-2.0
use futures::executor::block_on;
use futures::join;

fn main() {
    // https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/macro.join.html#examples
    let a = async { 1i32 };
    let b = async { 2i32 };
    block_on(async {
        assert_eq!(join!(a, b), (1, 2));
    })
}
