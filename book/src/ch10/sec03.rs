//! Validating References with [Lifetimes]
//!
//! [lifetimes]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
//!
//! # Examples
//!
//! Generic lifetimes in functions.
//!
//! ```
//! use the_book::ch10::longest;
//!
//! let a = "this is a sentence.";
//! let b = "This is also a sentence.";
//!
//! assert_eq!("This is also a sentence.", longest(&a, &b));
//! ```
//!
//! Different lifetime input case
//!
//! ```
//! use the_book::ch10::longest;
//!
//! let s1 = String::from("long string is long");
//! {
//!     let s2 = String::from("xyz");
//!     assert_eq!(
//!         &String::from("long string is long"),
//!         longest(&s1, &s2),
//!     );
//! }
//! ```
//!
//! Lifetime annotations in struct.
//!
//! ```
//! use the_book::ch10::ImportantExcerpt;
//!
//! let novel = String::from("Call me Ishmael.  Some years ago..."); // 'a -+
//! let first_sentence = novel.split('.').next().unwrap();           //     |
//! assert_eq!("Call me Ishmael", first_sentence);                   //     |
//! let i = ImportantExcerpt::new(first_sentence);                   //     |
//! assert_eq!("Call me Ishmael", i.part());                         //  <--+
//! ```
//!
//! Lifetime elision
//!
//! ```
//! use the_book::ch10::first_word;
//!
//! let sentence = String::from("This is a sentence."); // 'a -+
//! assert_eq!("This", first_word(sentence.as_str()));  //  <--+
//! ```
//!
//! Lifetime elision examples in methods.
//!
//! ```
//! use the_book::ch10::ImportantExcerpt;
//!
//! let novel = String::from("Call me Keith.  Some years ago...");
//! let mut lines = novel.split('.');
//! let first_sentence = lines.next().unwrap();
//! let i = ImportantExcerpt::new(first_sentence);
//! assert_eq!("Call me Keith", i.part());
//! let announcement = lines.next().unwrap().trim();
//! assert_eq!("Call me Keith", i.announce_and_return_part(announcement));
//! assert_eq!("Some years ago", i.announce_and_return_announcement(announcement));
//! ```

/// It returns the longest strings.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Lifetime annotations example in structures.
pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    /// `'a` lifetime parameter to `sentence` is required so that
    /// it will be linked to the `ImportantExcerpt` type itself.
    pub fn new(sentence: &'a str) -> Self {
        Self { part: sentence }
    }
    /// We don't need to provide the lifetime annotation for the
    /// return value, as it will get the &self lifetime annotation
    /// through the lifetime annotation rule #3.
    pub fn part(&self) -> &str {
        self.part
    }
    /// We don't need any lifetime annotations here, as `annoucement`
    /// will get its own lifetime annotation, e.g. `'b` through the
    /// first rule of the lifetime annotation elision rule and the
    /// return value will get the `'a` lifetime annotation, same lifetime
    /// with the `Self` type itself, through the third time lifetime
    /// annotation elision rule.
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
    /// This time, we need to give the explicit lifetime annotation to both
    /// the second argument, `announcement` as well as the return value,
    /// as the return value lifetime is not associated with the `Self` itself.
    pub fn announce_and_return_announcement<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        announcement
    }
}

/// `first_word` doesn't need the lifetime annotation because
/// the lifetime elision rule will put `'a` to both the argument
/// and the return value through the lifetime elision rule #1 and #2.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
