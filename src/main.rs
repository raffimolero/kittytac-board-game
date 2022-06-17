use std::fmt::Display;

use rand::distributions::{Bernoulli, DistIter, Uniform};
use rand::prelude::*;

/// rolls a 6 sided die
fn roll() -> u8 {
    let mut rng = thread_rng();
    rng.gen_range(1..=6)
}

fn non_whitespace_chars<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(|c| !c.is_whitespace())
}

fn arr_2d_from_iter<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> [[T; N]; N] {
    [(); N].map(|_| [(); N].map(|_| iter.next().expect(&format!("Ran out of items in an iterator while trying to fill an {N} by {N} array."))))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
    color: Color,
    kind: PieceKind,
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = ["♟♝♞♜♚", "♙♗♘♖♔"][self.color as usize].chars().nth(self.kind as usize).unwrap();
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileKind {
    Normal,
    Goal(Color),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    height: u8,
    kind: TileKind,
    piece: Option<Piece>,
}
impl Tile {
    fn new(chance: u8, piece: &str) -> Self {
        use Color::*;
        use TileKind::*;
        use PieceKind::*;

        let mut chars = piece.chars();
        let color = match chars.next().unwrap() {
            'r' => Some(Red),
            'b' => Some(Blue),
            '_' => None,
            unknown => panic!("oi, pieces should start with a color: r or b, lowercase. instead you gave me {unknown:?}.")
        };
        let piece = color.map(|color| {
            let kind = match chars.next().expect("oi, pieces need to be 2 chars long.") {
                'K' => King,
                'R' => Rook,
                'N' => Knight,
                'B' => Bishop,
                'P' => Pawn,
                unknown => panic!("oi, use '_' for spaces. i don't know what piece {unknown:?} is.")
            };
            Piece { color, kind }
        });
        Tile {
            height: (chance < 3) as u8 + (chance == 0) as u8,
            kind: if let Some(Piece { color, kind: King }) = piece { Goal(color) } else { Normal },
            piece,
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [l, r] = match self.height {
            0 => [' ', ' '],
            1 => ['[', ']'],
            2 => ['(', ')'],
            _ => panic!("Taller blocks not supported.")
        };
        write!(f, "{l}{}{r}", self.piece.map_or('_'.into(), |p| p.to_string()))
    }
}

struct Board<const N: usize> {
    tiles: [[Tile; N]; N],
}
impl Board<6> {
    fn new() -> Self {
        "
        rK rR __ __ __ __
        rN rP __ __ __ __
        __ __ __ __ __ __
        __ __ __ __ __ __
        __ __ __ __ bP bN
        __ __ __ __ bR bK
        ".into()
    }
}
impl Board<8> {
    fn new() -> Self {
        "
        __ __ __ __ __ rP rR rK
        __ __ __ __ __ __ rN rB
        __ __ __ __ __ __ rP __
        __ __ __ __ __ __ __ __
        __ __ __ __ __ __ __ __
        __ bP __ __ __ __ __ __
        bB bN __ __ __ __ __ __
        bK bR bP __ __ __ __ __
        ".into()
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
        for row in &self.tiles {
            for tile in row {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let board = Board::<6>::new();
    println!("{board}");

    let board = Board::<8>::new();
    println!("{board}");
}
