//! Chapter 20 Final Project: Building a Multithreaded Web Server
pub struct ThreadPool {
    _workers: Vec<Worker>,
}

struct Worker;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self {
            _workers: Vec::with_capacity(size),
        }
    }
}
