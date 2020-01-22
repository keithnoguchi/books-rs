//! Processing a Series of Items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html

/// `Shoe` type demonstrates the `filter` iterator adaptor.
#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: f32,
    pub style: String,
}

/// Collect the `shoe_size` [`Shoe`]s.
///
/// [`shoe`]: struct.Shoe.html
pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: f32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| (shoe.size - shoe_size).abs() < std::f32::EPSILON)
        .collect()
}

/// `Counter` type demonstrates the [`Iterrator`] trait implementation.
///
/// [`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
pub struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {
    /// Counter with the maximum `limit`.
    pub fn new(limit: u32) -> Self {
        Self { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = u32;
    /// It returns Some(u32) when the counter is less than `limit`,
    /// otherwise None.
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.limit {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shoes_in_my_size() {
        use super::Shoe;
        struct Test {
            shoes: Option<Vec<Shoe>>,
            size: f32,
            want: Vec<Shoe>,
        };
        let mut tests = vec![
            Test {
                shoes: Some(vec![
                    Shoe {
                        size: 8.0,
                        style: String::from("best fit"),
                    },
                    Shoe {
                        size: 7.5,
                        style: String::from("a bit small"),
                    },
                    Shoe {
                        size: 8.5,
                        style: String::from("a bit big"),
                    },
                ]),
                size: 8.0,
                want: vec![Shoe {
                    size: 8.0,
                    style: String::from("best fit"),
                }],
            },
            Test {
                shoes: Some(vec![
                    Shoe {
                        size: 8.0,
                        style: String::from("a bit small"),
                    },
                    Shoe {
                        size: 7.5,
                        style: String::from("best fit"),
                    },
                    Shoe {
                        size: 8.5,
                        style: String::from("even bigger"),
                    },
                ]),
                size: 7.5,
                want: vec![Shoe {
                    size: 7.5,
                    style: String::from("best fit"),
                }],
            },
        ];
        for t in &mut tests {
            match t.shoes.take() {
                None => panic!("unexpected test case"),
                Some(shoes) => {
                    let got = super::shoes_in_my_size(shoes, t.size);
                    assert_eq!(&t.want, &got);
                }
            }
        }
    }
    #[test]
    fn counter() {
        use super::Counter;
        struct Test {
            limit: u32,
            want: Vec<Option<u32>>,
        };
        let test = vec![
            Test {
                limit: 0,
                want: vec![None],
            },
            Test {
                limit: 3,
                want: vec![Some(1), Some(2), None],
            },
            Test {
                limit: 5,
                want: vec![Some(1), Some(2), Some(3), Some(4), None],
            },
        ];
        for t in &test {
            let mut counter = Counter::new(t.limit);
            for want in &t.want {
                assert_eq!(want, &counter.next());
            }
            // You can access the `Counter` iterator after
            // the actual iteration.  And the counter.count
            // actually contains the number of `next` method
            // call.
            assert_eq!(counter.count, t.want.len() as u32);
        }
    }
}
