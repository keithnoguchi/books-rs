//! Build your own [executor], v1
//!
//! [executor]: https://stjepang.github.io/2020/01/31/build-your-own-executor.html
use stjepang_blog::post20200125::v4::block_on;
use stjepang_blog::post20200131::v1::spawn;

fn main() {
    let msg = block_on(async {
        let handle = spawn(async {
            "Hello world from our executor!"
        });
        handle.await
    });
    println!("{}", msg);
}
