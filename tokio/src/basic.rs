// SPDX-License-Identifier: GPL-2.0
use futures::{self, Future};

// https://tokio.rs/docs/futures/basic/
pub struct HelloWorld {
    limit: u32,
    count: u32,
}

impl HelloWorld {
    #[allow(dead_code)]
    pub fn new(limit: u32) -> Self {
        Self { limit, count: 0 }
    }
}

impl Future for HelloWorld {
    type Item = String;
    type Error = ();
    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        const NAME: &str = "basic::HelloWorld";
        self.count += 1;
        if self.count < self.limit {
            eprintln!("[{}]: count={}", NAME, self.count);
            Ok(futures::Async::NotReady)
        } else {
            Ok(futures::Async::Ready("hello world".to_string()))
        }
    }
}

pub struct Display<T>(pub T);

impl<T> Future for Display<T>
where
    T: Future,
    T::Item: std::fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> futures::Poll<(), T::Error> {
        const NAME: &str = "basic::Display";
        let value = match self.0.poll() {
            Ok(futures::Async::Ready(value)) => value,
            Ok(futures::Async::NotReady) => {
                eprintln!("[{}]: Async::NotReady", NAME);
                return Ok(futures::Async::NotReady);
            }
            Err(err) => return Err(err),
        };
        println!("[{}]: {}", NAME, value);
        Ok(futures::Async::Ready(()))
    }
}

pub struct BetterDisplay<T>(pub T);

impl<T> Future for BetterDisplay<T>
where
    T: Future,
    T::Item: std::fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> futures::Poll<(), T::Error> {
        const NAME: &str = "basic::BetterDisplay";
        let value = futures::try_ready!(self.0.poll());
        println!("[{}]: {}", NAME, value);
        Ok(futures::Async::Ready(()))
    }
}

#[allow(dead_code)]
pub fn display(count: u32) -> impl Future<Item = (), Error = ()> {
    Display(HelloWorld::new(count))
}

#[allow(dead_code)]
pub fn better_display(count: u32) -> impl Future<Item = (), Error = ()> {
    BetterDisplay(HelloWorld::new(count))
}

#[cfg(test)]
mod tests {
    use tokio;
    #[test]
    fn run_hello_display() {
        let count = 1;
        let fut = super::Display(super::HelloWorld::new(count));
        tokio::run(fut);
    }
    #[test]
    fn run_hello_better_display() {
        let count = 1;
        let fut = super::BetterDisplay(super::HelloWorld::new(count));
        tokio::run(fut);
    }
}
