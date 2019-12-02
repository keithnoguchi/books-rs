// SPDX-License-Identifier: GPL-2.0
use futures::{self, Future};

// https://tokio.rs/docs/futures/combinators/
pub struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();
    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        const NAME: &str = "combinator::HelloWorld";
        eprintln!("[{}] poll()", NAME);
        Ok(futures::Async::Ready(format!("[{}]: hello world", NAME)))
    }
}

#[allow(dead_code)]
pub fn hello() -> impl Future<Item = (), Error = ()> {
    const NAME: &str = "combinator::hello";
    HelloWorld.map(|msg| println!("[{}]: {}", NAME, msg))
}

#[cfg(test)]
mod tests {
    #[test]
    fn map() {
        use super::Future;
        let fut = super::HelloWorld;
        tokio::run(fut.map(|msg| println!("{}", msg)));
    }
}
