// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn null_ptr() {
        use std::ptr;
        let p: *const i32 = ptr::null();
        assert!(p.is_null());
    }
    #[test]
    fn reference() {
        let my_num: i32 = 10;
        let my_num_ptr: *const i32 = &my_num;
        unsafe {
            assert_eq!(my_num, *my_num_ptr);
        }
    }
}
