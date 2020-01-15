///! [Tokio echo server]
///
/// [tokio echo server]: https://github.com/tokio-rs/book/blob/master/overview.md
use std::error::Error;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;

    runtime.block_on(async {
        println!("hello world!");
        loop {
            tokio::time::delay_for(std::time::Duration::from_millis(1000)).await;
        }
    })
}
