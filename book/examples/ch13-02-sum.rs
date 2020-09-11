//! Consuming adaptor, sum(), example
fn main() {
    let v = vec![1, 2, 3];
    // It doesn't have to be mutable because sum will consume the iterator.
    let i = v.iter();
    let total = i.sum();
    // You can't borrow the iterator after the call to sum() as it's already
    // consumed.
    //println!("iterator after consuming sum() call: {:?}", i);
    assert_eq!(6, total);
}
