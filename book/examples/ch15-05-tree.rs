//! [Preventing Reference Cycles]: Turning an `Rc<T>` into a `Weak<T>`
//!
//! [preventing reference cycles]: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html#preventing-reference-cycles-turning-an-rct-into-a-weakt
use std::rc::Rc;

use the_book::ch15::sec05::Node;

fn main() {
    let children = vec![
        Rc::new(Node::new("alice")),
        Rc::new(Node::new("bob")),
        Rc::new(Node::new("chris")),
    ];
    let parent = Rc::new(Node::new("parent"));

    for child in children {
        parent.clone().add_child(child);
    }
    assert_eq!(1, Rc::strong_count(&parent));
    assert_eq!(3, Rc::weak_count(&parent));
}
