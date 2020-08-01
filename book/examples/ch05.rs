//! Using Structs to Structure related data
//!
//! [structure]: https://doc.rust-lang.org/book/ch05-00-structs.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn square(width: u32) -> Self {
        Self { width, height: width }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Self) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

fn main() {
    let square = Rectangle::square(5);
    let rect = Rectangle::new(10, 5);
    let other1 = Rectangle::new(9, 4);
    let other2 = Rectangle::new(11, 12);
    assert_eq!(25, square.area());
    assert_eq!(50, rect.area());
    assert!(rect.can_hold(&other1));
    assert!(!rect.can_hold(&other2));
    println!("area of {:?} is {}", square, square.area());
    println!("area of {:?} is {}", rect, rect.area());
    println!("{:?} can hold {:?}: {}", rect, other1, rect.can_hold(&other1));
    println!("{:?} can hold {:?}: {}", rect, other2, rect.can_hold(&other2));
}
