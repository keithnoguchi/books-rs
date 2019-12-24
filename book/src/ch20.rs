//! Chapter 20 Final Project: Building a Multithreaded Web Server

/// fixed sized worker queue.
pub struct WorkQueue {
    size: usize,
    workers: Vec<Worker>,
}

impl WorkQueue {
    /// new creates size number of workers.
    ///
    /// # Panics
    ///
    /// It will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size != 0);
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker {});
        }
        Self { size, workers }
    }
    /// execute executes the closure on one of the workers.
    pub fn exec<F>(&self, f: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce() + Send + 'static,
    {
        f();
        Ok(())
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
        });
    }
}
