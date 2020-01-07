use std::fmt;
use std::rc::Rc;

struct Foo(String);

impl Foo {
    fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let x = Rc::new(Foo::new("foo"));
    bar(x.clone());
    baz(&*x);
    let x = Rc::new(75);
    assert_eq!(75, *x);
}

fn bar<T>(x: T)
where
    T: fmt::Display,
{
    println!("bar: {}", x);
}

fn baz<T>(x: T)
where
    T: fmt::Display,
{
    println!("baz: {}", x);
}
