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
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
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
                want: vec![Shoe { size: 8.0, style: String::from("best fit") }],
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
                want: vec![ Shoe { size: 7.5, style: String::from("best fit") } ],
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
}
