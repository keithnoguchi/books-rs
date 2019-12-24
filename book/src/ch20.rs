//! Chapter 20 Final Project: Building a Multithreaded Web Server

/// fixed sized worker queue.
pub struct WorkQueue {
    size: usize,
    workers: Vec<Worker>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl WorkQueue {
    /// `new` creates the `size` number of workers' queue.
    ///
    /// # Panics
    ///
    /// It will panic if the size is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch20::WorkQueue;
    ///
    /// let wq = WorkQueue::new(2);
    /// assert_eq!(2, wq.size());
    /// ```
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker { id });
        }
        Self { size, workers }
    }
    /// `size` returns the size of the work queue.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch20::WorkQueue;
    /// let tests = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    /// for t in &tests {
    ///     let wq = WorkQueue::new(*t);
    ///     assert_eq!(*t, wq.size());
    /// }
    /// ```
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
    /// `exec` executes the closure on one of the workers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch20::WorkQueue;
    ///
    /// let wq = WorkQueue::new(10);
    /// for _ in 0..wq.size() {
    ///     wq.exec(|| {
    ///         println!("Hello WorkQueue");
    ///         Ok(())
    ///     }).unwrap();
    /// }
    /// let wq = WorkQueue::new(5);
    /// for id in 0..wq.size() {
    ///     wq.exec(move || {
    ///         println!("It returns values");
    ///         Ok(id)
    ///     }).unwrap();
    /// }
    /// ```
    pub fn exec<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        f()
    }
}

impl Drop for WorkQueue {
    fn drop(&mut self) {
        for w in &self.workers {
            println!("dropping {:?}", w);
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
}

#[cfg(test)]
mod tests {
    use super::WorkQueue;
    #[test]
    fn new() {
        let wq = WorkQueue::new(1);
        assert_eq!(1, wq.size);
    }
    #[test]
    fn size() {
        let tests = [1, 2, 3, 4, 5, 6, 7];
        for t in &tests {
            let wq = WorkQueue::new(*t);
            assert_eq!(*t, wq.size());
        }
    }
    #[test]
    fn exec() {
        let wq = WorkQueue::new(1);
        wq.exec(|| {
            println!("hello");
            Ok(())
        })
        .unwrap();
        let wq = WorkQueue::new(1);
        wq.exec(|| {
            println!("returns u32");
            Ok(1u32)
        })
        .unwrap();
    }
}
