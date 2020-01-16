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

[concepts] https://book.async.rs/concepts.html
[tasks]: https://book.async.rs/concepts/tasks.html
[tutorial]: https://book.async.rs/tutorial/index.html
[writing an accept loop]: https://book.async.rs/tutorial/accept_loop.html
[receiving messages]: https://book.async.rs/tutorial/receiving_messages.html

[cat.rs]: examples/ch02-02-cat.rs
[panic.rs]: examples/ch02-02-panic.rs
[abort.rs]: examples/ch02-02-abort.rs
[accept-server.rs]: examples/ch03-02-server.rs
[receive-server.rs]: examples/ch03-03-server.rs

## References

- [The async-std Book]: async-std book!
  - [Crate async-std]: Async version of the Rust standard library
  - [Crate futures v0.3]: Abstructions for Asynchronous Programming
- [The Async Book]: Asynchronous Programming in Rust
- [Original futures design]: Original futures design by [Aaron Turon]

[async-std book]: https://book.async.rs/
[crate async-std]: https://docs.rs/async-std/latest/
[crate futures]: https://docs.rs/futures/latest/
[the async book]: https://rust-lang.github.io/async-book/
[original futures design]: https://aturon.github.io/blog/2016/09/07/futures-design/
[Aaron Turon]: https://aturon.github.io/blog/

Happy Hacking!
