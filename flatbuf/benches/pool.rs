//! flatbuffer builder pool benchmark
//!
//! # Examples
//!
//! ```sh
//! $ cargo bench --bench pool -j 2
//! Finished bench [optimized] target(s) in 0.02s
//! Running /home/kei/git/books-rs/target/release/deps/pool-afa691c94dbb07fb
//!
//! running 5 tests
//! test pool_monster_dynamic ... bench:       5,762 ns/iter (+/- 1,009)
//! test pool_monster_mutex   ... bench:       5,389 ns/iter (+/- 1,714)
//! test pool_monster_v1      ... bench:       5,384 ns/iter (+/- 1,285)
//! test pool_monster_v2      ... bench:       5,419 ns/iter (+/- 1,043)
//! test pool_monster_v3      ... bench:       5,328 ns/iter (+/- 1,581)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured; 0 filtered out
//! ```
#![feature(test)]
extern crate test;

use flatbuf_tutorial::monster::Monster;
use flatbuf_tutorial::pool::{v1, v2, v3};
use flatbuffers::FlatBufferBuilder;
use parking_lot::Mutex;

use test::Bencher;

const MIN_POOL_SIZE: usize = 4;
const MAX_POOL_SIZE: usize = 64;
const BUFFER_CAPACITY: usize = 64;

#[bench]
fn pool_monster_dynamic(b: &mut Bencher) {
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
fn pool_monster_v1(b: &mut Bencher) {
    v1::init_min_pool_size(MIN_POOL_SIZE);
    v1::init_max_pool_size(MAX_POOL_SIZE);
    v1::init_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v1::BuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_v2(b: &mut Bencher) {
    v2::init_min_pool_size(MIN_POOL_SIZE);
    v2::init_max_pool_size(MAX_POOL_SIZE);
    v2::init_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v2::BuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_v3(b: &mut Bencher) {
    v3::init_min_pool_size(MIN_POOL_SIZE);
    v3::init_max_pool_size(MAX_POOL_SIZE);
    v3::init_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v3::BuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}
