//! Smart Pointers example
use std::cell::RefCell;
use std::rc::Rc;

use the_book::ch15::{Graph, LimitTracker, List, Messenger, MyBox, RefCellList};

struct PrintMessenger;

impl Messenger for PrintMessenger {
    fn send(&self, msg: &str) {
        println!("{}", msg);
    }
}

fn main() {
    println!("============================");
    println!("ch15: Smart Pointer examples");
    println!("============================");
    let mut i32s = List::Nil;
    i32s = List::Cons(9i32, Box::new(i32s));
    i32s = List::Cons(8, Box::new(i32s));
    i32s = List::Cons(7, Box::new(i32s));
    let mut chars = List::Nil;
    chars = List::Cons('c', Box::new(chars));
    chars = List::Cons('b', Box::new(chars));
    chars = List::Cons('a', Box::new(chars));
    let mut f32s = List::Nil;
    f32s = List::Cons(9.2, Box::new(f32s));
    f32s = List::Cons(12.2, Box::new(f32s));
    f32s = List::Cons(11.2f32, Box::new(f32s));
    print_list("List<i32>", &i32s);
    print_list("List<char>", &chars);
    print_list("List<f32>", &f32s);
    let mut a = Rc::new(Graph::Nil);
    a = Rc::new(Graph::Vertex(5, vec![a]));
    a = Rc::new(Graph::Vertex(4, vec![a]));
    a = Rc::new(Graph::Vertex(3, vec![a]));
    let b = Graph::Vertex(1, vec![a.clone()]);
    let c = Graph::Vertex(2, vec![a.clone()]);
    print_graph("b", &b);
    print_graph("c", &c);
    // You can have multiple immutable borrow.
    let name = String::from("rust");
    let boxed_name = Box::new(&name); // just immutable borrowing the &str.
    let myboxed_name = MyBox::new(&name); // again, it's just immutable borrowing.
                                          // And the implicit deref coercision.
    hello(&name);
    hello(&boxed_name);
    hello(&myboxed_name);
    hello(&(*myboxed_name)[..]); // without the deref coercision.
    let name = String::from("rust");
    let mut myboxed_name = MyBox::new(name);
    (*myboxed_name).push_str("acian");
    hello(&myboxed_name);
    let messenger = PrintMessenger {};
    let mut tracker = LimitTracker::new(&messenger, 100);
    tracker.set_value(75);
    tracker.set_value(90);
    tracker.set_value(100);
    let a = Rc::new(RefCellList::Node(
        'a',
        RefCell::new(Rc::new(RefCellList::Null)),
    ));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());
    let b = Rc::new(RefCellList::Node('b', RefCell::new(a.clone())));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.next());
    println!("a next item = {:?}", a.next());
    if let Some(next) = a.next() {
        *next.borrow_mut() = b.clone();
    }
    println!("b rc count = {}", Rc::strong_count(&b));
    println!("a rc count = {}", Rc::strong_count(&a));
    // println!("a next items = {:?}", a.next());
    println!("main finished");
}

fn print_list<T>(title: &str, mut list: &List<T>)
where
    T: std::fmt::Display,
{
    println!("{}", title);
    while let List::Cons(val, next) = list {
        println!("{}", val);
        list = next;
    }
}

fn print_graph<T>(title: &str, mut vertex: &Graph<T>)
where
    T: std::fmt::Display,
{
    println!("Graph({})", title);
    while let Graph::Vertex(val, next) = vertex {
        println!("{}", val);
        vertex = &next[0];
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}
