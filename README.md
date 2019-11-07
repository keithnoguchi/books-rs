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
- [boxed]: [Module alloc::boxed] example
- [borrow]: [Enum alloc::borrow::Cow], [Trait alloc::borrow::Borrow] examples
- [first]: [A Bad Singly-Linked Stack]
- [second]: [An Ok Singly-Linked Stack]

[array]: src/array.rs
[vector]: src/vector.rs
[string]: src/string.rs
[hashmap]: src/hashmap.rs
[generic]: src/generic.rs
[boxed]: src/boxed.rs
[borrow]: src/borrow.rs
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
- [Crate alloc]: The Rust core allocation and collections library
  - [Module alloc::boxed]: A pointer type for heap allocation
  - [Enum alloc::borrow::Cow]: A clone-on-write smart pointer
  - [Trait alloc::borrow::Borrow]: A trait for borrowing data
  - [Trait alloc::borrow::BorrowMut]: A trait for mutably borrowing data
  - [Trait alloc::borrow::ToOwned]: A generalization of Clone to borrowed data
- [Too Many Linked Lists]: Learning Rust with Entirely Too Many Linked List
- [The async book]: Asynchronous Programming in Rust
- [Zero-Cost Async IO]: RustLatam 2019 talk by [Without Boats]

[The book]: https://doc.rust-lang.org/stable/book/
[Crate alloc]: https://doc.rust-lang.org/alloc/index.html
[Module alloc::boxed]: https://doc.rust-lang.org/alloc/boxed/index.html
[Enum alloc::borrow::Cow]: https://doc.rust-lang.org/alloc/borrow/enum.Cow.html
[Trait alloc::borrow::Borrow]: https://doc.rust-lang.org/alloc/borrow/trait.Borrow.html
[Trait alloc::borrow::BorrowMut]: https://doc.rust-lang.org/alloc/borrow/trait.BorrowMut.html
[Trait alloc::borrow::ToOwned]: https://doc.rust-lang.org/alloc/borrow/trait.ToOwned.html
[Too Many Linked Lists]: http://rust-unofficial.github.io/too-many-lists/
[The async book]: https://rust-lang.github.io/async-book/
[Zero-Cost Async IO]: https://www.youtube.com/watch?v=skos4B5x7qE
[Without Boats]: https://github.com/withoutboats

Happy Hacking!
