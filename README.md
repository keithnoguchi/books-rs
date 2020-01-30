# books-rs

The collections of [Rust lang] books.

[![DroneCI]](https://cloud.drone.io/keithnoguchi/books-rs)
[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/books-rs)

[Rust lang]: https://www.rust-lang.org
[DroneCI]: https://cloud.drone.io/api/badges/keithnoguchi/books-rs/status.svg
[CircleCI]: https://circleci.com/gh/keithnoguchi/books-rs.svg?style=svg

# Books

## the-book

[the-book] crate is the examples shown in [the Book].

[the-book]: book/Cargo.toml
[the book]: https://doc.rust-lang.org/stable/book/

## style-book

[style-book] crate is the examples shown in [the style guide].

[style-book]: style/Cargo.toml
[the style guide]: https://doc.rust-lang.org/1.0.0/style/README.html

## async-book

[async-book] crate is the examples shown in [the Async Book], the Async
Programming Book.

- [ptr]: [Primitive Type pointer] and [Module std::ptr] examples
- [boxed]: [Module alloc::boxed] example
- [borrow]: [Enum alloc::borrow::Cow], [Trait alloc::borrow::Borrow] examples
- [futures]: [Crate futures] example
- [stream]: [Module futures::stream] example

[async-book]: async/Cargo.toml
[the async book]: https://rust-lang.github.io/async-book/
[ptr]: async/src/ptr.rs
[boxed]: async/src/boxed.rs
[borrow]: async/src/borrow.rs
[futures]: async/src/futures.rs
[stream]: async/src/stream.rs

## async-std-book

[async-std-book] crate for the examples in [async-std book].

[async-std-book]: async-std/README.md
[async-std book]: https://book.async.rs/

## tokio-book

[tokio-book] crate hosts the examples shown in the [Tokio 0.2 Book].

[tokio-book]: tokio/README.md
[tokio 0.2 book]: https://github.com/tokio-rs/book/blob/master/SUMMARY.md

## list-book

[list-book] crate is the examples shown in [the List Book], The Unofficial
Too Many List Book.

- [first]: [A Bad Singly-Linked Stack]
- [second]: [An Ok Singly-Linked Stack]

[list-book]: list/Cargo.toml
[first]: list/src/first.rs
[second]: list/src/second.rs
[the list book]: http://rust-unofficial.github.io/too-many-lists/
[A Bad Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/first.html
[An Ok Singly-Linked Stack]: http://rust-unofficial.github.io/too-many-lists/second.html

## flatbuf-tutorial

[flatbuf-tutorial] crate is the examples shown in [FlatBuffers tutorial].

- [monster]: Monster example

[flatbuf-tutorial]: flatbuf/Cargo.toml
[monster]: flatbuf/src/monster.rs

## Programming WebAssembly with Rust

[wasm] crate contains the examples shown in [Programming WebAssembly with Rust].

[wasm]: wasm/README.md
[programming webassembly with rust]: https://pragprog.com/book/khrust/programming-webassembly-with-rust

# Test

```sh
$ make test
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
- [Async-std book]: async-std makes async programming foundations easy and approachable.
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
  - [Writing Flatbuffers schema]
  - [crate flatc-rust]: Programmatical way to invoke flatc command
- [Lock-free Rust]: Crossbeam in 2019

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
[flatbuffers tutorial]: https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html
[writing flatbuffers schema]: https://google.github.io/flatbuffers/flatbuffers_guide_writing_schema.html
[crate flatc-rust]: https://docs.rs/flatc-rust/0.1.2/flatc_rust/#examples
[lock-free rust]: https://stjepang.github.io/2019/01/29/lock-free-rust-crossbeam-in-2019.html

Happy Hacking!
