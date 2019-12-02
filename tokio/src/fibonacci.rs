// SPDX-License-Identifier: GPL-2.0
use futures::{self, Future};

// https://tokio.rs/docs/futures/streams/
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 1, next: 1 }
    }
}

impl futures::Stream for Fibonacci {
    type Item = u64;
    type Error = ();
    fn poll(&mut self) -> futures::Poll<Option<u64>, ()> {
        let curr = self.curr;
        let next = curr + self.next;
        self.curr = self.next;
        self.next = next;
        Ok(futures::Async::Ready(Some(curr)))
    }
}

pub struct SlowFibonacci {
    interval: tokio::timer::Interval,
    curr: u64,
    next: u64,
}

impl SlowFibonacci {
    #[allow(dead_code)]
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            interval: tokio::timer::Interval::new_interval(duration),
            curr: 1,
            next: 1,
        }
    }
}

impl futures::Stream for SlowFibonacci {
    type Item = u64;
    type Error = ();
    fn poll(&mut self) -> futures::Poll<Option<u64>, ()> {
        futures::try_ready!(self
            .interval
            .poll()
            // ignore error
            .map_err(|_| ()));
        let curr = self.curr;
        let next = curr + self.next;
        self.curr = self.next;
        self.next = next;
        Ok(futures::Async::Ready(Some(curr)))
    }
}

pub struct Display<T> {
    stream: T,
    curr: usize,
    max: usize,
}

impl<T> Display<T> {
    #[allow(dead_code)]
    pub fn new(stream: T, max: usize) -> Self {
        Self {
            stream,
            curr: 0,
            max,
        }
    }
}

impl<T> Future for Display<T>
where
    T: futures::Stream,
    T::Item: std::fmt::Display,
{
    type Item = ();
    type Error = T::Error;
    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        while self.curr < self.max {
            let value = match futures::try_ready!(self.stream.poll()) {
                Some(value) => value,
                None => break,
            };
            println!("value #{} = {}", self.curr, value);
            self.curr += 1;
        }
        Ok(futures::Async::Ready(()))
    }
}

#[allow(dead_code)]
pub fn fibonacci() -> impl futures::Stream<Item = u64, Error = ()> {
    futures::stream::unfold((1, 1), |(curr, next)| {
        let new_next = curr + next;
        Some(Ok((curr, (next, new_next))))
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn into_iterator() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
        }
        let tests = [
            Test {
                name: "1, 2, 3",
                data: vec![1, 2, 3],
            },
            Test {
                name: "1, 2, 3, 4, 5",
                data: vec![1, 2, 3, 4, 5],
            },
        ];
        for t in &tests {
            use futures::Stream;
            let name = t.name;
            let data = t.data.clone();
            tokio::run(futures::stream::iter_ok(data).for_each(move |i| {
                println!("[{}]: {}", name, i);
                Ok(())
            }));
        }
    }
    #[test]
    fn fibonacci() {
        struct Test {
            name: &'static str,
            count: u64,
        }
        let tests = [
            Test {
                name: "10 entries",
                count: 10,
            },
            Test {
                name: "50 entries",
                count: 50,
            },
        ];
        for t in &tests {
            use futures::Stream;
            let name = t.name;
            tokio::run(super::fibonacci().take(t.count).for_each(move |i| {
                println!("[{}]: {}", name, i);
                Ok(())
            }));
        }
    }
    #[test]
    fn display_slow_fibonacci() {
        struct Test {
            name: &'static str,
            delay: u64,
            count: usize,
        }
        let tests = [
            Test {
                name: "10 fibonaccis with 50msec delay",
                delay: 50,
                count: 10,
            },
            Test {
                name: "50 fibonaccis with 1msc delay",
                delay: 1,
                count: 50,
            },
        ];
        for t in &tests {
            let msec = std::time::Duration::from_millis(t.delay);
            let fib = super::SlowFibonacci::new(msec);
            let stream = super::Display::new(fib, t.count);
            println!("{}", t.name);
            tokio::run(stream);
        }
    }
    #[test]
    fn display_fibonacci() {
        struct Test {
            name: &'static str,
            count: usize,
        }
        let tests = [
            Test {
                name: "print out 1 fibonacci number",
                count: 1,
            },
            Test {
                name: "print out 5 fibonacci numbers",
                count: 5,
            },
            Test {
                name: "print out 10 fibonacci numbers",
                count: 10,
            },
        ];
        for t in &tests {
            let fib = super::Fibonacci::new();
            let stream = super::Display::new(fib, t.count);
            println!("{}", t.name);
            tokio::run(stream);
        }
    }
}
