//! Running Code on Cleanup with the [Drop Trait]
//!
//! [drop trait]: https://doc.rust-lang.org/book/ch15-03-drop.html
use the_book::ch15::sec03::SmartPointer;

fn main() {
    let (x, y) = ("hey", "rusta");
    let (a, b) = (SmartPointer::new(&x), SmartPointer::new(&y));
    println!("setup smart pointers a={}, b={}", *a, *b);
    drop(a);
    print!("you can't access moved/dropped a");
    //println!("={}", *a);
    println!();
    println!("but you can with b={}", *b);
}
