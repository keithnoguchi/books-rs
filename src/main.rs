// SPDX-License-Identifier: GPL-2.0
use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}
