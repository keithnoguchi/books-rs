//! `parking_log::Mutex<Vec>` based flatbuffer builder pool
use std::{
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Weak},
};

use flatbuffers::FlatBufferBuilder;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

/// `FlatBufferBuilder` pool.
///
/// # Examples
///
/// ```
/// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
///
/// let mut b = FlatBufferBuilderPool::get();
/// let name = b.create_string("something fun");
/// b.finish(name, None);
/// ```
pub struct FlatBufferBuilderPool {
    /// Initial local pool size.
    init: usize,

    /// Maximum local pool size.
    max: usize,

    /// Flatbuffer buffer capacity of the local pool buffer.
    buffer_capacity: usize,
}

static mut INIT_POOL_SIZE: usize = 32;
static mut MAX_POOL_SIZE: usize = 1_024;
static mut BUFFER_CAPACITY: usize = 64;

impl FlatBufferBuilderPool {
    /// Get the `FlatBufferBuilder` from the global pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// let mut b = FlatBufferBuilderPool::get();
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
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// FlatBufferBuilderPool::init_global_pool_size(0);
    /// let mut b = FlatBufferBuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn init_global_pool_size(size: usize) {
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
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// FlatBufferBuilderPool::max_global_pool_size(4);
    /// let mut b = FlatBufferBuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn max_global_pool_size(size: usize) {
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
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// FlatBufferBuilderPool::global_buffer_capacity(64);
    /// let mut b = FlatBufferBuilderPool::get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn global_buffer_capacity(capacity: usize) {
        unsafe {
            BUFFER_CAPACITY = capacity;
        }
    }
}

/// `GlobalBuilder` encapsulates the `FlatBufferBuilder` instance
/// for the global pool.
pub struct GlobalBuilder(Option<FlatBufferBuilder<'static>>);

impl GlobalBuilder {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn buffer_capacity() -> usize {
        unsafe { BUFFER_CAPACITY }
    }
}

impl Default for GlobalBuilder {
    #[inline]
    fn default() -> Self {
        Self(Some(FlatBufferBuilder::new_with_capacity(
            Self::buffer_capacity(),
        )))
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
    let (init, max) = unsafe { (INIT_POOL_SIZE, MAX_POOL_SIZE) };
    let mut pool = Vec::with_capacity(max);
    for _ in 0..init {
        pool.push(GlobalBuilder::new());
    }
    Mutex::new(pool)
});

impl FlatBufferBuilderPool {
    /// Create a local `FlatBufferBuilder` pool instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let mut pool = FlatBufferBuilderPool::new().build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Change the initial local pool size.
    ///
    /// It should be called before calling the first `get`
    /// function otherwise the change won't applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let pool = FlatBufferBuilderPool::new()
    ///     .init_pool_size(0)
    ///     .build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn init_pool_size(mut self, size: usize) -> Self {
        self.init = size;
        if self.max < size {
            self.max = size;
        }
        self
    }

    /// Change the maximum local pool size.
    ///
    /// It should be called before calling the first `get`
    /// function otherwise the change won't applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let pool = FlatBufferBuilderPool::new()
    ///     .max_pool_size(4)
    ///     .build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn max_pool_size(mut self, size: usize) -> Self {
        self.max = size;
        if self.init > size {
            self.init = size;
        }
        self
    }

    /// Change the initial `FlatBufferBuilder` buffer size.
    ///
    /// The value only applicable for the newly allocated
    /// `FlatBufferBuilder` instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let pool = FlatBufferBuilderPool::new()
    ///     .buffer_capacity(64)
    ///     .build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn buffer_capacity(mut self, capacity: usize) -> Self {
        self.buffer_capacity = capacity;
        self
    }

    /// Build a local `FlatBufferBuilder` pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let pool = FlatBufferBuilderPool::new()
    ///     .build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    pub fn build<'a>(&self) -> FlatBufferBuilderLocalPool<'a> {
        let inner = Arc::new(Mutex::new(Vec::with_capacity(self.max)));
        for _ in 0..self.init {
            let builder = LocalBuilder::new(
                Arc::downgrade(&inner),
                self.max,
                FlatBufferBuilder::new_with_capacity(self.buffer_capacity),
            );
            inner.lock().push(builder);
        }
        FlatBufferBuilderLocalPool::<'a> {
            max: self.max,
            buffer_capacity: self.buffer_capacity,
            inner,
        }
    }
}

const LOCAL_INIT_POOL_SIZE: usize = 32;
const LOCAL_MAX_POOL_SIZE: usize = 1_024;
const LOCAL_BUFFER_CAPACITY: usize = 64;

impl Default for FlatBufferBuilderPool {
    fn default() -> Self {
        Self {
            init: LOCAL_INIT_POOL_SIZE,
            max: LOCAL_MAX_POOL_SIZE,
            buffer_capacity: LOCAL_BUFFER_CAPACITY,
        }
    }
}

/// Local `FlatBufferBuilder` pool.
///
/// # Examples
///
/// ```
/// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
///
/// // Get the builder from the global pool.
/// let pool = FlatBufferBuilderPool::new().build();
/// let mut b = pool.get();
/// let name = b.create_string("something fun");
/// b.finish(name, None);
/// ```
pub struct FlatBufferBuilderLocalPool<'a> {
    /// Maximum local pool size.
    max: usize,

    /// Flatbuffer buffer capacity for the local pool.
    buffer_capacity: usize,

    /// Local pool.
    inner: Arc<Mutex<Vec<LocalBuilder<'a>>>>,
}

impl<'a> FlatBufferBuilderLocalPool<'a> {
    /// Get the `FlatBufferBuilder` from the local pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use flatbuf_tutorial::pool::v1::FlatBufferBuilderPool;
    ///
    /// // Get the builder from the local pool.
    /// let pool = FlatBufferBuilderPool::new().build();
    /// let mut b = pool.get();
    /// let name = b.create_string("something fun");
    /// b.finish(name, None);
    /// ```
    #[inline]
    pub fn get(&self) -> LocalBuilder<'a> {
        let mut pool = self.inner.lock();
        match pool.pop() {
            Some(builder) => builder,
            None => LocalBuilder::new(
                Arc::downgrade(&self.inner),
                self.max,
                FlatBufferBuilder::new_with_capacity(self.buffer_capacity),
            ),
        }
    }
}

impl<'a> Drop for FlatBufferBuilderLocalPool<'a> {
    fn drop(&mut self) {
        let mut pool = self.inner.lock();
        while let Some(mut builder) = pool.pop() {
            builder.drain();
        }
    }
}

/// `LocalBuilder` encapsulates the `FlatBufferBuilder` instance
/// for the local pool.
pub struct LocalBuilder<'a> {
    /// Local pool.
    pool: Weak<Mutex<Vec<LocalBuilder<'a>>>>,

    /// Maximum local pool size.
    max: usize,

    /// Drained state.
    drained: AtomicBool,

    /// Actual builder.
    inner: Option<FlatBufferBuilder<'a>>,
}

impl<'a> LocalBuilder<'a> {
    fn new(pool: Weak<Mutex<Vec<Self>>>, max: usize, builder: FlatBufferBuilder<'a>) -> Self {
        Self {
            pool,
            max,
            drained: AtomicBool::new(false),
            inner: Some(builder),
        }
    }
    #[inline]
    fn drain(&mut self) {
        self.drained.store(true, Ordering::SeqCst);
    }
    #[inline]
    fn is_drained(&self) -> bool {
        self.drained.load(Ordering::SeqCst)
    }
}

impl<'a> Deref for LocalBuilder<'a> {
    type Target = FlatBufferBuilder<'a>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<'a> DerefMut for LocalBuilder<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

impl<'a> Drop for LocalBuilder<'a> {
    fn drop(&mut self) {
        if self.is_drained() {
            return;
        }
        if let Some(mut builder) = self.inner.take() {
            builder.reset();
            if let Some(pool) = &self.pool.upgrade() {
                let mut pool = pool.lock();
                if pool.len() < self.max {
                    pool.push(LocalBuilder::new(self.pool.clone(), self.max, builder));
                }
            }
        }
    }
}
