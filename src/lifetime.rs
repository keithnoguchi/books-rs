// SPDX-License-Identifier: GPL-2.0
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    #[allow(dead_code)]
    fn new(part: &'a str) -> Self {
        Self { part }
    }
    #[allow(dead_code)]
    // no lifetime annotation for self because of
    // the lifetime elision.
    fn level(&self) -> i32 {
        5
    }
    #[allow(dead_code)]
    // no lifetime annotation for self, announcement, nor
    // return values because of the lifetime elision.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
    #[allow(dead_code)]
    // we need the explicit lifetime annotation to override the
    // lifetime elision, which is using 'a for the return value,
    // which is different from the actual code.
    fn announcement(&self, announcement: &'b str) -> &'b str {
        announcement
    }
}

#[allow(dead_code)]
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

#[allow(dead_code)]
fn longest_local<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        String::from("this is really long string")
    } else {
        String::from("this is yet longer string")
        // This will cause error due to returning the local
        // variable.
        //}
    };
    a
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
    #[test]
    fn longest_local() {
        let string1 = String::from("here you go!");
        let string2 = String::from("abc");
        let want = String::from("here you go!");
        let result = super::longest_local(&string1, &string2);
        assert_eq!(&want, result);
    }
    #[test]
    fn lifetime_in_struct() {
        let excerpt;
        {
            let novel = String::from("Call me Ishmael.  Some years ago...");
            let first_sentence = novel
                .split('.')
                .next()
                .expect("Can't find the '.'");
            excerpt = super::ImportantExcerpt::new(first_sentence);
            assert_eq!("Call me Ishmael", excerpt.part);
        }
        // This can't work due to lifetime annotation.
        //assert_eq!("Call me Ishmael", excerpt.part);
    }
}
