use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use crate::util::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub const DIRECTIONS: [Vec2; 4] = [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 1 },
    ];

    pub const DIR_WITH_DIAGONALS: [Vec2; 8] = [
        Vec2 { x: -1, y: 0 },
        Vec2 { x: 1, y: 0 },
        Vec2 { x: 0, y: -1 },
        Vec2 { x: 0, y: 1 },
        Vec2 { x: -1, y: -1 },
        Vec2 { x: -1, y: 1 },
        Vec2 { x: 1, y: -1 },
        Vec2 { x: 1, y: 1 },
    ];

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64,
        }
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn distance_2d(&self, other: &Vec2) -> Vec2 {
        *self - *other
    }

    pub fn manhattan(&self, other: &Vec2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn v(x: i64, y: i64) -> Vec2 {
    Vec2::new(x, y)
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<i64> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: i64) -> Vec2 {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i64) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

impl Add<Direction> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Direction) -> Vec2 {
        self + rhs.to_vec2()
    }
}

impl Sub<Direction> for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Direction) -> Vec2 {
        self - rhs.to_vec2()
    }
}

impl AddAssign<Direction> for Vec2 {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl SubAssign<Direction> for Vec2 {
    fn sub_assign(&mut self, rhs: Direction) {
        *self = *self - rhs;
    }
}
