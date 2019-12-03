# rustbook

Rust examples from multiple [Rust lang] books with the [table driven]
unit tests.

[![DroneCI]](https://cloud.drone.io/keithnoguchi/rustbook)
[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/rustbook)

[Rust lang]: https://www.rust-lang.org
[table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
[DroneCI]: https://cloud.drone.io/api/badges/keithnoguchi/rustbook/status.svg
[CircleCI]: https://circleci.com/gh/keithnoguchi/rustbook.svg?style=svg

# Crates

## the-book crate

[the-book] crate presents the examples provided in [the Book], the Rust Programming
Language, with the [table driven] unit tests.  Here are the modules in this
crate:

- [array]: Standard array type example
- [vector]: Standard vector type example
- [string]: String type example
- [hashmap]: HashMap type example
- [ch09]: Error Handling
- [ch10]: Generic Types, Traits, and Lifetimes
  - [generic]: Generic types examples
  - [lifetime]: Lifetime examples
- [ch11]: Writing Automated Tests
  - [tests/integration_test]: Integration test examples
- [ch12]: An I/O Project: Building a Command Line Program
- [ch14]: More About Cargo and Crates.io
- [fs]: std::fs examples

[the book]: https://doc.rust-lang.org/stable/book/
[the-book]: book/Cargo.toml
[array]: book/src/array.rs
[vector]: book/src/vector.rs
[string]: book/src/string.rs
[hashmap]: book/src/hashmap.rs
[ch09]: book/src/ch09.rs
[ch10]: book/src/ch10/mod.rs
[generic]: book/src/ch10/generic.rs
[lifetime]: book/src/ch10/lifetime.rs
[ch11]: book/src/ch11.rs
[ch12]: book/src/ch12.rs
[ch14]: book/src/ch14.rs
[tests/integration_test]: book/tests/integration_test.rs
[fs]: book/src/fs.rs

## async-book crate

[async-book] crate presents the examples provided in [the Async Book], the Async
Programming Book, with the [table driven] unit tests.  Here are the modules
in this crate:

- [ptr]: [Primitive Type pointer] and [Module std::ptr] examples
- [boxed]: [Module alloc::boxed] example
- [borrow]: [Enum alloc::borrow::Cow], [Trait alloc::borrow::Borrow] examples
- [futures]: [Crate futures] example
- [stream]: [Module futures::stream] example

[the async book]: https://rust-lang.github.io/async-book/
[async-book]: async/Cargo.toml
[ptr]: async/src/ptr.rs
[boxed]: async/src/boxed.rs
[borrow]: async/src/borrow.rs
[futures]: async/src/futures.rs
[stream]: async/src/stream.rs

## tokio-book crate

[tokio-took] crate presents the examples provided in
[tokio Getting Started Guide].

[tokio-book]: tokio/Cargo.toml
[tokio getting started guide]: https://tokio.rs/docs/overview/

## list-book crate

[list-book] crate presents the examples provided in [the List Book], The Unofficial
Too Many List Book, with the [table driven] unit tests.  Here are the modules
in this crate:

- [first]: [A Bad Singly-Linked Stack]
- [second]: [An Ok Singly-Linked Stack]

[list-book]: list/Cargo.toml
[first]: list/src/first.rs
[second]: list/src/second.rs
[the list book]: http://rust-unofficial.github.io/too-many-lists/
[A Bad Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/first.html
[An Ok Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/second.html

## flatbuf-tutorial

[flatbuf-tutorial] crate presents the [FlatBuffers tutorial] with some unit test.

- [monster]: Monster example

[flatbuf-tutorial]: flatbuf/Cargo.toml
[monster]: flatbuf/src/monster.rs
[flatbuffers tutorial]: https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html]

# Test

```sh
$ make test 2>/dev/null | head -20

running 13 tests
test borrow::tests::abs_all_array ... ok
test borrow::tests::borrow ... ok
test borrow::tests::borrow_mut ... ok
test borrow::tests::abs_all_vector ... ok
test boxed::tests::recursive_list ... ok
test borrow::tests::to_owned ... ok
test futures::test::async_bar_foo ... ok
test boxed::tests::stack_to_heap ... ok
test futures::test::join ... ok
test ptr::tests::null_ptr ... ok
test futures::test::select ... ok
test stream::tests::stream ... ok
test ptr::tests::reference ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


running 0 tests
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
- [Too Many Linked Lists]: Learning Rust with Entirely Too Many Linked List
- [The Unstable Book]: The book all about the cutting edge features!
- [FlatBuffers tutorial]:
  - [crate flatc-rust]: Programmatical way to invoke flatc command

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
[the unstable book]: https://doc.rust-lang.org/nightly/unstable-book/
[Flatbuffers tutorial]: https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html
[crate flatc-rust]: https://docs.rs/flatc-rust/0.1.2/flatc_rust/#examples

Happy Hacking!
