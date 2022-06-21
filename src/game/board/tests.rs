use crate::game::tile::{Piece, TileKind};

use super::*;
use test_case::test_case;

#[test]
fn test_display_position() {
    assert_eq!(Position { x: 0, y: 0 }.to_string(), "a1");
    assert_eq!(Position { x: 1, y: 0 }.to_string(), "b1");
    assert_eq!(Position { x: 1, y: 1 }.to_string(), "b2");
    assert_eq!(Position { x: 7, y: 7 }.to_string(), "h8");
}

#[test]
fn test_fromstr_position() {
    assert_eq!(Position { x: 0, y: 0 }, "a1".parse::<Position>().unwrap());
    assert_eq!(Position { x: 1, y: 0 }, "b1".parse::<Position>().unwrap());
    assert_eq!(Position { x: 1, y: 1 }, "b2".parse::<Position>().unwrap());
    assert_eq!(Position { x: 7, y: 7 }, "h8".parse::<Position>().unwrap());
}

#[test_case(
    "a6",
    Position { x: 0, y: 5 },
    TileKind::Goal(Team::Red),
    Some(Piece {
        team: Team::Red,
        kind: PieceKind::King,
    });
    "Red King on a6"
)]
#[test_case(
    "f1",
    Position { x: 5, y: 0 },
    TileKind::Goal(Team::Blue),
    Some(Piece {
        team: Team::Blue,
        kind: PieceKind::King,
    });
    "Blue King on f1"
)]
#[test_case(
    "a1",
    Position { x: 0, y: 0 },
    TileKind::Normal,
    None;
    "Nothing on a1"
)]
fn test_index_board_in_bounds(
    notation: &str,
    expected_position: Position,
    tile_kind: TileKind,
    tile_piece: Option<Piece>,
) {
    let actual_position = notation.parse::<Position>().unwrap();
    assert_eq!(
        actual_position, expected_position,
        "{notation} didn't match with {expected_position}"
    );

    let board = Board::<6>::new();

    let Tile {
        height,
        kind,
        piece,
    } = board[actual_position];

    assert!(height <= 2);
    assert_eq!(kind, tile_kind);
    assert_eq!(piece, tile_piece);
}

#[should_panic]
#[test_case(
    "g1",
    Position { x: 6, y: 0 };
    "Index x"
)]
#[test_case(
    "a9",
    Position { x: 0, y: 8 };
    "Index y"
)]
#[test_case(
    "g9",
    Position { x: 6, y: 8 };
    "Index x and y"
)]
fn test_index_board_out_of_bounds(notation: &str, expected_position: Position) {
    let actual_position = notation.parse::<Position>().unwrap();
    assert_eq!(
        actual_position, expected_position,
        "{notation} didn't match with {expected_position}"
    );

    let board = Board::<6>::new();
    board[actual_position];
}

// i won't write tests for the board display, it's too dynamic.
// changing standards very quickly and keeping up is more important than ensuring stability.
#[test]
fn test_board_init() {
    let board = Board::<6>::new();
    println!("{board}");

    let board = Board::<8>::new();
    println!("{board}");
}
