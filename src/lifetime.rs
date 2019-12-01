// SPDX-License-Identifier: GPL-2.0
#[rustfmt::skip]
#[cfg(test)]
mod tests {
    #[test]
    fn shorter_lifetime() {
        let r;                                       // --------+-- 'a
        {                                            //         |
            let x = 5;                               // -+-- 'b |
            r = &x;                                  //  |      |
            println!("r = {}", r);                   //  |      |
            assert_eq!(x, *r);                       //  |      |
            assert_eq!(&x, r);                       //  |      |
        }                                            // -+      |
        // this will cause a compile time error      //         |
        // due to the dangling reference.            //         |
        //println!("r = {}", r);                     //         |
    }                                                // --------+
    #[test]
    fn longer_lifetime() {
        let x = 5;                                   // --------+-- 'b
        let r = &x;                                  // -+-- 'a |
        assert_eq!(*r, x);                           //  |      |
        assert_eq!(r, &x);                           //  |      |
        println!("r = {}", r);                       //  |      |
    }                                                // -+------+
}
