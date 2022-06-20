pub mod game;
pub mod helpers;

use game::board::Board;

fn main() {
    let board = Board::<6>::new();
    println!("{board}");

    let mut board = Board::<8>::new();
    println!("{board}");

    // let temp = board.tiles[0][7].piece;
    // board.tiles[0][7].piece = board.tiles[7][0].piece;
    // board.tiles[7][0].piece = temp;
    // println!("{board}");
}
