//! # Art
//!
//! A library for modeling artistic conecpts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(PartialEq, Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    ///
    ///  # Examples
    /// ```
    /// use art::{kinds::{PrimaryColor, SecondaryColor}, utils::mix};
    ///
    /// let c1 = PrimaryColor::Blue;
    /// let c2 = PrimaryColor::Red;
    ///
    /// assert_eq!(SecondaryColor::Purple, mix(c1, c2));
    /// ```
    /// # Panics
    /// panic when mixing same color
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == c2 {
            panic!("can't mix same color!. c1: {:?}, c2: {:?}", c1, c2);
        }
        let v = vec![c1, c2];
        if v.contains(&PrimaryColor::Red) && v.contains(&PrimaryColor::Yellow) {
            return SecondaryColor::Orange;
        } else if v.contains(&PrimaryColor::Yellow) && v.contains(&PrimaryColor::Blue) {
            return SecondaryColor::Green;
        } else {
            return SecondaryColor::Purple;
        }
    }
}
