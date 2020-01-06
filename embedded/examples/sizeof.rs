//! [Rust Pointers for C Programmers]
//!
//! [rust pointers for c programmers]: https://blahg.josefsipek.net/?p=580
use std::mem::size_of;

/// 100 bytes size type.
///
/// In C:
/// ```
/// struct T {
///     uint8_t stuff[100];
/// };
/// ```
struct T {
    _stuff: [u8; 100],
}

/// Check each size with [size_of].
///
/// [size_of]: https://doc.rust-lang.org/std/mem/fn.size_of.html
fn main() {
    let pointer_size: usize = 8; // on 64 bit architecture.
    let usize_size: usize = size_of::<usize>();
    assert_eq!(100, size_of::<T>());
    assert_eq!(pointer_size, size_of::<*const T>());
    assert_eq!(pointer_size, size_of::<*mut T>());
    assert_eq!(pointer_size, size_of::<&T>());
    assert_eq!(pointer_size, size_of::<&mut T>());
    assert_eq!(pointer_size, size_of::<Box<T>>());
    assert_eq!(200, size_of::<[T; 2]>());
    assert_eq!(pointer_size, size_of::<&[T; 2]>());
    //assert_eq!(unknown_size, size_of::<[T]>());
    assert_eq!(pointer_size + usize_size, size_of::<&[T]>());
}
