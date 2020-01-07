//! Rust Pointers for C Programmers
//!
//! [rust pointers for c programmers]: https://blahg.josefsipek.net/?p=580

/// Pass the value as the mutable raw pointer.
#[no_mangle]
pub fn raw(p: *mut usize) {
    if !p.is_null() {
        unsafe {
            *p = 5;
        }
    }
}

/// Pass the value as the immutable raw pointer.
#[no_mangle]
pub fn deref_raw(p: *const usize) -> usize {
    if !p.is_null() {
        unsafe {
            *p
        }
    } else {
        0
    }
}

/// Pass the value as the mutable borrow.
#[no_mangle]
pub fn safe(p: &mut usize) {
    *p = 6;
}

/// Pass the value as the immutable borrow.
#[no_mangle]
pub fn deref_safe(p: &usize) -> usize {
    *p
}

fn main() {
    let mut p = 0;

    raw(&mut p as *mut usize);
    assert_eq!(p, deref_raw(&p as *const usize));
    safe(&mut p);
    assert_eq!(p, deref_safe(&p));
}
