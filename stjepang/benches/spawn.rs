//! Build your own [executor]
//!
//! [executor]: https://stjepang.github.io/2020/01/31/build-your-own-executor.html
#![feature(test)]
extern crate test;

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use stjepang_blog::post20200125::v4::block_on;
use stjepang_blog::post20200131::{v1, v2};

use test::Bencher;

#[bench]
fn custom_v2_spawn_0_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v2::spawn(Yields(0));
        })
    });
}

#[bench]
fn custom_v2_spawn_10_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v2::spawn(Yields(10));
        })
    });
}

#[bench]
fn custom_v2_spawn_50_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v2::spawn(Yields(50));
        })
    });
}

#[bench]
fn custom_v1_spawn_0_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v1::spawn(Yields(0));
        })
    });
}

#[bench]
fn custom_v1_spawn_10_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v1::spawn(Yields(10));
        })
    });
}

#[bench]
fn custom_v1_spawn_50_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            v1::spawn(Yields(50));
        })
    });
}

#[bench]
fn async_std_spawn_0_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            async_std::task::spawn(Yields(0));
        })
    });
}

#[bench]
fn async_std_spawn_10_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            async_std::task::spawn(Yields(10));
        })
    });
}

#[bench]
fn async_std_spawn_50_yields(b: &mut Bencher) {
    b.iter(|| {
        block_on(async {
            async_std::task::spawn(Yields(50));
        })
    });
}

#[bench]
fn tokio_spawn_0_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || {
        runtime.block_on(async {
            tokio::spawn(Yields(0));
        })
    });
}

#[bench]
fn tokio_spawn_10_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || {
        runtime.block_on(async {
            tokio::spawn(Yields(10));
        })
    });
}

#[bench]
fn tokio_spawn_50_yields(b: &mut Bencher) {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    b.iter(move || {
        runtime.block_on(async {
            tokio::spawn(Yields(50));
        })
    });
}

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
