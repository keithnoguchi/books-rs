//! [Unique Pointers]
//!
//! [unique pointers]: https://github.com/nrc/r4cppp/blob/master/unique.md

struct Foo(String);

impl Foo {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    fn foo(&self) {
        println!("{}", self.0);
    }
    fn name(&mut self, name: &str) {
        self.0 = name.to_string();
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn unique<T>(val: T) -> Box<T> {
    let x = Box::new(val);
    x
}

fn main() {
    let x = unique(75);
    assert_eq!(75, *x);
    assert_eq!(75, *x);
    assert_eq!(75, *x);
    assert_eq!(75, *x);
    let mut y = x;
    *y += 1;
    assert_eq!(76, *y);
    assert_eq!(76, *y);
    assert_eq!(76, *y);
    assert_eq!(76, *y);
    assert_eq!(76, *y);
    assert_eq!(76, *y);
    let x = unique(Foo::new("foo"));
    x.foo();
    x.foo();
    x.foo();
    let x = Box::new(Box::new(Box::new(Foo::new("deep foo"))));
    x.foo();
    x.foo();
    x.foo();
    let foo = Foo::new("another foo");
    let mut x = Box::new(foo);
    x.name("different foo");
    x.foo();
    x.foo();
    x.foo();
    let y = 75;
    let mut x = Box::new(y);
    *x = 76;
    assert_eq!(76, *x);
    assert_eq!(75, y);
    let mut x = Box::new(&y);
    *x = &4;
    assert_eq!(4, **x);
    assert_eq!(75, y);
}
