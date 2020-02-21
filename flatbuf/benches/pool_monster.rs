//! flatbuffer builder pool benchmark
//!
//! # Examples
//!
//! ```sh
//! $ c bench --bench pool_monster
//! Finished bench [optimized] target(s) in 0.03s
//!
//! running 8 tests
//! test pool_monster_global_v1 ... bench:       5,436 ns/iter (+/- 1,601)
//! test pool_monster_global_v2 ... bench:       5,499 ns/iter (+/- 1,624)
//! test pool_monster_global_v3 ... bench:       5,395 ns/iter (+/- 1,677)
//! test pool_monster_local_v1  ... bench:       5,417 ns/iter (+/- 2,345)
//! test pool_monster_local_v2  ... bench:       5,473 ns/iter (+/- 1,209)
//! test pool_monster_local_v3  ... bench:       5,426 ns/iter (+/- 276)
//! test pool_monster_mutex     ... bench:       5,379 ns/iter (+/- 1,560)
//! test pool_monster_stack     ... bench:       5,638 ns/iter (+/- 377)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out
//! ```
#![feature(test)]
extern crate test;

use test::Bencher;

use flatbuf_tutorial::pool::{v1, v2, v3};
use flatbuf_tutorial::Monster;
use flatbuffers::FlatBufferBuilder;
use parking_lot::Mutex;

const INIT_POOL_SIZE: usize = 4_096;
const MAX_POOL_SIZE: usize = 8_192;
const BUFFER_CAPACITY: usize = 1_024;

#[bench]
fn pool_monster_stack(b: &mut Bencher) {
    b.iter(|| {
        let mut b = FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY);
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_mutex(b: &mut Bencher) {
    let builder = Mutex::new(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY));
    b.iter(|| {
        let mut b = &mut *builder.lock();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v1(b: &mut Bencher) {
    v1::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v1::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v1::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v1::FlatBufferBuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v2(b: &mut Bencher) {
    v2::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v2::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v2::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v2::FlatBufferBuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v3(b: &mut Bencher) {
    v3::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v3::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v3::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v3::FlatBufferBuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v1(b: &mut Bencher) {
    let pool = v1::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v2(b: &mut Bencher) {
    let pool = v2::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v3(b: &mut Bencher) {
    let pool = v3::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}
