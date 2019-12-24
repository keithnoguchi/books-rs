//! Final Project: Building a Multithreaded Web Server
use the_book::ch20::WorkQueue;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wq = WorkQueue::new(1);
    wq.exec(|| {
        println!("Hello WorkQueue!");
        Ok(())
    })
}
