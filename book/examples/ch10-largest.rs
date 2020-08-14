//! [Generic Types], Traits, and Lifetimes
//!
//! [generic types]: https://doc.rust-lang.org/book/ch10-00-generics.html
use std::cmp::Ordering;

use the_book::ch10::largest;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Yellow,
}

#[derive(Debug, PartialEq)]
enum Make {
    Honda,
    Subaru,
    Toyota,
}

#[derive(Debug, PartialEq)]
struct Car {
    id: usize,
    color: Color,
    make: Make,
}

impl Car {
    fn new(id: usize) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    fn make(mut self, make: Make) -> Self {
        self.make = make;
        self
    }
}

impl Default for Car {
    fn default() -> Self {
        Self {
            id: 0,
            color: Color::Red,
            make: Make::Honda,
        }
    }
}

impl PartialOrd for Car {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

#[derive(Debug, PartialEq)]
struct Dot<T: Default> {
    color: Color,
    loc: T,
}

impl<T: Default> Dot<T> {
    fn new(loc: T) -> Self {
        Self {
            loc,
            ..Self::default()
        }
    }
}

impl<T: Default> Default for Dot<T> {
    fn default() -> Self {
        Self {
            color: Color::Blue,
            loc: T::default(),
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let list: [i32; 0] = [];
    assert_eq!(None, largest(&list));
    let list = ['a'];
    assert_eq!(&'a', largest(&list).unwrap());
    let list = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(&'e', largest(&list).unwrap());
    let list = ['e', 'd', 'c', 'b', 'a'];
    assert_eq!(&'e', largest(&list).unwrap());
    let list = [1];
    assert_eq!(&1, largest(&list).unwrap());
    let list = [1, 2, 3, 4, 5];
    assert_eq!(&5, largest(&list).unwrap());
    let list = [5, 4, 3, 2, 1];
    assert_eq!(&5, largest(&list).unwrap());
    let list = [
        Car::new(0),
        Car::new(100).color(Color::Blue).make(Make::Toyota),
        Car::new(5).color(Color::Yellow).make(Make::Subaru),
    ];
    assert_eq!(
        &Car::new(100).color(Color::Blue).make(Make::Toyota),
        largest(&list).unwrap()
    );
    let p = Point { x: 1, y: 2 };
    assert_eq!(&1, p.x());
    assert_eq!(&2, &p.y);
    let p = Point {
        x: Dot::new(1),
        y: Dot::new(2),
    };
    assert_eq!(&Dot::new(1), p.x());
}
