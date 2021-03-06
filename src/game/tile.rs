use super::Team;
use crate::helpers::{Color, RESET};
use std::fmt::Display;

pub static TILE_FLIPPING: bool = true;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
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
            Team::Red => Color::Red.show(false, true),
            Team::Blue => Color::Blue.show(false, true),
        };
        write!(f, "{color}{icon}{RESET}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Normal,
    Goal(Team),
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
                    panic!("oi, use '_' for spaces. i don't know what piece {unknown:?} is.")
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
        let (l, r, fg) = match self.height {
            0 => (" ", " ", Color::Green),
            1 => ("|", "|", Color::Yellow),
            2 => ("║", "║", Color::Cyan),
            unknown => panic!("Max tile height is 2, not {unknown}."),
        };
        let bg = match self.kind {
            TileKind::Normal => Color::Black,
            TileKind::Goal(Team::Red) => Color::Red,
            TileKind::Goal(Team::Blue) => Color::Blue,
        }
        .show(true, false);
        let fg = fg.show(false, false);
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
        write!(f, "{bg}{fg}{l}{piece}{bg}{fg}{r}{RESET}")
    }
}
