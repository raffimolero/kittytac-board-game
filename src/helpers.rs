use std::fmt::Display;

/// for format strings
pub const RESET: &'static str = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
impl Color {
    pub fn trigger(code: &str) -> String {
        format!("\x1b[{code}m")
    }

    // pub fn reset() -> String {
    //     Self::trigger("0")
    // }

    pub fn show(self, back: bool, bright: bool) -> String {
        Self::trigger(&format!(
            "{}{}{}",
            if back { 4 } else { 3 },
            self as u8,
            if bright { ";1" } else { "" }
        ))
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show(false, false))
    }
}

pub fn arr_2d_from_iter<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> [[T; N]; N] {
    [(); N].map(|_| {
        [(); N].map(|_| {
            iter.next().expect(&format!(
                "Ran out of items in an iterator while trying to fill an {N} by {N} array."
            ))
        })
    })
}
