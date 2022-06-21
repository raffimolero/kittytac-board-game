//! Contains the logic for Boards and gameplay.

#[cfg(test)]
mod tests;

use super::{tile::Tile, Position, PositionParseErr, Team};
use crate::{
    game::tile::PieceKind,
    helpers::{arr_2d_from_iter, repeat_char, Color, RESET},
};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use rand::{distributions::Uniform, prelude::*};
use thiserror::Error;

// TODO: move contents and api
pub enum Move {
    Move {
        from: Position,
        to: Position,
    },
    Push {
        from: Position,
        to: Position,
    },
    KnightPush {
        from: Position,
        to: Position,
        push: Position,
    },
    Resign,
}

#[derive(Error, Debug)]
pub enum InvalidMove {
    #[error("Cancelled the move operation.")]
    Cancelled,

    #[error("{0}")]
    InvalidPosition(#[from] PositionParseErr),

    #[error(
        "You cannot move a piece to {1}.\n\
        It's out of bounds for a board of size {0} by {0}."
    )]
    OutOfBounds(usize, Position),

    #[error("The tile at position {0} is empty.")]
    EmptyPosition(Position),

    #[error("The {0:?} team tried to move the piece at {1}, which is from the {2:?} team.")]
    WrongTeam(Team, Position, Team),

    #[error("A {0:?} cannot move from {1} to {2}.")]
    InvalidTrajectory(PieceKind, Position, Position),

    #[error(
        "A Knight must push by exactly one space in one of 8 directions.\n\
        It cannot push a piece from {0} onto {1}."
    )]
    InvalidPush(Position, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<const N: usize> {
    pub tiles: [[Tile; N]; N],
    pub turn: Team,
}

impl<const N: usize> Board<N> {
    // TODO: write a test
    pub fn get_move_from(
        &self,
        mut input: impl FnMut() -> String,
        mut output: impl FnMut(&str),
    ) -> Result<Move, InvalidMove> {
        let mut get_bounded_position = |msg| {
            output(&format!("{self}\n{msg}"));
            let pos = input().parse::<Position>()?;
            if pos.x >= N || pos.y >= N {
                return Err(InvalidMove::OutOfBounds(N, pos));
            }
            Ok(pos)
        };

        let from = get_bounded_position("Which piece would you like to move?")?;

        let piece = if let Some(x) = self[from].piece {
            x
        } else {
            Err(InvalidMove::EmptyPosition(from))?
        };
        if self.turn != piece.team {
            Err(InvalidMove::WrongTeam(self.turn, from, piece.team))?
        }

        let to = get_bounded_position("Where would you like to move that piece?")?;

        if !piece.kind.can_move(from, to) {
            Err(InvalidMove::InvalidTrajectory(piece.kind, from, to))?
        }
        Ok(if let Some(pushee) = self[to].piece {
            if piece.kind == PieceKind::Knight {
                let push = get_bounded_position(&format!(
                    "You are about to push a {pushee} with a Knight.\n\
                    Where would you like to push it?"
                ))?;
                if to.moore_distance(push) != 1 {
                    Err(InvalidMove::InvalidPush(to, push))?
                }
                Move::KnightPush { from, to, push }
            } else {
                Move::Push { from, to }
            }
        } else {
            Move::Move { from, to }
        })
    }

    pub fn make_move(&mut self) -> Result<(), InvalidMove> {
        todo!()
    }

    pub fn get_legal_moves() -> Vec<Move> {
        todo!()
    }
}

impl<const N: usize> Index<Position> for Board<N> {
    type Output = Tile;

    fn index(&self, Position { x, y }: Position) -> &Self::Output {
        &self.tiles[N - 1 - y][x]
    }
}
impl<const N: usize> IndexMut<Position> for Board<N> {
    fn index_mut(&mut self, Position { x, y }: Position) -> &mut Self::Output {
        &mut self.tiles[N - 1 - y][x]
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

// fails if s does not contain the exact number of tiles needed
impl<const N: usize> From<&str> for Board<N> {
    fn from(s: &str) -> Self {
        let rng = thread_rng();
        let mut rng = Uniform::from(0..9).sample_iter(rng);

        let tile_chars = s.split_whitespace();
        let mut tile_iter = tile_chars.map(|tile_char| {
            let roll = rng.next().unwrap();
            Tile::new(roll, tile_char)
        });
        let tiles = arr_2d_from_iter(&mut tile_iter);

        assert_eq!(tile_iter.next(), None, "Board has too many tiles.");
        Self {
            tiles,
            turn: Team::Blue,
        }
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
