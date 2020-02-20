//! flatbuffer builder pool benchmark
#![feature(test)]
extern crate test;

use std::sync::Mutex;

use flatbuf_tutorial::monster::Monster;
use flatbuf_tutorial::pool::{v1, v3};
use flatbuffers::FlatBufferBuilder;

use test::Bencher;

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
        let mut b = &mut *builder.lock().unwrap();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_v1(b: &mut Bencher) {
    b.iter(|| {
        let mut b = v1::BuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_v3(b: &mut Bencher) {
    b.iter(|| {
        let mut b = v3::BuilderPool::get();
        let monster = Monster::create(&mut b, "monster");
        b.finish(monster, None);
    });
}
