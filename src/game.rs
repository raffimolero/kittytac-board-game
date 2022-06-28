use crate::helpers::num_to_char;
use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use thiserror::Error;

#[cfg(test)]
mod tests;

pub mod board;
pub mod tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Red,
    Blue,
}
impl Default for Team {
    fn default() -> Self {
        Self::Blue
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}
impl Position {
    /// pushes the destination by 1 tile in the opposite direction from self.
    /// returns None if self is not to dest is not in one of the 8 compass directions,
    /// or if the resulting position is out of bounds set from {0, 0}..cap.
    pub fn project(self, dest: Self, cap: Self) -> Option<Self> {
        if self == dest {
            None?
        }

        let diff_x = dest.x - self.x;
        let diff_y = dest.y - self.y;

        let ortho = (diff_x == 0) ^ (diff_y == 0);
        let diag = diff_x.abs() == diff_y.abs();

        (ortho || diag).then(|| {
            fn project(dest: i32, diff: i32, cap: i32) -> Option<i32> {
                let projected = dest + diff.signum();
                (0..cap).contains(&projected).then(|| projected)
            }
            Some(Self {
                x: project(dest.x, diff_x, cap.x)?,
                y: project(dest.y, diff_y, cap.y)?,
            })
        })?
    }
}

#[derive(Error, Debug)]
pub enum PositionParseErr {
    #[error("Empty position.")]
    NoFile,

    #[error(
        "{0:?} is not a valid file (column.)\n\
         Files are letters. They are not numbers or symbols."
    )]
    InvalidFile(char),

    #[error("A file (column) was specified without a rank (row.)")]
    NoRank,

    #[error(
        "{0:?} is not a valid rank (row.)\n\
         Ranks are single-digit numbers."
    )]
    InvalidRank(char),
}
impl FromStr for Position {
    type Err = PositionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        #[rustfmt::skip]
        let mut parse_next = |
            range: RangeInclusive<char>,
            on_empty: Self::Err,
            on_invalid: fn(char) -> Self::Err
        | -> Result<i32, Self::Err> {
            let symbol = chars.next().ok_or(on_empty)?.to_ascii_lowercase();
            if !range.contains(&symbol) {
                Err(on_invalid(symbol))?
            }
            Ok(symbol as i32 - *range.start() as i32)
        };

        // hell i could write a macro for this so i would just have to type `parse_next!('a'..='z', File)`
        // but that'd just be yeeting more mess into a different location coupled with this exact function
        Ok(Self {
            x: parse_next('a'..='z', Self::Err::NoFile, Self::Err::InvalidFile)?,
            y: parse_next('1'..='9', Self::Err::NoRank, Self::Err::InvalidRank)?,
        })
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = num_to_char(self.x as u8, 'a'..='z');
        let y = num_to_char(self.y as u8, '1'..='9');
        write!(f, "{x}{y}")
    }
}
