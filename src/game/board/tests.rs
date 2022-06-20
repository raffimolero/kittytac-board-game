use super::*;

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

// there is no way i will rewrite the whole board init function in a more verbose manner.
// TODO:
// maybe i'll write an init function for smaller boards but i've already verified that it works.
// but i uh, don't know how it fails.
// #[test]
// fn test_board_init() {
//     assert_eq!(Board::<6>::new(), Board::<6> {
//         turn: Team::Red,
//         tiles: [
//             [Tile {

//             }]
//         ]
//     });
// }
