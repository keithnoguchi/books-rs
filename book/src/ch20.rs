//! Multithreaded [Web Server]
//!
//! [web server]: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
use std::{
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};
use tracing::{error, info, instrument, span, warn, Level};

pub struct ThreadPool<T: Send + 'static> {
    counter: AtomicUsize,
    max: usize,
    tx: Sender<Job<T>>,
    workers: Vec<Worker>,
}

impl<T: Send + 'static> fmt::Debug for ThreadPool<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size={}", self.workers.len())
    }
}

impl<T: Send + 'static> Drop for ThreadPool<T> {
    #[instrument]
    fn drop(&mut self) {
        while let Some(worker) = self.workers.pop() {
            info!("drop worker{:?}", worker);
        }
        info!("ThreadPool dropped");
    }
}

impl<T: Send + 'static> ThreadPool<T> {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    #[instrument]
    pub fn new(size: usize, max: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for id in { 0..size } {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        info!(size = size, workers = workers.len());
        Self {
            counter: AtomicUsize::new(0),
            max,
            tx,
            workers,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let span = span!(Level::INFO, "execute");
        let _guard = span.enter();
        let count = self.counter.fetch_add(1, Ordering::SeqCst);
        if count < self.max {
            if let Err(err) = self.tx.send(Box::new(f)) {
                warn!("{:?}", err);
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    handle: thread::JoinHandle<()>,
}

impl Drop for Worker {
    #[instrument]
    fn drop(&mut self) {
        info!("worker dropped");
    }
}

impl Worker {
    fn new<T>(id: usize, rx: Arc<Mutex<Receiver<Job<T>>>>) -> Self
    where
        T: Send + 'static,
    {
        let handle = thread::spawn(move || Self::worker(id, rx));
        Self { handle }
    }
    #[instrument]
    fn worker<T>(id: usize, rx: Arc<Mutex<Receiver<Job<T>>>>)
    where
        T: Send + 'static,
    {
        loop {
            let job = match rx.lock().unwrap().recv() {
                Err(err) => {
                    error!("receive error: {}", err);
                    continue;
                }
                Ok(job) => job,
            };
            job();
        }
    }
}

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;
