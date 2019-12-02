// SPDX-License-Identifier: GPL-2.0
//! More About Cargo and crates.io

pub mod art {
    //! A library for modeling artistic concepts
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub mod kinds {
        /// The primary colors according to the RYB color model.
        #[derive(PartialEq, Debug)]
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }
        /// The secondary colors according to the RYB color model.
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple,
        }
    }
    pub mod utils {
        use super::kinds::*;
        /// Combines two primary colors in equal amounts to create
        /// a secondary color.
        pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
            SecondaryColor::Orange
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn check_primary_color_re_exports() {
            struct Test {
                color: PrimaryColor,
                want: PrimaryColor,
            }
            let tests = [
                Test {
                    color: kinds::PrimaryColor::Red,
                    want: PrimaryColor::Red,
                },
                Test {
                    color: kinds::PrimaryColor::Yellow,
                    want: PrimaryColor::Yellow,
                },
                Test {
                    color: kinds::PrimaryColor::Blue,
                    want: PrimaryColor::Blue,
                },
            ];
            for t in &tests {
                assert_eq!(t.want, t.color);
            }
        }
    }
}
