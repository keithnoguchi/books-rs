// SPDX-License-Identifier: GPL-2.0
// https://rust-lang.github.io/async-book/print.html#under-the-hood-executing-futures-and-tasks
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

#[allow(dead_code)]
enum Poll<T> {
    Ready(T),
    Pending,
}

#[allow(dead_code)]
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            Poll::Ready(self.socket.read_buf())
        } else {
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
struct Socket {}

impl Socket {
    fn has_data_to_read(&self) -> bool {
        false
    }
    fn read_buf(&self) -> Vec<u8> {
        Vec::<u8>::new()
    }
    fn set_readable_callback(&self, _wake: fn()) {}
}

#[allow(dead_code)]
pub struct Join<A, B> {
    a: Option<A>,
    b: Option<B>,
}

impl<A, B> SimpleFuture for Join<A, B>
where
    A: SimpleFuture<Output = ()>,
    B: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }
        }
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }
        if self.a.is_none() && self.b.is_none() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
pub struct AndThenFut<A, B> {
    first: Option<A>,
    second: B,
}

impl<A, B> SimpleFuture for AndThenFut<A, B>
where
    A: SimpleFuture<Output = ()>,
    B: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                Poll::Ready(()) => self.first.take(),
                Poll::Pending => return Poll::Pending,
            };
        }
        self.second.poll(wake)
    }
}
