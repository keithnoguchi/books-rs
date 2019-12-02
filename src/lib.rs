// SPDX-License-Identifier: GPL-2.0
//!
//! `rustbox` is a collection of examples demonstrated in
//! [the Rust Programming Language] with the [table driven] unit tests.
//!
//! [the rust programming language]: https://doc.rust-lang.org/stable/book/
//! [table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
//!

pub use chapter11::add_two;

mod array;
mod borrow;
mod boxed;
pub mod chapter11;
mod first;
mod fs;
mod futures;
mod generic;
mod hashmap;
mod lifetime;
mod ptr;
mod second;
mod simple;
mod stream;
mod string;
mod vector;
