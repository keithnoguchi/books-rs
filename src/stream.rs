/// SPDX-License-Identifier: GPL-2.0
use futures::{channel::mpsc, sink::SinkExt};

#[allow(dead_code)]
async fn send(data: Vec<i32>) -> mpsc::Receiver<i32> {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    for v in &data {
        tx.send(*v).await.unwrap();
    }
    drop(tx);
    rx
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        use futures::stream::StreamExt;
        let test = async {
            let data = vec![1, 2];
            let mut rx = super::send(data).await;
            assert_eq!(Some(1i32), rx.next().await);
            assert_eq!(Some(2i32), rx.next().await);
            assert_eq!(None, rx.next().await);
        };
        futures::executor::block_on(test);
    }
}
