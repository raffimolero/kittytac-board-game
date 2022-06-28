//! Contains the logic for Boards and gameplay.

#[cfg(test)]
mod tests;

use super::{
    tile::{Piece, Tile},
    Position, PositionParseErr, Team,
};
use crate::{
    game::tile::PieceKind,
    helpers::{arr_2d_from_iter, repeat_char, Color, IO, RESET},
};
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use rand::{distributions::Uniform, prelude::*};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        "You cannot move a piece on {1}.\n\
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

pub enum GameState {
    Ongoing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<const N: usize> {
    pub tiles: [[Tile; N]; N],
    pub turn: Team,
}
// deriving default cannot use const generics.
// - [T; 0] is always Default, which would conflict with [T; N]
impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Self {
            tiles: [(); N].map(|_| [(); N].map(|_| Default::default())),
            turn: Default::default(),
        }
    }
}

pub struct Warning {
    prompt: String,
    proceed: String,
}
impl Warning {
    fn warn(&self, io: impl IO) -> bool {
        todo!()
    }
}

pub enum Ruling {
    Allow(Option<Warning>),
    Deny(String),
}
impl Ruling {
    fn check(&self, io: impl IO) -> bool {
        use Ruling::*;
        match self {
            Allow(None) => true,
            Allow(Some(warning)) => warning.warn(io),
            Deny(reason) => {
                io.output(reason);
                false
            }
        }
    }
}

pub struct Rules {
    climb_double_cliffs: Ruling,
    move_after_climb: Ruling,
    push_teammates: Ruling,
    king_suicide: Ruling,
    suicide_off_cliff: Ruling,
    suicide_off_board: Ruling,
    cliff_bonk_friendly_fire: Ruling,
}
impl Default for Rules {
    fn default() -> Self {
        use Ruling::*;
        Self {
            climb_double_cliffs: Deny("You cannot move a piece up a 2-high cliff.".to_owned()),
            move_after_climb: Deny("A piece cannot keep moving after climbing up a step.".to_owned()),
            push_teammates: Allow(None),
            king_suicide: Allow(Some(Warning {
                prompt: "You are about to kill your king. You will immediately lose the game if you continue.".to_owned(),
                proceed: "gg".to_owned(),
            })),
            suicide_off_cliff: Allow(Some(Warning {
                prompt: "Your piece will fall off a cliff and die.".to_owned(),
                proceed: "yeet".to_owned(),
            })),
            suicide_off_board: Allow(Some(Warning {
                prompt: "Your piece will fall off the board and die.".to_owned(),
                proceed: "adios".to_owned(),
            })),
            cliff_bonk_friendly_fire: Allow(Some(Warning {
                prompt: "Pushing this piece off a cliff will kill another one of your own pieces.".to_owned(),
                proceed: "bonk".to_owned(),
            })),
        }
    }
}

impl<const N: usize> Board<N> {
    pub fn is_cliff(&self, from: Position, to: Position) -> bool {
        self[from].height - 2 >= self[to].height
    }

    pub fn move_concerns(&self, rules: Rules, io: impl IO, mut from: Position, to: Position) {
        let dx = (to.x - from.x).signum();
        let dy = (to.y - from.y).signum();

        let is_king = matches!(
            self[from].piece,
            Some(Piece {
                kind: PieceKind::King,
                ..
            }),
        );
        let is_king = self[from]
            .piece
            .expect("The From position was empty. This should'e been checked before asking for move concerns.")
            .kind == PieceKind::King;
        let mut has_climbed = false;
        let mut prev_height;
        while from != to {
            prev_height = self[from].height;
            from.x += dx;
            from.y += dy;
            let cur_height = self[from].height;
            match [prev_height, cur_height] {
                [0, 2] => rules.climb_double_cliffs,
                [2, 0] => rules.suicide_off_cliff,
                _ => {}
            };
        }

        todo!()
    }

    // TODO: test
    pub fn get_move_from(&self, io: impl IO) -> Result<Move, InvalidMove> {
        let mut get_position = |check_bounds: bool, msg: &str| -> Result<Position, InvalidMove> {
            io.output(&format!("{self}\n{msg}"));
            let input = io.input();
            if input.to_lowercase() == "cancel" {
                Err(InvalidMove::Cancelled)?
            }
            let pos = input.parse::<Position>()?;
            if check_bounds && (pos.x >= N as i32 || pos.y >= N as i32) {
                Err(InvalidMove::OutOfBounds(N, pos))?
            }
            Ok(pos)
        };

        let from = get_position(true, "Which piece would you like to move?")?;

        let piece = self[from].piece.ok_or(InvalidMove::EmptyPosition(from))?;

        if self.turn != piece.team {
            Err(InvalidMove::WrongTeam(self.turn, from, piece.team))?
        }

        let to = get_position(false, "Where would you like to move that piece?")?;

        // TODO: check if terrain blocks the piece
        if !piece.kind.can_move(from, to) {
            Err(InvalidMove::InvalidTrajectory(piece.kind, from, to))?
        }
        Ok(if let Some(pushee) = self[to].piece {
            if piece.kind == PieceKind::Knight {
                let push = get_position(
                    false,
                    &format!(
                        "You are about to push a {pushee} with a Knight.\n\
                        Where would you like to push it?"
                    ),
                )?;
                if !PieceKind::King.can_move(to, push) {
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

    pub fn make_move_unchecked(&mut self, m: Move) -> Result<GameState, InvalidMove> {
        todo!()
    }

    pub fn get_legal_moves() -> Vec<Move> {
        todo!()
    }
}

impl<const N: usize> Index<Position> for Board<N> {
    type Output = Tile;

    fn index(&self, Position { x, y }: Position) -> &Self::Output {
        &self.tiles[N - 1 - y as usize][x as usize]
    }
}
impl<const N: usize> IndexMut<Position> for Board<N> {
    fn index_mut(&mut self, Position { x, y }: Position) -> &mut Self::Output {
        &mut self.tiles[N - 1 - y as usize][x as usize]
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
