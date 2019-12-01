/// SPDX-License-Identifier: GPL-2.0
use futures::{channel::mpsc, sink::SinkExt, stream};
use std::pin;

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

#[allow(dead_code)]
async fn sum_with_next(mut stream: pin::Pin<&mut dyn stream::Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // for `next`
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        struct Test {
            name: &'static str,
            data: Vec<i32>,
            want: Vec<Option<i32>>,
        };
        let tests = [
            Test {
                name: "send 1i32",
                data: vec![1],
                want: vec![Some(1), None],
            },
            Test {
                name: "send 1 and 2",
                data: vec![1, 2],
                want: vec![Some(1), Some(2), None],
            },
            Test {
                name: "send 1, 2, and 3",
                data: vec![1, 2, 3],
                want: vec![Some(1), Some(2), Some(3), None],
            },
        ];
        for t in &tests {
            let test = async {
                use futures::stream::StreamExt;
                let mut rx = super::send(t.data.clone()).await;
                for want in &t.want {
                    assert_eq!(*want, rx.next().await, "{}", t.name);
                }
            };
            futures::executor::block_on(test);
        }
    }
}
