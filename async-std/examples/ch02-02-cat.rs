//! [Tasks] example
//!
//! # Examples
//!
//! ```sh
//! $ cargo run --example ch02-02-cat Cargo.toml
//! Compiling async-std-book v0.1.0 (/home/kei/git/books-rs/async-std)
//! Finished dev [unoptimized + debuginfo] target(s) in 0.91s
//! Running `/home/kei/git/books-rs/target/debug/examples/ch02-02-cat Cargo.toml`
//! <<waiting for the task>>
//! [[started a task]]
//! [package]
//! name = "async-std-book"
//! version = "0.1.0"
//! authors = ["Keith Noguchi <keith.noguchi@gmail.com>"]
//! edition = "2018"
//!
//! [dependencies]
//! async-std = "^1"
//! [[finished a task]]
//! <<finish wating for the task>>
//! ```
//!
//! [tasks]: https://book.async.rs/concepts/tasks.html
use async_std::io::ReadExt;
use async_std::{fs::File, io, task};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let progname = args.next().unwrap();
    let path = args.next().unwrap_or_else(move || {
        eprintln!("usage: {} <filename>", progname);
        std::process::exit(1);
    });
    let reader = task::spawn(async move {
        eprintln!("[[started a task]]");
        match read_file(&path).await {
            Ok(data) => print!("{}", data),
            Err(err) => eprintln!("read_file(): {:?}", err),
        }
        eprintln!("[[finished a task]]");
    });
    eprintln!("<<waiting for the task>>");
    task::block_on(reader);
    eprintln!("<<finish wating for the task>>");
    Ok(())
}

// read_file to read a data from the specified `path` file
// and returns the String.
async fn read_file(path: &str) -> io::Result<String> {
    let mut f = File::open(path).await?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).await?;
    Ok(buf)
}
