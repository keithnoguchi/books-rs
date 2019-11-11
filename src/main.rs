// SPDX-License-Identifier: GPL-2.0
use futures::{self, executor::block_on};

struct Song {
    name: String,
}

fn main() {
    block_on(async_main());
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn learn_song() -> Song {
    let name = String::from("You're my sunshine");
    for _ in 1..1000 {
        print!(".")
    }
    Song { name }
}

async fn sing_song(song: Song) {
    println!("sing {}", song.name);
}

async fn dance() {
    println!("let's dance");
}
