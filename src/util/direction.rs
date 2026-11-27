use std::fmt;
use std::ops::Mul;

use crate::util::position::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Rotation {
    Right,
    Left,
    None,
}

impl Direction {
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(-1, 0),
            Direction::Down => Vec2::new(1, 0),
            Direction::Left => Vec2::new(0, -1),
            Direction::Right => Vec2::new(0, 1),
        }
    }

    pub fn rotate(&self, rotation: &Rotation) -> Direction {
        match (self, rotation) {
            (dir, Rotation::None) => *dir,
            (Direction::Up, Rotation::Right) => Direction::Right,
            (Direction::Right, Rotation::Right) => Direction::Down,
            (Direction::Down, Rotation::Right) => Direction::Left,
            (Direction::Left, Rotation::Right) => Direction::Up,
            (Direction::Up, Rotation::Left) => Direction::Left,
            (Direction::Left, Rotation::Left) => Direction::Down,
            (Direction::Down, Rotation::Left) => Direction::Right,
            (Direction::Right, Rotation::Left) => Direction::Up,
        }
    }

    pub fn get_rotation(&self, other: &Direction) -> Rotation {
        match (self, other) {
            (Direction::Up, Direction::Left)
            | (Direction::Left, Direction::Down)
            | (Direction::Down, Direction::Right)
            | (Direction::Right, Direction::Up) => Rotation::Left,
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => Rotation::Right,
            _ => Rotation::None,
        }
    }

    pub fn char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Mul<i64> for Direction {
    type Output = Vec2;
    fn mul(self, rhs: i64) -> Vec2 {
        let v = self.to_vec2();
        Vec2::new(v.x * rhs, v.y * rhs)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char())
    }
}

impl From<Vec2> for Direction {
    fn from(v: Vec2) -> Direction {
        match (v.x.signum(), v.y.signum()) {
            (-1, 0) => Direction::Up,
            (1, 0) => Direction::Down,
            (0, -1) => Direction::Left,
            (0, 1) => Direction::Right,
            _ => unreachable!("not a cardinal direction: {v:?}"),
        }
    }
}
