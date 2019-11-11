// SPDX-License-Identifier: GPL-2.0
use futures::executor::block_on;

struct Song {
    name: String,
}

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}

async fn learn_song() -> Song {
    let name = String::from("You're my sunshine");
    Song { name }
}

async fn sing_song(song: Song) {
    println!("sing {}", song.name);
}

async fn dance() {
    println!("let's dance");
}
