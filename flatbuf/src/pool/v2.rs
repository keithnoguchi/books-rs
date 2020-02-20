//! crossbeam_queue::SegQueue based flatbuffer builder pool
use std::ops::{Deref, DerefMut};

use crossbeam_queue::SegQueue;
use flatbuffers::FlatBufferBuilder;
use once_cell::sync::Lazy;

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
    #[inline]
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
    #[inline]
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self(Some(FlatBufferBuilder::new_with_capacity(buffer_capacity())))
    }
}

impl Deref for Builder {
    type Target = FlatBufferBuilder<'static>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for Builder {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for Builder {
    #[inline]
    fn drop(&mut self) {
        if let Some(mut builder) = self.0.take() {
            if POOL.len() < max_pool_size() {
                builder.reset();
                POOL.push(Builder(Some(builder)));
            }
        }
    }
}

static mut MIN_POOL_SIZE: usize = 32;
static mut MAX_POOL_SIZE: usize = 1_024;
static mut BUFFER_CAPACITY: usize = 64;

#[inline]
pub fn init_min_pool_size(size: usize) {
    unsafe {
        MIN_POOL_SIZE = size;
        if MAX_POOL_SIZE < MIN_POOL_SIZE {
            MAX_POOL_SIZE = MIN_POOL_SIZE;
        }
    }
}

#[inline]
pub fn init_max_pool_size(size: usize) {
    unsafe {
        MAX_POOL_SIZE = size;
        if MIN_POOL_SIZE > MAX_POOL_SIZE {
            MIN_POOL_SIZE = MAX_POOL_SIZE;
        }
    }
}

#[inline]
pub fn init_buffer_capacity(capacity: usize) {
    unsafe {
        BUFFER_CAPACITY = capacity;
    }
}

#[inline]
fn min_pool_size() -> usize {
    unsafe {
        MIN_POOL_SIZE
    }
}

#[inline]
fn max_pool_size() -> usize {
    unsafe {
        MAX_POOL_SIZE
    }
}

#[inline]
fn buffer_capacity() -> usize {
    unsafe {
        BUFFER_CAPACITY
    }
}

static POOL: Lazy<SegQueue<Builder>> = Lazy::new(|| {
    let pool = SegQueue::new();
    for _ in { 0..min_pool_size() } {
        pool.push(Builder::new());
    }
    pool
});
