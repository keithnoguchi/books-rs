//! Build your own [block_on()] with crashing recursive block_on() call
//!
//! # Examples
//!
//! ```no_run
//! $ rustup default nightly
//! info: using existing install for 'nightly-x86_64-unknown-linux-gnu'
//! info: default toolchain set to 'nightly-x86_64-unknown-linux-gnu'
//!
//! nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.43.0-nightly (58b834344 2020-02-05)
//!
//! $ cargo version
//! cargo 1.42.0-nightly (9d32b7b01 2020-01-26)
//! $ cargo bench
//! Finished bench [optimized] target(s) in 0.04s
//! Running /home/kei/git/books-rs/target/release/deps/stjepang-65beb570857a59af
//!
//! running 1 test
//! test blog20200125::v4::tests::should_panic_with_recursive_block_on ... ignored
//!
//! test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
//!
//! Running /home/kei/git/books-rs/target/release/deps/block_on_v4-d099975acef20c10
//!
//! running 12 tests
//! test async_block_on_0_yields    ... bench:          35 ns/iter (+/- 4)
//! test async_block_on_10_yields   ... bench:       3,391 ns/iter (+/- 254)
//! test async_block_on_50_yields   ... bench:      16,039 ns/iter (+/- 1,726)
//! test custom_block_on_0_yields   ... bench:           4 ns/iter (+/- 1)
//! test custom_block_on_10_yields  ... bench:         191 ns/iter (+/- 21)
//! test custom_block_on_50_yields  ... bench:         952 ns/iter (+/- 57)
//! test futures_block_on_0_yields  ... bench:          14 ns/iter (+/- 0)
//! test futures_block_on_10_yields ... bench:         341 ns/iter (+/- 41)
//! test futures_block_on_50_yields ... bench:       1,706 ns/iter (+/- 132)
//! test tokio_block_on_0_yields    ... bench:          10 ns/iter (+/- 2)
//! test tokio_block_on_10_yields   ... bench:         190 ns/iter (+/- 12)
//! test tokio_block_on_50_yields   ... bench:         922 ns/iter (+/- 51)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 12 measured; 0 filtered out
//! ```
//! [block_on()]: https://stjepang.github.io/2020/01/25/build-your-own-block-on.html
#![feature(test)]

extern crate test;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use stjepang::blog20200125::v4::block_on;

use test::Bencher;

struct Yields(u32);

impl Future for Yields {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 == 0 {
            Poll::Ready(())
        } else {
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[bench]
fn custom_block_on_0_yields(b: &mut Bencher) {
    b.iter(|| block_on(Yields(0)));
}

#[bench]
fn custom_block_on_10_yields(b: &mut Bencher) {
    b.iter(|| block_on(Yields(10)));
}

#[bench]
fn custom_block_on_50_yields(b: &mut Bencher) {
    b.iter(|| block_on(Yields(50)));
}

#[bench]
fn futures_block_on_0_yields(b: &mut Bencher) {
    b.iter(|| futures::executor::block_on(Yields(0)));
}

#[bench]
fn futures_block_on_10_yields(b: &mut Bencher) {
    b.iter(|| futures::executor::block_on(Yields(10)));
}

#[bench]
fn futures_block_on_50_yields(b: &mut Bencher) {
    b.iter(|| futures::executor::block_on(Yields(50)));
}

#[bench]
fn async_block_on_0_yields(b: &mut Bencher) {
    b.iter(|| async_std::task::block_on(Yields(0)));
}

#[bench]
fn async_block_on_10_yields(b: &mut Bencher) {
    b.iter(|| async_std::task::block_on(Yields(10)));
}

#[bench]
fn async_block_on_50_yields(b: &mut Bencher) {
    b.iter(|| async_std::task::block_on(Yields(50)));
}

#[bench]
fn tokio_block_on_0_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || runtime.block_on(Yields(0)));
}

#[bench]
fn tokio_block_on_10_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || runtime.block_on(Yields(10)));
}

#[bench]
fn tokio_block_on_50_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || runtime.block_on(Yields(50)));
}
