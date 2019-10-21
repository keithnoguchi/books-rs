# rustbox

Rust sandbox for fun!

[![CircleCI]](https://circleci.com/gh/keithnoguchi/workflows/rustbox)

[CircleCI]: https://circleci.com/gh/keithnoguchi/rustbox.svg?style=svg

# Test

```sh
$ make test
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/rustbox-9e9575d91e3bd718

running 6 tests
test array::tests::largest_on_array ... ok
test vector::tests::length_of_vector ... ok
test array::tests::next_on_array ... ok
test vector::tests::next_on_vector ... ok
test vector::tests::push_on_vector ... ok
test vector::tests::pop_from_vector ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests rustbox

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$
```

Happy Hacking!
