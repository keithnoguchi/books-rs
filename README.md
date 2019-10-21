# rustbox

[Rust] sandbox with the [table driven] unit tests.

[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/rustbox)

[Rust]: https://www.rust-lang.org
[table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
[CircleCI]: https://circleci.com/gh/keithnoguchi/rustbox.svg?style=svg

# Modules

- [array]: Standard array
- [vector]: Standard vector
- [string]: String type

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

Happy Hacking!

[array]: src/array.rs
[vector]: src/vector.rs
[string]: src/string.rs
