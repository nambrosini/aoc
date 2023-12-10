use std::ops::Add;

use crate::util::direction::Direction;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

impl Position {
    pub fn x(&self) -> usize {
        self.x as usize
    }
    pub fn y(&self) -> usize {
        self.y as usize
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<&Direction> for Position {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => Position::new(-1, 0),
            Direction::South => Position::new(1, 0),
            Direction::East => Position::new(0, 1),
            Direction::West => Position::new(0, -1),
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position::new(value.0 as i64, value.1 as i64)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        (&value).into()
    }
}
