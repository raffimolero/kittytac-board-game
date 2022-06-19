use super::tile::Tile;
use crate::helpers::{arr_2d_from_iter, repeat_char, Color, RESET};
use rand::{distributions::Uniform, prelude::*};
use std::{fmt::Display, iter::repeat, ops::RangeInclusive};
use thiserror::Error;

// pub fn num_to_chars(range: RangeInclusive<char>, mut num: usize) -> String {
//     let radix = *range.end() as u8 - *range.start() as u8 + 1;
//     if num == 0 {
//         return "0".to_owned();
//     }
//     let mut digits = vec![];
//     while num > 0 {
//         digits.push(num % radix);
//         num /= radix;
//     }
//     digits
//         .into_iter()
//         .rev()
//         .map(|x| (x + range.start() as u8) as char)
//         .collect()
// }

// TODO: index board with position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let x = num_to_chars('a'..'z', self.x);
        // let x = self
        // let y = self.y.to_string();
        write!(f, "{}, {}", self.x, self.y)
    }
}

// TODO: move contents and api
enum Move {
    Move {
        from: Position,
        to: Position,
    },
    Push {
        from: Position,
        to: Position,
        push: Position,
    },
    Resign,
}

#[derive(Error, Debug)]
enum InvalidMove {
    #[error(
        "You cannot move a piece to {1}.\n\
        It's out of bounds for a board of size {0} by {0}."
    )]
    OutOfBounds(usize, Position),
}

pub struct Board<const N: usize> {
    pub tiles: [[Tile; N]; N],
}

impl<const N: usize> Board<N> {
    fn get_legal_moves() -> Vec<Move> {
        todo!()
    }

    fn make_move(&mut self) -> Result<(), InvalidMove> {
        todo!()
    }
}

impl Board<6> {
    pub fn new() -> Self {
        "
				rK rR __ __ __ __
				rN rP __ __ __ __
				__ __ __ __ __ __
				__ __ __ __ __ __
				__ __ __ __ bP bN
				__ __ __ __ bR bK
				"
        .into()
    }
}

impl Board<8> {
    pub fn new() -> Self {
        "
				__ __ __ __ __ rP rR rK
				__ __ __ __ __ __ rN rB
				__ __ __ __ __ __ rP __
				__ __ __ __ __ __ __ __
				__ __ __ __ __ __ __ __
				__ bP __ __ __ __ __ __
				bB bN __ __ __ __ __ __
				bK bR bP __ __ __ __ __
				"
        .into()
    }
}

impl<const N: usize> From<&str> for Board<N> {
    fn from(start_pos: &str) -> Self {
        let rng = thread_rng();
        let mut rng = Uniform::from(0..9).sample_iter(rng);
        let tile_chars = start_pos.split_whitespace();
        let tile_iter = tile_chars.map(|tile_char| {
            let roll = rng.next().unwrap();
            Tile::new(roll, tile_char)
        });
        let tiles = arr_2d_from_iter(tile_iter);
        Self { tiles }
    }
}

impl<const N: usize> Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bar = repeat_char('━', N * 3);
        let border_color = Color::Magenta.show(false, false);
        writeln!(f, "{border_color}┏{bar}┓",)?;
        for row in &self.tiles {
            write!(f, "┃")?;
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f, "{border_color}┃")?;
        }
        writeln!(f, "┗{bar}┛{RESET}")?;

        Ok(())
    }
}
