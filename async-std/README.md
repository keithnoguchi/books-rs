# async-std

Examples from [the async-std Book].

## Examples

- [Concepts]
  - [Tasks]
    - [cat.rs]: `task::spawn()` example
    - [panic.rs]: `panic!()` propagation example
    - [abort.rs]: `panic!()` to abort example
- [Tutorial]: Writing a chat
  - [Writing an accept loop]
    - [accept-server.rs]: accepter server example
  - [Receiving messages]
    - [receive-server.rs]: receiver server example
  - [Sending messages]
    - [send-server.rs]: sending server example
  - [Connecting Readers and Writers]
    - [broker-server.rs]: broker server example
  - [Clean Shutdown]
    - [shutdown-server.rs]: Server shutdown example
  - [Handling Disconnection]
    - [disconnect-server.rs]: Client disconnection example

[concepts]: https://book.async.rs/concepts.html
[tasks]: https://book.async.rs/concepts/tasks.html
[tutorial]: https://book.async.rs/tutorial/index.html
[writing an accept loop]: https://book.async.rs/tutorial/accept_loop.html
[receiving messages]: https://book.async.rs/tutorial/receiving_messages.html
[sending messages]: https://book.async.rs/tutorial/sending_messages.html
[connecting readers and writers]: https://book.async.rs/tutorial/connecting_readers_and_writers.html
[clean shutdown]: https://book.async.rs/tutorial/clean_shutdown.html
[handling disconnection]: https://book.async.rs/tutorial/handling_disconnection.html

[cat.rs]: examples/ch02-02-cat.rs
[panic.rs]: examples/ch02-02-panic.rs
[abort.rs]: examples/ch02-02-abort.rs
[accept-server.rs]: examples/ch03-02-server.rs
[receive-server.rs]: examples/ch03-03-server.rs
[send-server.rs]: examples/ch03-04-server.rs
[broker-server.rs]: examples/ch03-05-server.rs
[shutdown-server.rs]: examples/ch03-07-server.rs
[disconnect-server.rs]: example/ch03-08-server.rs

### A chat server

Here is the chat server example, as in [examples/chat-server.rs].

```rust
use async_std::task;
use async_std_book::Server;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), Error> {
    let addr = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("[::1]:8000"));
    task::block_on(Server::new(addr).run())
}
```

[examples/chat-server.rs]: examples/chat-server.rs

## References

- [The async-std Book]: async-std book!
  - [Crate async-std]: Async version of the Rust standard library
- [The Async Book]: Asynchronous Programming in Rust
  - [Crate futures]: Abstractions for Asynchronous Programming
- [Original futures design]: Original futures design by [Aaron Turon]

[the async-std book]: https://book.async.rs/
[crate async-std]: https://docs.rs/async-std/latest/
[crate futures]: https://docs.rs/futures/latest/
[the async book]: https://rust-lang.github.io/async-book/
[original futures design]: https://aturon.github.io/blog/2016/09/07/futures-design/
[Aaron Turon]: https://aturon.github.io/blog/

Happy Hacking!
