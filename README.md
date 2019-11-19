# rustbox

[Rust lang] sandbox with the [table driven] unit tests.

[![DroneCI]](https://cloud.drone.io/keithnoguchi/rustbox)
[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/rustbox)

[Rust lang]: https://www.rust-lang.org
[table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
[DroneCI]: https://cloud.drone.io/api/badges/keithnoguchi/rustbox/status.svg
[CircleCI]: https://circleci.com/gh/keithnoguchi/rustbox.svg?style=svg

# Modules

- [array]: Standard array type example
- [vector]: Standard vector type example
- [string]: String type example
- [hashmap]: HashMap type example
- [generic]: Generics example
- [ptr]: [Primitive Type pointer] and [Module std::ptr] examples
- [boxed]: [Module alloc::boxed] example
- [borrow]: [Enum alloc::borrow::Cow], [Trait alloc::borrow::Borrow] examples
- [futures]: [Crate futures] example
- [stream]: [Module futures::stream] example
- [timer]: [Build a Timer] example
- [exec]: [Build an Executor] example
- [first]: [A Bad Singly-Linked Stack]
- [second]: [An Ok Singly-Linked Stack]

[array]: src/array.rs
[vector]: src/vector.rs
[string]: src/string.rs
[hashmap]: src/hashmap.rs
[generic]: src/generic.rs
[ptr]: src/ptr.rs
[boxed]: src/boxed.rs
[borrow]: src/borrow.rs
[futures]: src/futures.rs
[stream]: src/stream.rs
[timer]: src/timer.rs
[exec]: src/exec.rs
[first]: src/first.rs
[second]: src/second.rs
[A Bad Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/first.html
[An Ok Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/second.html

# Test

```sh
$ make test
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running target/debug/deps/rustbox-9e9575d91e3bd718

running 8 tests
test array::tests::largest ... ok
test vector::tests::get ... ok
test array::tests::slice ... ok
test array::tests::next ... ok
test vector::tests::next ... ok
test vector::tests::len ... ok
test vector::tests::pop ... ok
test vector::tests::push ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests rustbox

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$
```

# References

- [The book]: The Rust Programming Language
- [What's unique about Rust?]: RustLatam 2019 Opening Keynote by [Niko Matsakis]
- [Crate alloc]: The Rust core allocation and collections library
  - [Module alloc::boxed]: A pointer type for heap allocation
  - [Enum alloc::borrow::Cow]: A clone-on-write smart pointer
  - [Trait alloc::borrow::Borrow]: A trait for borrowing data
  - [Trait alloc::borrow::BorrowMut]: A trait for mutably borrowing data
  - [Trait alloc::borrow::ToOwned]: A generalization of Clone to borrowed data
- [Too Many Linked Lists]: Learning Rust with Entirely Too Many Linked List
- [The async book]: Asynchronous Programming in Rust
- [Zero-Cost Async IO]: RustLatam 2019 talk by [Without Boats]
- [Crate std]: The Rust Standard Library
  - [Module std::ptr]: Manually manage memory through raw pointers
  - [Module std::future]: Asynchronous values
  - [Module std::task]: Types and Traits for working with asynchronous tasks
- [Crate futures]: Zero-cost asynchronous programming in Rust
  - [Crate futures tests]: futures tests
  - [Module futures::stream]: Asynchronous streams
  - [Module futures::compat]: Interop between futures 0.1 and 0.3
- [Crate Metal I/O]: A lightweight I/O library for Rust

[The book]: https://doc.rust-lang.org/stable/book/
[What's unique about Rust?]: https://www.youtube.com/watch?v=jQOZX0xkrWA
[Crate alloc]: https://doc.rust-lang.org/alloc/index.html
[Module alloc::boxed]: https://doc.rust-lang.org/alloc/boxed/index.html
[Enum alloc::borrow::Cow]: https://doc.rust-lang.org/alloc/borrow/enum.Cow.html
[Trait alloc::borrow::Borrow]: https://doc.rust-lang.org/alloc/borrow/trait.Borrow.html
[Trait alloc::borrow::BorrowMut]: https://doc.rust-lang.org/alloc/borrow/trait.BorrowMut.html
[Trait alloc::borrow::ToOwned]: https://doc.rust-lang.org/alloc/borrow/trait.ToOwned.html
[Primitive Type pointer]: https://doc.rust-lang.org/std/primitive.pointer.html
[Module std::ptr]: https://doc.rust-lang.org/std/ptr/index.html
[Too Many Linked Lists]: http://rust-unofficial.github.io/too-many-lists/
[The async book]: https://rust-lang.github.io/async-book/
[Build a Timer]:  https://rust-lang.github.io/async-book/02_execution/03_wakeups.html#applied-build-a-timer
[Build an Executor]: https://rust-lang.github.io/async-book/02_execution/04_executor.html
[Zero-Cost Async IO]: https://www.youtube.com/watch?v=skos4B5x7qE
[Crate std]: https://doc.rust-lang.org/std/index.html
[Module std::future]: https://doc.rust-lang.org/std/future/index.html
[Module std::task]: https://doc.rust-lang.org/std/task/index.html
[Crate futures]: https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/index.html
[Crate futures tests]: https://github.com/rust-lang-nursery/futures-rs/tree/master/futures/tests
[Module futures::stream]: https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/stream/index.html
[Module futures::compat]: https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/compat/index.html
[Niko Matsakis]: https://twitter.com/nikomatsakis
[Without Boats]: https://github.com/withoutboats
[Crate Metal I/O]: https://github.com/tokio-rs/mio
[Crate crossbeam]: https://docs.rs/crossbeam/0.7.3/crossbeam/
[Crate parking_lot]: https://docs.rs/parking_lot/0.1.0/parking_lot/
[Crate diesel]: https://diesel.rs/guides/getting-started/

Happy Hacking!
