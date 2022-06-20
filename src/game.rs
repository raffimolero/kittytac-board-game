#[cfg(test)]
mod tests;

pub mod board;
pub mod tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Red,
    Blue,
}

fn difference(x: usize, y: usize) -> usize {
    x.max(y) - x.min(y)
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

        let diff_x = dest.x as isize - self.x as isize;
        let diff_y = dest.y as isize - self.y as isize;

        let ortho = (diff_x == 0) ^ (diff_y == 0);
        let diag = diff_x.abs() == diff_y.abs();

        (ortho || diag).then(|| {
            fn project(dest: usize, diff: isize, cap: usize) -> Option<usize> {
                let projected = dest as isize + diff.signum();
                (0..cap as isize)
                    .contains(&projected)
                    .then(|| projected as usize)
            }
            Some(Self {
                x: project(dest.x, diff_x, cap.x)?,
                y: project(dest.y, diff_y, cap.y)?,
            })
        })?
    }

    pub fn moore_distance(self, other: Self) -> usize {
        let x_diff = difference(self.x, other.x);
        let y_diff = difference(self.y, other.y);
        x_diff.max(y_diff)
    }
}
