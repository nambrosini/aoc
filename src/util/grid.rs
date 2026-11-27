use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use pathfinding::directed::astar::{astar as pf_astar, astar_bag_collect};

use crate::util::position::Vec2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for e in row {
                write!(f, "{e}")?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Grid<T> {
    pub fn create_empty(width: usize, height: usize, fill: T) -> Self {
        Self(vec![vec![fill; width]; height])
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn count_neighbours(&self, target: T) -> Grid<u32> {
        let rows = self.len();
        let cols = self[0].len();
        let mut counts = Grid::create_empty(cols, rows, 9u32);
        for x in 0..rows {
            for y in 0..cols {
                if self[x][y] != target {
                    continue;
                }
                counts[x][y] = self.count_neighbours_at(x, y, target);
            }
        }
        counts
    }

    fn count_neighbours_at(&self, x: usize, y: usize, target: T) -> u32 {
        let rows = self.len();
        let cols = self[0].len();

        let x_min = x.saturating_sub(1);
        let x_max = (x + 1).min(rows - 1);
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(cols - 1);

        let mut count = 0;
        for i in x_min..=x_max {
            for j in y_min..=y_max {
                if i == x && j == y {
                    continue;
                }
                if self[i][j] == target {
                    count += 1;
                }
            }
        }
        count
    }
}

pub trait Parse {
    fn parse(input: &str) -> Self;
}

impl Parse for Grid<char> {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(|l| l.chars().collect()).collect())
    }
}

impl<T: From<char>> Grid<T> {
    pub fn parse_from_chars(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| l.chars().map(|c| c.into()).collect())
                .collect(),
        )
    }
}

impl Parse for Grid<i64> {
    fn parse(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
                .collect(),
        )
    }
}

pub trait Find<T> {
    fn find(&self, target: &T) -> Vec2;
    fn find_all(&self, target: &T) -> Vec<Vec2>;
}

impl<T: PartialEq> Find<T> for Grid<T> {
    fn find(&self, target: &T) -> Vec2 {
        for (i, row) in self.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                if e == target {
                    return Vec2::from_usize(i, j);
                }
            }
        }
        panic!("target not found in grid");
    }

    fn find_all(&self, target: &T) -> Vec<Vec2> {
        let mut found = Vec::new();
        for (i, row) in self.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                if e == target {
                    found.push(Vec2::from_usize(i, j));
                }
            }
        }
        found
    }
}

pub trait Inbound {
    fn inbound(&self, pos: &Vec2) -> bool;
}

impl<T> Inbound for Grid<T> {
    fn inbound(&self, pos: &Vec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x() < self.len()
            && self.first().is_some_and(|r| pos.y() < r.len())
    }
}

pub trait Read<T> {
    fn get(&self, pos: &Vec2) -> Option<T>;
}

impl<T: Copy> Read<T> for Grid<T> {
    fn get(&self, pos: &Vec2) -> Option<T> {
        if let Some(row) = self.iter().nth(pos.x()) {
            return row.get(pos.y()).copied();
        }
        None
    }
}

pub trait Write<T> {
    fn set(&mut self, pos: &Vec2, val: T);
}

impl<T> Write<T> for Grid<T> {
    fn set(&mut self, pos: &Vec2, val: T) {
        self[pos.x()][pos.y()] = val;
    }
}

pub trait ToMap<T> {
    fn to_map(&self) -> HashMap<Vec2, T>;
}

impl<T: Clone> ToMap<T> for Grid<T> {
    fn to_map(&self) -> HashMap<Vec2, T> {
        let mut map = HashMap::new();
        for (i, row) in self.iter().enumerate() {
            for (j, e) in row.iter().enumerate() {
                map.insert(Vec2::from_usize(i, j), e.clone());
            }
        }
        map
    }
}

fn successors<T: PartialEq + Copy>(
    grid: &Grid<T>,
    pos: &Vec2,
    blocked: Option<T>,
) -> Vec<(Vec2, i64)> {
    Vec2::DIRECTIONS
        .iter()
        .filter_map(|&d| {
            let next = *pos + d;
            if !grid.inbound(&next) {
                return None;
            }
            if blocked.is_some_and(|b| grid.get(&next).unwrap() == b) {
                return None;
            }
            Some((next, 1))
        })
        .collect()
}

pub fn astar<T: PartialEq + Copy>(
    grid: &Grid<T>,
    start: Vec2,
    goal: Vec2,
    blocked: Option<T>,
) -> Option<Vec<Vec2>> {
    pf_astar(
        &start,
        |p| successors(grid, p, blocked),
        |p| p.manhattan(&goal),
        |&p| p == goal,
    )
    .map(|(path, _cost)| path)
}

pub fn astar_all<T: PartialEq + Copy>(
    grid: &Grid<T>,
    start: Vec2,
    goal: Vec2,
    blocked: Option<T>,
) -> Vec<Vec<Vec2>> {
    astar_bag_collect(
        &start,
        |p| successors(grid, p, blocked),
        |p| p.manhattan(&goal),
        |&p| p == goal,
    )
    .map(|(paths, _cost)| paths)
    .unwrap_or_default()
}
