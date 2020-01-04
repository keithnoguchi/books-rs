//! Final Project: Building a Multithreaded Web Server
use std::thread;
use std::time::Duration;
use the_book::ch20::WorkQueue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wq = WorkQueue::new(10);
    for id in 0..10 {
        wq.exec(move || {
            println!("Hello WorkQueue {}!", id);
            Ok(())
        })
        .unwrap();
    }
    println!("main thread sleeping...");
    thread::sleep(Duration::from_secs(2));
    Ok(())
}
