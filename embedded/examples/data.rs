//! [Data types]
//!
//! [data types]: https://github.com/nrc/r4cppp/blob/master/data-types.md
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    parent: Option<Rc<Node>>,
    value: i32,
}

impl Node {
    fn is_root(&self) -> bool {
        match self.parent {
            None => true,
            _ => false,
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("dropping {:?}", self);
    }
}

#[derive(Debug)]
struct S {
    field1: i32,
    field2: SomeOtherStruct,
}

#[derive(Debug)]
struct SomeOtherStruct;

impl S {
    fn new() -> Self {
        let field1 = 0;
        let field2 = SomeOtherStruct {};
        Self { field1, field2 }
    }
}

fn main() {
    let s = S::new();
    println!("S = {:?}", s);
    assert_eq!(0, s.field1);
    let root = Node { parent: None, value: 100 };
    assert_eq!(true, root.is_root());
    assert_eq!(100, root.value);
    let root = Rc::new(root);
    let leaf = Node { parent: Some(root.clone()), value: 115 };
    assert_eq!(false, leaf.is_root());
    assert_eq!(115, leaf.value);
    match &leaf.parent {
        Some(parent) => assert_eq!(&root, parent),
        _ => panic!("wrong parent"),
    };
}
