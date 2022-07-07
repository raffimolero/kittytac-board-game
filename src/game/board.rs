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
    #[error("{0}")]
    Cancelled(#[from] Cancelled),

    #[error("{0}")]
    InvalidPosition(#[from] PositionParseErr),

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

pub enum Ruling {
    Allow,
    Warn { prompt: String, proceed: String },
    Deny(String),
}
impl Ruling {
    fn check(&self, mut io: impl IO) -> bool {
        use Ruling::*;
        match self {
            Allow => true,
            Warn { prompt, proceed } => {
                io.output(&prompt);
                io.output(&format!("Type {proceed} to continue anyway."));
                io.input() == *proceed
            }
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
            push_teammates: Allow,
            king_suicide: Warn {
                prompt: "You are about to kill your king. You will immediately lose the game if you continue.".to_owned(),
                proceed: "gg".to_owned(),
            },
            suicide_off_cliff: Warn {
                prompt: "Your piece will fall off a cliff and die.".to_owned(),
                proceed: "yeet".to_owned(),
            },
            suicide_off_board: Warn {
                prompt: "Your piece will fall off the board and die.".to_owned(),
                proceed: "adios".to_owned(),
            },
            cliff_bonk_friendly_fire: Warn {
                prompt: "Pushing this piece off a cliff will kill another one of your own pieces.".to_owned(),
                proceed: "bonk".to_owned(),
            },
        }
    }
}

#[derive(Error, Debug)]
#[error("Cancelled the move operation.")]
pub struct Cancelled;

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

impl<const N: usize> Board<N> {
    pub fn is_cliff(&self, from: Position, to: Position) -> bool {
        self[from].height - 2 >= self[to].height
    }

    // RESOLVE: mutable borrow or move?
    pub fn check_push(&self, mut rules: Rules,)

    // TODO: return value
    pub fn check_terrain(&self, mut rules: Rules, mut io: impl IO, mut from: Position, to: Position) {
        // [ ]
        // [x]
        // TODO: return value
        fn resolve_concern(ruling: &mut Ruling) {
            match ruling {
                Ruling::Allow => {}
                Ruling::Warn { prompt, proceed } => todo!(),
                Ruling::Deny(_) => todo!(),
            }
        }

        let dx = (to.x - from.x).signum();
        let dy = (to.y - from.y).signum();

        let is_king = self[from]
            .piece
            .expect("The From position was empty. This should've been checked before asking for move concerns.")
            .kind == PieceKind::King;
        let mut has_climbed = false;
        let mut prev_height;
        while from != to {
            if has_climbed {
                match rules.move_after_climb {
                    Ruling::Allow => {}
                    Ruling::Warn { prompt, proceed } => todo!(),
                    Ruling::Deny(_) => todo!(),
                }
            }
            prev_height = self[from].height;
            from.x += dx;
            from.y += dy;
            let cur_height = self[from].height;
            match [prev_height, cur_height] {
                [0, 2] => {
                    rules.climb_double_cliffs;
                }
                [2, 0] => {
                    rules.suicide_off_cliff;
                }
                [0, 1] | [1, 2] => {
                    has_climbed = true;
                }
            };
        }

        todo!()
    }

    fn input_position(
        &self,
        mut io: impl IO,
        check_bounds: bool,
        msg: &str,
    ) -> Result<Position, Cancelled> {
        loop {
            io.output(&format!("{self}\n{msg}"));
            let input = io.input();
            if input.to_lowercase() == "cancel" {
                return Err(Cancelled);
            }
            let pos = if let Ok(x) = input.parse::<Position>() {
                x
            } else {
                io.output("That isn't a valid position. Try again.");
                continue;
            };
            if check_bounds && (pos.x >= N as i32 || pos.y >= N as i32) {
                io.output(&format!(
                    "You cannot move a piece on {pos}.\n\
                    It's out of bounds for a board of size {N} by {N}."
                ));
                continue;
            }

            return Ok(pos);
        }
    }

    // TODO: test
    pub fn get_move_from(&self, mut io: impl IO) -> Result<Move, InvalidMove> {
        let from = self.input_position(&mut io, true, "Which piece would you like to move?")?;

        let piece = self[from].piece.ok_or(InvalidMove::EmptyPosition(from))?;

        if self.turn != piece.team {
            Err(InvalidMove::WrongTeam(self.turn, from, piece.team))?
        }

        let to = self.input_position(&mut io, false, "Where would you like to move that piece?")?;

        // TODO: check if terrain blocks the piece
        if !piece.kind.can_move(from, to) {
            Err(InvalidMove::InvalidTrajectory(piece.kind, from, to))?
        }
        self[to]
            .piece
            .map_or(Ok(Move::Move { from, to }), |pushee| {
                if piece.kind == PieceKind::Knight {
                    self.input_position(
                        &mut io,
                        false,
                        &format!(
                            "You are about to push a {pushee} with a Knight.\n\
                            Where would you like to push it?"
                        ),
                    )
                    .map_err(InvalidMove::Cancelled)
                    .and_then(|push| {
                        PieceKind::King
                            .can_move(to, push) // TODO: validate this push
                            .then_some(Move::KnightPush { from, to, push })
                            .ok_or(InvalidMove::InvalidPush(to, push))
                    })
                } else {
                    Ok(Move::Push { from, to })
                }
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
