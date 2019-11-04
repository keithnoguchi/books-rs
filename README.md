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

[array]: src/array.rs
[vector]: src/vector.rs
[string]: src/string.rs
[hashmap]: src/hashmap.rs
[generic]: src/generic.rs

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
- [The async book]: Asynchronous Programming in Rust

[The book]: https://doc.rust-lang.org/stable/book/
[The async book]: https://rust-lang.github.io/async-book/

Happy Hacking!
