//! Global pool example
use flatbuf_tutorial::{BuilderPool, Monster};

const INIT_POOL_SIZE: usize = 4;
const MAX_POOL_SIZE: usize = 64;
const BUFFER_CAPACITY: usize = 64;

fn main() {
    BuilderPool::init_pool_size(INIT_POOL_SIZE);
    BuilderPool::max_pool_size(MAX_POOL_SIZE);
    BuilderPool::buffer_capacity(BUFFER_CAPACITY);

    let mut b = BuilderPool::get();
    let monster = Monster::create(&mut b, "monster");
    b.finish(monster, None);
}
