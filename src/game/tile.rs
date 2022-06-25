use super::{Position, Team};
use crate::helpers::{Color, RESET};
use std::fmt::Display;

#[cfg(test)]
mod tests;

pub static TILE_FLIPPING: bool = true;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
}
impl PieceKind {
    pub fn can_move(self, from: Position, to: Position) -> bool {
        // TODO: test
        // you know what tests suck maybe i should just resort to mathematical proofs
        use PieceKind::*;
        let dx = from.x.abs_diff(to.x);
        let dy = from.y.abs_diff(to.y);
        match self {
            Pawn | King => dx.max(dy) == 1,
            Bishop => dx == dy,
            Knight => dx * dy == 2,
            Rook => (dx == 0) ^ (dy == 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub team: Team,
    pub kind: PieceKind,
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = ["♟♝♞♜♚", "♙♗♘♖♔"][self.team as usize]
            .chars()
            .nth(self.kind as usize)
            .unwrap();

        let color = match self.team {
            Team::Red => Color::Red,
            Team::Blue => Color::Blue,
        }
        .show(false, true);

        write!(f, "{color}{icon}{RESET}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Normal,
    Goal(Team),
}
impl Default for TileKind {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub height: u8,
    pub kind: TileKind,
    pub piece: Option<Piece>,
}
impl Tile {
    pub fn new(chance: u8, piece: &str) -> Self {
        use PieceKind::*;
        use Team::*;
        use TileKind::*;

        let mut chars = piece.chars();
        let team = match chars.next().unwrap() {
				'r' => Some(Red),
				'b' => Some(Blue),
				'_' => None,
				unknown => panic!("oi, pieces should start with a team: r or b, lowercase. instead you gave me {unknown:?}.")
		};
        let piece = team.map(|team| {
            let kind = match chars.next().expect("oi, pieces need to be 2 chars long.") {
                'K' => King,
                'R' => Rook,
                'N' => Knight,
                'B' => Bishop,
                'P' => Pawn,
                unknown => {
                    panic!("oi, use '_' for spaces, and keep piece kinds uppercase. i don't know what piece {unknown:?} is.")
                }
            };
            Piece { team, kind }
        });
        Tile {
            height: (chance < 3) as u8 + (chance == 0) as u8,
            kind: if let Some(Piece { team, kind: King }) = piece {
                Goal(team)
            } else {
                Normal
            },
            piece,
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (bar, fg) = match self.height {
            0 => (" ", Color::Green),
            1 => ("|", Color::Yellow),
            2 => ("║", Color::Cyan),
            unknown => panic!("Max tile height is 2, not {unknown}."),
        };

        let fg = fg.show(false, false);
        let bg = match self.kind {
            TileKind::Normal => Color::Black,
            TileKind::Goal(Team::Red) => Color::Red,
            TileKind::Goal(Team::Blue) => Color::Blue,
        }
        .show(true, false);

        let piece = self.piece.map_or_else(
            || format!("{}◦{RESET}", Color::Black.show(false, false)),
            |mut p| {
                // p is a Copy of the piece, it does not need to be set back
                if TILE_FLIPPING && self.kind != TileKind::Normal {
                    p.team = match p.team {
                        Team::Red => Team::Blue,
                        Team::Blue => Team::Red,
                    }
                }
                p.to_string()
            },
        );
        // don't ask me how this works
        let bar = [bg, fg, bar.into()].concat();
        write!(f, "{bar}{piece}{bar}{RESET}")
    }
}
