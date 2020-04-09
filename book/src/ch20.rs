//! Building a [Multithreaded] Web Server
//!
//! [multithreaded]: https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html
use std::{
    error::Error,
    fmt::{self, Debug},
    io,
    marker::PhantomData,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};
use tracing::{debug, error, info, instrument};

pub struct ThreadPool<T = ()>
where
    T: Send + 'static,
{
    tx: Option<Sender<Job<T>>>,
    workers: Vec<Worker<T>>,
}

impl<T> ThreadPool<T>
where
    T: Send + 'static,
{
    /// # Panics
    ///
    /// Function `new` will panic if the `size` argument is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        Self {
            workers,
            tx: Some(tx),
        }
    }
    pub fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where
        F: FnOnce() -> io::Result<T> + Send + 'static,
    {
        match &self.tx {
            Some(tx) => tx.send(Box::new(f)).map_err(|err| err.into()),
            None => Err("channel closed".into()),
        }
    }
}

impl<T> Debug for ThreadPool<T>
where
    T: Send + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadPool workers={}", self.workers.len())
    }
}

impl<T> Drop for ThreadPool<T>
where
    T: Send + 'static,
{
    #[instrument]
    fn drop(&mut self) {
        // drop the channel to signal workers to complete the task.
        if let Some(tx) = self.tx.take() {
            drop(tx)
        }
        while let Some(mut worker) = self.workers.pop() {
            debug!("cleanup {:?}", worker);
            if let Some(err) = worker.handle.take().and_then(|handle| handle.join().err()) {
                error!("worker error: {:?}", err);
            }
        }
    }
}

struct Worker<T = ()>
where
    T: Send + 'static,
{
    id: usize,
    handle: Option<JoinHandle<Result<(), Box<dyn Error + Send + Sync + 'static>>>>,
    phantom: PhantomData<T>,
}

impl<T: Send + 'static> Worker<T> {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job<T>>>>) -> Self {
        let handle = thread::spawn(|| Self::run(rx));
        Self {
            id,
            handle: Some(handle),
            phantom: PhantomData,
        }
    }
    #[instrument]
    fn run(rx: Arc<Mutex<Receiver<Job<T>>>>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        loop {
            let job = match rx.lock() {
                Err(err) => return Err(format!("lock error: {}", err).into()),
                Ok(job) => job,
            }
            .recv()?;
            match job() {
                Err(err) => error!("work error: {}", err),
                Ok(_) => debug!("work done"),
            }
        }
    }
}

impl<T: Send + 'static> Debug for Worker<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Worker{}", self.id)
    }
}

impl<T: Send + 'static> Drop for Worker<T> {
    #[instrument]
    fn drop(&mut self) {
        info!("{:?} dropped", self);
    }
}

type Job<T> = Box<dyn FnOnce() -> io::Result<T> + Send + 'static>;
