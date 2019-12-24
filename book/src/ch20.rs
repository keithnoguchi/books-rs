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
    /// let _wq = WorkQueue::new(2);
    /// ```
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker {});
        }
        Self { size, workers }
    }
    /// `exec` executes the closure on one of the workers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use the_book::ch20::WorkQueue;
    ///
    /// let wq = WorkQueue::new(10);
    /// for _ in 0..10 {
    ///     wq.exec(|| {
    ///         println!("Hello WorkQueue");
    ///         Ok(())
    ///     });
    /// }
    /// ```
    pub fn exec<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        f()
    }
}

struct Worker;

#[cfg(test)]
mod tests {
    use super::WorkQueue;
    #[test]
    fn new() {
        let wq = WorkQueue::new(1);
        assert_eq!(1, wq.size);
    }
    fn exec() {
        let wq = WorkQueue::new(1);
        wq.exec(|| {
            println!("hello");
            Ok(())
        });
    }
}
