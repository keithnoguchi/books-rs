//! Using [Message Passing] to Transfer Data Between Threads
//!
//! [message passing]: https://doc.rust-lang.org/book/ch16-02-threads.html
use std::error::Error;
use std::sync::mpsc;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let data = String::from("hi");
        tx.send(data)
    });
    let data = rx.recv()?;
    println!("{}", data);
    Ok(())
}
