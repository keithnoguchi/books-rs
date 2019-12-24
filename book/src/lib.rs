// SPDX-License-Identifier: GPL-2.0
//! `book` is a collection of examples demonstrated in
//! [the Rust Programming Language] with the [table driven] unit tests.
//!
//! [the rust programming language]: https://doc.rust-lang.org/stable/book/
//! [table driven]: https://dave.cheney.net/2019/05/07/prefer-table-driven-tests
pub use ch09::Error;
pub use ch11::add_two;

mod array;
pub mod ch09;
pub mod ch10;
pub mod ch11;
pub mod ch12;
pub mod ch14;
pub mod ch20;
mod fs;
mod hashmap;
mod vector;
