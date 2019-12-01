// SPDX-License-Identifier: GPL-2.0
#[allow(dead_code)]
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

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
    #[test]
    fn longest() {
        struct Test {
            name: &'static str,
            a: String,
            b: &'static str,
            want: &'static str,
        }
        let tests = [
            Test {
                name: "longer string",
                a: String::from("abcd"),
                b: "cde",
                want: "abcd",
            },
            Test {
                name: "longer string slice",
                a: String::from("abc"),
                b: "cdef",
                want: "cdef",
            },
        ];
        for t in &tests {
            let got = super::longest(&t.a, t.b);
            debug_assert_eq!(t.want, got, "longest: {}", t.name);
        }
    }
    #[test]
    fn longest_lifetime() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = super::longest(&string1, &string2);
            assert_eq!(string1, result);
        }
        // This will violate the lifetime annotation.
        //assert_eq!(string1, result);
    }
}
