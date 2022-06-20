pub mod board;
pub mod tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}
impl Position {
    /// pushes the destination by 1 tile in the opposite direction from self.
    /// returns None if self is not to dest is not in one of the 8 compass directions,
    /// or if the resulting position is out of bounds set from {0, 0}..cap.
    pub fn project(self, dest: Self, cap: Self) -> Option<Self> {
        if self == dest {
            None?
        }
        fn project(from: usize, to: usize, cap: usize) -> Option<usize> {
            let difference = from as isize - to as isize;
            let projected = from as isize + difference.signum();
            (0..cap as isize)
                .contains(&projected)
                .then(|| projected as usize)
        }
        Some(Self {
            x: project(self.x, dest.x, cap.x)?,
            y: project(self.y, dest.y, cap.y)?,
        })
    }

    pub fn moore_distance(self, other: Self) -> usize {
        fn difference(x: usize, y: usize) -> usize {
            x.max(y) - x.min(y)
        }
        let x_diff = difference(self.x, other.x);
        let y_diff = difference(self.y, other.y);
        x_diff.max(y_diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test_moore_distance() {
    let p = [
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
        Position { x: 3, y: 2 },
        Position { x: 0, y: 8 },
        Position { x: 8, y: 0 },
    ];

    // i trust my moore distance function about as much as i trust this test
    // both work
    for (i, ((a, b), expected)) in p
        .iter()
        .flat_map(|a| p.iter().map(|b| (*a, *b)))
        .zip(
            [
                [0, 1, 1, 3, 8, 8],
                [1, 0, 1, 2, 8, 7],
                [1, 1, 0, 2, 7, 7],
                [3, 2, 2, 0, 6, 5],
                [8, 8, 7, 6, 0, 8],
                [8, 7, 7, 5, 8, 0],
            ]
            .concat(),
        )
        .enumerate()
    {
        assert_eq!(
            a.moore_distance(b),
            expected,
            "p[{}] was {a:?} and p[{}] was {b:?}",
            i / p.len(),
            i % p.len()
        );
    }
}
}
