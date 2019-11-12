/// SPDX-License-Identifier: GPL-2.0
use futures::{channel::mpsc, sink::SinkExt};

#[allow(dead_code)]
async fn send() -> mpsc::Receiver<i32> {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    tx.send(1i32).await.unwrap();
    tx.send(2i32).await.unwrap();
    drop(tx);
    rx
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        use futures::stream::StreamExt;
        let test = async {
            let mut rx = super::send().await;
            assert_eq!(Some(1i32), rx.next().await);
            assert_eq!(Some(2i32), rx.next().await);
            assert_eq!(None, rx.next().await);
        };
        futures::executor::block_on(test);
    }
}
