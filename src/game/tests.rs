use super::*;
use test_case::test_case;

#[test_case(
    Position { x: 0, y: 0 },
    Position { x: 2, y: 0 },
    Some(Position { x: 3, y: 0 });
    "Project orthogonally"
)]
#[test_case(
    Position { x: 0, y: 0 },
    Position { x: 2, y: 2 },
    Some(Position { x: 3, y: 3 });
    "Project diagonally pos pos"
)]
#[test_case(
    Position { x: 5, y: 5 },
    Position { x: 1, y: 1 },
    Some(Position { x: 0, y: 0 });
    "Project diagonally neg neg"
)]
#[test_case(
    Position { x: 5, y: 1 },
    Position { x: 1, y: 5 },
    Some(Position { x: 0, y: 6 });
    "Project diagonally neg pos"
)]
#[test_case(
    Position { x: 7, y: 4 },
    Position { x: 5, y: 2 },
    Some(Position { x: 4, y: 1 });
    "Project diagonally offset"
)]
#[test_case(
    Position { x: 0, y: 0 },
    Position { x: 2, y: 4 },
    None;
    "Project knightwise"
)]
#[test_case(
    Position { x: 2, y: 2 },
    Position { x: 0, y: 0 },
    None;
    "Project out of bounds 0"
)]
#[test_case(
    Position { x: 4, y: 4 },
    Position { x: 1, y: 1 },
    Some(Position { x: 0, y: 0 });
    "Project in bounds 0"
)]
#[test_case(
    Position { x: 2, y: 2 },
    Position { x: 6, y: 6 },
    Some(Position { x: 7, y: 7 });
    "Project in bounds 7"
)]
#[test_case(
    Position { x: 2, y: 2 },
    Position { x: 7, y: 7 },
    None;
    "Project out of bounds 7"
)]
#[test_case(
    Position { x: 2, y: 2 },
    Position { x: 2, y: 2 },
    None;
    "Project equal"
)]
fn test_project(from: Position, to: Position, expected: Option<Position>) {
    let cap = Position { x: 8, y: 8 };
    assert_eq!(from.project(to, cap), expected);
}

// congratulations, we have eliminated an entire test
// very wow
// #[test]
// fn test_moore_distance() {
//     let p = [
//         Position { x: 0, y: 0 },
//         Position { x: 1, y: 0 },
//         Position { x: 1, y: 1 },
//         Position { x: 3, y: 2 },
//         Position { x: 0, y: 8 },
//         Position { x: 8, y: 0 },
//         Position { x: 8, y: 8 },
//     ];

//     // i was bored
//     // i trust my moore distance function about as much as i trust this test
//     // both work
//     for (i, ((a, b), expected)) in p
//         .iter()
//         .flat_map(|a| p.iter().map(|b| (*a, *b)))
//         .zip(
//             [
//                 [0, 1, 1, 3, 8, 8, 8],
//                 [1, 0, 1, 2, 8, 7, 8],
//                 [1, 1, 0, 2, 7, 7, 7],
//                 [3, 2, 2, 0, 6, 5, 6],
//                 [8, 8, 7, 6, 0, 8, 8],
//                 [8, 7, 7, 5, 8, 0, 8],
//                 [8, 8, 7, 6, 8, 8, 0],
//             ]
//             .concat(),
//         )
//         .enumerate()
//     {
//         assert_eq!(
//             a.moore_distance(b),
//             expected,
//             "p[{}] was {a:?} and p[{}] was {b:?}",
//             i / p.len(),
//             i % p.len()
//         );
//     }
// }
