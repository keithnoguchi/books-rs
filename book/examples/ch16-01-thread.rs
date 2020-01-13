//! [Fearess Concurrency] examples
//!
//! [fearess concurrency]: https://doc.rust-lang.org/book/ch16-01-threads.html
use std::{any::Any, fmt::Debug, thread};

fn main() -> Result<(), Box<dyn Any + Send + 'static>> {
    let v = vec![1, 2, 3];
    let t = thread::spawn(|| {
        print_vec(v); // infers the move
    });
    t.join()
}

fn print_vec<T: Debug>(v: Vec<T>) {
    println!("{:?}", v);
}
