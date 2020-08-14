//! [Generic Data Types]
//!
//! [generic data types]: https://doc.rust-lang.org/book/ch10-01-syntax.html

/// Generic `largest` function with `PartialOrd` trait bound
///
/// # Examples
///
/// ```
/// use the_book::ch10::largest;
///
/// let list = ['a', 'b', 'c', 'd', 'e'];
/// assert_eq!(&'e', largest(&list).unwrap());
/// let list: [i32; 0] = [];
/// assert_eq!(None, largest(&list));
/// ```
pub fn largest<T>(a: &[T]) -> Option<&T>
where
    T: PartialOrd,
{
    let mut i = a.iter();
    let mut largest = i.next()?;
    i.for_each(|item| {
        if *item > *largest {
            largest = item;
        }
    });
    Some(largest)
}

/// `Point` generic struct
///
/// # Examples
///
/// `Point` generic strucutre with the same types.
///
/// ```
/// use the_book::ch10::Point;
///
/// assert_eq!(&1, Point::new(1, 2).x());
/// assert_eq!(&2.0, Point::new(1.0, 2.0).y());
/// assert_eq!(&'a', Point::new('a', 'b').x())
/// ```
///
/// `Point` generic structure with the different types.
///
/// ```
/// use the_book::ch10::Point;
///
/// assert_eq!(&1, Point::new(1, 'a').x());
/// assert_eq!(&'b', Point::new(1.0, 'b').y());
/// ```
///
/// `mixup` `Point` method to showcase the method generic parameter
///
/// ```
/// use the_book::ch10::Point;
///
/// let p = Point::new(1, 5.4);
/// let q = Point::new('a', String::from("hello"));
///
/// let r = p.mixup(q);
/// assert_eq!(&1, r.x());
/// assert_eq!("hello", r.y());
/// ```
pub struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &U {
        &self.y
    }
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
