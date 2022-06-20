//! a bunch of general-purpose stuff

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    iter::repeat,
    ops::RangeInclusive,
};

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

pub fn getln() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf
}

pub fn println(msg: &str) {
    println!("{msg}");
}

pub fn num_to_char(num: u8, range: RangeInclusive<char>) -> char {
    debug_assert!(
        num + *range.start() as u8 <= *range.end() as u8,
        "{num} was too large to convert into a char between {range:?}"
    );
    (num + *range.start() as u8) as char
}

pub fn repeat_char(c: char, count: usize) -> String {
    repeat(c).take(count).collect::<String>()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "10 was too large to convert into a char between '0'..='9'"]
    fn test_num_to_char_overflow_decimal() {
        num_to_char(10, '0'..='9');
    }

    #[test]
    #[should_panic = "27 was too large to convert into a char between 'a'..='z'"]
    fn test_num_to_char_overflow_lowercase_letters() {
        num_to_char(27, 'a'..='z');
    }
}
