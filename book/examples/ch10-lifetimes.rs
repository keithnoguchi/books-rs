//! [Lifetimes]
//!
//! [lifetimes]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("\tr: {}", r);
    }
    //println!("r: {}", r);
}
