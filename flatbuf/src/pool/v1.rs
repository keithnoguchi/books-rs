//! `parking_log::Mutex<Vec>` based flatbuffer builder pool
use std::ops::{Deref, DerefMut};

use flatbuffers::FlatBufferBuilder;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

/// `FlatBufferBuilder` pool.
///
/// # Examples
///
/// ```
/// use flatbuf_tutorial::pool::v1::BuilderPool;
///
/// let mut b = BuilderPool::get();
/// let name = b.create_string("something fun");
/// b.finish(name, None);
/// ```
pub struct BuilderPool;

static mut INIT_POOL_SIZE: usize = 32;
static mut MAX_POOL_SIZE: usize = 1_024;
static mut BUFFER_CAPACITY: usize = 64;

impl BuilderPool {
    /// Get the `FlatBufferBuilder` from the global pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::BuilderPool;
    ///
    /// let mut b = BuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn get() -> GlobalBuilder {
        let mut pool = POOL.lock();
        match pool.pop() {
            Some(builder) => builder,
            None => GlobalBuilder::new(),
        }
    }

    /// Change the initial global pool size.
    ///
    /// It should be called before calling the first `get`
    /// function otherwise the change won't applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::BuilderPool;
    ///
    /// BuilderPool::init_pool_size(0);
    /// let mut b = BuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn init_pool_size(size: usize) {
        unsafe {
            INIT_POOL_SIZE = size;
            if MAX_POOL_SIZE < size {
                MAX_POOL_SIZE = size;
            }
        }
    }

    /// Change the maximum global pool size.
    ///
    /// It should be called before calling the first `get`
    /// function otherwise the change won't applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::BuilderPool;
    ///
    /// BuilderPool::max_pool_size(4);
    /// let mut b = BuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn max_pool_size(size: usize) {
        unsafe {
            MAX_POOL_SIZE = size;
            if INIT_POOL_SIZE > size {
                INIT_POOL_SIZE = size;
            }
        }
    }

    /// Change the initial `FlatBufferBuilder` buffer size.
    ///
    /// The value only applicable for the newly allocated
    /// `FlatBufferBuilder` instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::BuilderPool;
    ///
    /// BuilderPool::buffer_capacity(64);
    /// let mut b = BuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn buffer_capacity(capacity: usize) {
        unsafe {
            BUFFER_CAPACITY = capacity;
        }
    }
}

/// `GlobalBuilder` encapsulates the `FlatBufferBuilder` instance
/// maintained in the global pool.
pub struct GlobalBuilder(Option<FlatBufferBuilder<'static>>);

impl GlobalBuilder {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn capacity() -> usize {
        unsafe { BUFFER_CAPACITY }
    }
}

impl Default for GlobalBuilder {
    #[inline]
    fn default() -> Self {
        Self(Some(
            FlatBufferBuilder::new_with_capacity(Self::capacity()),
        ))
    }
}

impl Deref for GlobalBuilder {
    type Target = FlatBufferBuilder<'static>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl DerefMut for GlobalBuilder {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}

impl Drop for GlobalBuilder {
    #[inline]
    fn drop(&mut self) {
        if let Some(mut builder) = self.0.take() {
            // resetting the builder outside of the lock
            // to reduce the pool manipulation contention.
            builder.reset();
            let max = unsafe { MAX_POOL_SIZE };
            let mut pool = POOL.lock();
            if pool.len() < max {
                pool.push(GlobalBuilder(Some(builder)))
            }
        }
    }
}

static POOL: Lazy<Mutex<Vec<GlobalBuilder>>> = Lazy::new(|| {
    let init = unsafe { INIT_POOL_SIZE };
    let mut pool = Vec::new();
    for _ in { 0..init } {
        pool.push(GlobalBuilder::new());
    }
    Mutex::new(pool)
});
