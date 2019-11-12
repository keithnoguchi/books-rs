// SPDX-License-Identifier: GPL-2.0
// https://rust-lang.github.io/async-book/print.html#task-wakeups-with-waker
use rustbox::{exec, timer};
use std::time;

fn main() {
    let (executor, spawner) = exec::new_executor_and_spawner();
    spawner.spawn(async {
        println!("howdy!");
        timer::TimerFuture::new(time::Duration::new(2, 0)).await;
        println!("done!");
    });
    drop(spawner);
    executor.run();
}
