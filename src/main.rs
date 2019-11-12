// SPDX-License-Identifier: GPL-2.0
// https://rust-lang.github.io/async-book/print.html#task-wakeups-with-waker
use futures;
use rustbox::{exec, timer};
use std::{future, time};

fn main() {
    let (executor, spawner) = exec::new_executor_and_spawner();
    for i in 0..5 {
        spawner.spawn(async move {
            println!("[timer{}] howdy!", i);
            timer::TimerFuture::new(time::Duration::new(2, 0)).await;
            println!("[timer{}] done!", i);
        });
    }
    spawner.spawn(async {
        blocks().await;
    });
    spawner.spawn(async {
        move_block().await;
    });
    drop(spawner);
    executor.run();
}

async fn blocks() {
    let my_string = "hey".to_string();
    let future_one = async {
        println!("[future_one] {}", my_string);
    };

    let future_two = async {
        println!("[future_two] {}", my_string);
    };
    let ((), ()) = futures::join!(future_one, future_two);
}

fn move_block() -> impl future::Future<Output = ()> {
    let my_string = "foo".to_string();
    // https://rust-lang.github.io/async-book/print.html#async-move
    async move {
        println!("[move_block] {}", my_string);
    }
}
