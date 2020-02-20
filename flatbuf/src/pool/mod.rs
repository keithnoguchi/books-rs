//! flatbuffer builder pool
pub mod v1;
pub mod v2;
pub mod v3;
pub use v3::{
    init_min_pool_size, init_max_pool_size, init_buffer_capacity, BuilderPool,
};
