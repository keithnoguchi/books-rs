//! Global pool example
use flatbuf_tutorial::{FlatBufferBuilderPool, Monster};

const INIT_POOL_SIZE: usize = 4;
const MAX_POOL_SIZE: usize = 64;
const BUFFER_CAPACITY: usize = 64;

fn main() {
    let pool = FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    let mut b = pool.get();
    let monster = Monster::create(&mut b, "monster");
    b.finish(monster, None);
}
