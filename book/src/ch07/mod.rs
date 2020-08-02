//! [Managing Growing Projects with Packages, Creates, and Modules]
//!
//! [managing growing projects with packages, creates, and modules]: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
#![allow(dead_code)]
// No need to make public to be accessed by the parent module.
mod front_of_house {
    /// child is private by default.  It should be explicitly opt in to
    /// be called by the parent with `pub(super)`.
    pub(super) fn hosting_add_to_waitlist() {
        hosting::add_to_waitlist()
    }

    /// Private functions free to be called by descendents.
    fn hosting_add_to_waitlist_backdoor() {
        hosting::add_to_waitlist()
    }

    pub(super) mod hosting {
        /// grand parent module can't access with `pub(super)` keyword.
        ///
        /// Instead, you need to call it from the parent function, e.g.
        /// `hosing_add_to_waitlist()` and make it `pub(super)` to be
        /// called by the grand parent.
        pub(super) fn add_to_waitlist() {}

        /// `pub(create)` makes the functions can be called by grand
        /// parent directly.
        pub(crate) fn seat_at_table() {}

        /// Private function which is not callable from outside the module,
        /// except descendents.
        fn cleanup_table() {}
    }
    pub(super) mod serving {
        pub(crate) fn take_order() {
            // I can access the parent functions no cost.
            super::hosting_add_to_waitlist_backdoor();
            // also `pub(super)` sibling functions.
            super::hosting::add_to_waitlist();
            // and of course, `pub(crate)` sibling functions.
            super::hosting::seat_at_table();
            // but not the private sibling functions.
            //super::hosting::cleanup_table();
        }
        pub(crate) fn serve_order() {
            // we can call `serve_order()` as it's a sibling function
            // and doesn't need any privacy policy.
            take_payment();
        }
        fn take_payment() {}
    }
}

pub(super) mod back_of_house {
    pub(crate) struct Breakfast {
        pub(crate) toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub(crate) fn summer(toast: &str) -> Self {
            Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Those two pub keywords, pub(super), pub(crate), are needed to
    // access the great grand child.
    front_of_house::hosting_add_to_waitlist();
    // Can call `pub(crate)` grand child function.
    front_of_house::hosting::seat_at_table();

    // Check if the sibling can call thier `public` functions.
    front_of_house::serving::take_order();

    // Check if the siblings can call each other.
    front_of_house::serving::serve_order();

    // Let's order a breakfast from the back of house.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Oops, changed my mind, let's make it to sourdough.
    meal.toast = String::from("Sourdough");
    println!("Let's eat {} toast", meal.toast);
    // you can't change the seasonal fruit.
    //meal.seasonal_fruit = String::from("apple");
}
