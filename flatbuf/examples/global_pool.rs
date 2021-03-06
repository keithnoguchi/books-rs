//! Global pool example
use flatbuf_tutorial::{FlatBufferBuilderPool, Monster};

const INIT_POOL_SIZE: usize = 4;
const MAX_POOL_SIZE: usize = 64;
const BUFFER_CAPACITY: usize = 64;

fn main() {
    FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);

    let mut b = FlatBufferBuilderPool::get();
    let monster = Monster::create(&mut b, "monster");
    b.finish(monster, None);
}
