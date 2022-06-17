mod game;
mod helpers;

use game::Board;

fn main() {
    let board = Board::<6>::new();
    println!("{board}");

    let board = Board::<8>::new();
    println!("{board}");
}
