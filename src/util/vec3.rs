use std::fmt::Display;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub const fn v3(x: i64, y: i64, z: i64) -> Vec3 {
    Vec3::new(x, y, z)
}

impl Vec3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub const fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn z(&self) -> usize {
        self.z as usize
    }

    pub fn manhattan_distance(&self, other: &Self) -> u64 {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }

    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).pow(2) as f64
            + (self.y - other.y).pow(2) as f64
            + (self.z - other.z).pow(2) as f64)
            .sqrt()
    }
}

impl FromStr for Vec3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");

        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let z = split.next().unwrap().parse().unwrap();

        Ok(Self { x, y, z })
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        v3(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        v3(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        v3(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl From<(usize, usize, usize)> for Vec3 {
    fn from(value: (usize, usize, usize)) -> Self {
        Vec3::new(value.0 as i64, value.1 as i64, value.2 as i64)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::vec3::v3;

    #[test]
    fn test_euclidean_distance() {
        let p1 = v3(2, 5, 6);
        let p2 = v3(3, 4, 7);

        let actual = p1.euclidean_distance(&p2);
        let expected = 3f64.sqrt();

        assert_eq!(expected, actual);
    }
}
