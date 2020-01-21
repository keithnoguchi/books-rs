//! [Rc<T>], the Reference Counted Smart Pointer
//!
//! [rc<t>]: https://doc.rust-lang.org/book/ch15-04-rc.html
use std::rc::Rc;

use the_book::ch15::sec02::Graph;

fn main() {
    let mut a = Rc::new(Graph::Nil);
    a = Rc::new(Graph::Vertex(5, vec![a]));
    a = Rc::new(Graph::Vertex(4, vec![a]));
    a = Rc::new(Graph::Vertex(3, vec![a]));
    let b = Graph::Vertex(1, vec![a.clone()]);
    let c = Graph::Vertex(2, vec![a.clone()]);
    print("b", &b);
    print("c", &c);
}

fn print<T>(title: &str, mut vertex: &Graph<T>)
where
    T: std::fmt::Display,
{
    println!("Graph({})", title);
    while let Graph::Vertex(val, next) = vertex {
        println!("{}", val);
        vertex = &next[0];
    }
}
