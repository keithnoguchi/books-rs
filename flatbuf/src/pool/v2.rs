//! crossbeam_queue::SegQueue based flatbuffer builder pool
use std::ops::{Deref, DerefMut};

use crossbeam_queue::SegQueue;
use flatbuffers::FlatBufferBuilder;
use once_cell::sync::Lazy;

const INIT_POOL_SIZE: usize = 32;
const MAX_POOL_SIZE: usize = 1_024;
const BUFFER_CAPACITY: usize = 64;

/// `FlatBufferBuilder` pool.
///
/// # Examples
///
/// ```
/// use flatbuf_tutorial::pool::v2::BuilderPool;
///
/// let mut b = BuilderPool::get();
/// let name = b.create_string("something fun");
/// b.finish(name, None);
/// ```
pub struct BuilderPool;

impl BuilderPool {
    /// get() returns the pre-allocated [`Builder`] from
    /// the pool, or returns the newly allocated one.
    ///
    /// [`builder`]: struct.Builder.html
    pub fn get() -> Builder {
        match POOL.pop() {
            Ok(builder) => builder,
            Err(_) => Builder::new(),
        }
    }
}

/// `Builder` encapsulates the `FlatBufferBuilder` instance.
pub struct Builder(Option<FlatBufferBuilder<'static>>);

impl Builder {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self(Some(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY)))
    }
}

impl Deref for Builder {
    type Target = FlatBufferBuilder<'static>;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        if let Some(mut builder) = self.0.take() {
            // resetting the builder outside of the lock
            // to reduce the pool manipulation contention.
            builder.reset();
            if POOL.len() < MAX_POOL_SIZE {
                POOL.push(Builder(Some(builder)));
            }
        }
    }
}

static POOL: Lazy<SegQueue<Builder>> = Lazy::new(|| {
    let pool = SegQueue::new();
    for _ in { 0..INIT_POOL_SIZE } {
        pool.push(Builder::new());
    }
    pool
});
