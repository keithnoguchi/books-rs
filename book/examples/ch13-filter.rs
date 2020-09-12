//! Processing a series of items with [Iterators]
//!
//! [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let my_size = 13;
    let my_shoes = shoes_in_my_size(&shoes, my_size);
    println!("{:?}", shoes);
    let shoe = Shoe {
        size: 13,
        style: String::from("sandal"),
    };
    let want = vec![&shoe];
    assert_eq!(want, my_shoes);
}

fn shoes_in_my_size<'a>(shoes: &'a [Shoe], size: u32) -> Vec<&'a Shoe> {
    shoes.iter().filter(|shoe| shoe.size == size).collect()
}
