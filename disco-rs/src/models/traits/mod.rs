use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, BitXor, Not},
};

pub mod messagable;

/// The bit flag trait contains methods which can be used on bitflags objects, such as gateway intents, permissions, etc.
pub trait BitFlag: PartialEq + Copy + Debug + BitOr + BitAnd + BitXor + Not {
    /// Checks if one set of bitflags, contains the other set of bitflags. (Note: 0 includes 0)
    fn includes(self, other: Self) -> bool;
    /// Checks if a set of bitflags includes at least one of the other set. (Note: 0 includes 0)
    fn includes_any(self, other: Self) -> bool;
}
