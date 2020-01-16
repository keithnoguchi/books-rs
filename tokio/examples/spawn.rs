//! [Spawning Tasks]
//!
//! [spawning tasks]: https://github.com/tokio-rs/book/blob/master/getting-started/runtime.md
use futures::{future, stream, StreamExt};
use std::error::Error;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let mut runtime = Runtime::new()?;
    let mut s = stream::iter(vec![0i32, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    runtime.block_on(async move {
        while let Some(value) = s.next().await {
            println!("Got value {:?} from the stream", value);
            let f = future::ready(value);

            // [Spawn] on the current local default tokio executor.
            //
            // [spawn]: https://docs.rs/tokio/0.1.22/tokio/executor/fn.spawn.html
            tokio::spawn(async move {
                let value = f.await;
                println!("Got value {:?} from the later future", value)
            });
        }
        Ok(())
    })
}
