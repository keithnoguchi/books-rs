//! [Borrowed pointers]
//!
//! [borrowed pointers]: https://github.com/nrc/r4cppp/blob/master/borrowed.md

fn main() {
    let x = &3; // type &i32
    assert_eq!(&3, x);
    let y = *x; // type i32
    assert_eq!(3, y);
    bar(x, *x);
    bar(&y, y);
    let x = 5;
    let y = &x;
    let z = y;
    let w = y;
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);
    assert_eq!(5, x);
    let mut x = 5;
    let y = &mut x;
    *y = 6;
    assert_eq!(6, *y);
    assert_eq!(6, x);
    let z = 3;
    let mut y = &x;
    assert_eq!(6, *y);
    y = &z;
    assert_eq!(3, *y);
    let mut x = 5;
    {
        let y = &x;
        //x = 4; can't update after immutable borrow above.
        assert_eq!(5, *y);
        assert_eq!(5, x); // but can access the original value.
    }
    {
        let y = &mut x;
        //x = 4; can't update after the mutable borrow above.
        *y = 3;
        assert_eq!(3, *y);
        assert_eq!(3, x); // we can access it as the immutable value.
        let z = &x;
        assert_eq!(3, *z); // and the immutable borrow, as well.
    }
    x = 4; // now I can change it, as immutable borrow is out of scope.
    assert_eq!(4, x);
    let x = Box::new(3);
    foo(&x);
    foo(&*x);
    let x = 4;
    //foo(x); // you can't do that.
    foo(&x);
}

fn bar(x: &i32, y: i32) {
    println!("x = {}, y = {}", x, y);
}

fn foo(x: &i32) {
    println!("x = {}", *x);
}
