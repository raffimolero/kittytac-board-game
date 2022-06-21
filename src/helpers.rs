//! a bunch of general-purpose stuff

#[cfg(test)]
mod tests;

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

pub fn arr_2d_from_iter<T, const W: usize, const H: usize>(
    iter: &mut impl Iterator<Item = T>,
) -> [[T; W]; H] {
    [(); H].map(|_| {
        [(); W].map(|_| {
            iter.next().expect(&format!(
                "Ran out of items while trying to fill a {W} by {H} array."
            ))
        })
    })
}
