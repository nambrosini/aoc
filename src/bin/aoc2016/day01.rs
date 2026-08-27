use itertools::Itertools;
use std::convert::{From, TryFrom, TryInto};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

impl Direction {
    fn rotate(&self, step: &Step) -> Direction {
        match self {
            Direction::N => match step {
                Step::R(_) => Direction::E,
                Step::L(_) => Direction::W,
            },
            Direction::E => match step {
                Step::R(_) => Direction::S,
                Step::L(_) => Direction::N,
            },
            Direction::S => match step {
                Step::R(_) => Direction::W,
                Step::L(_) => Direction::E,
            },
            Direction::W => match step {
                Step::R(_) => Direction::N,
                Step::L(_) => Direction::S,
            },
        }
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => Position::new(0, 1),
            Direction::S => Position::new(0, -1),
            Direction::E => Position::new(1, 0),
            Direction::W => Position::new(-1, 0),
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<u32> for Position {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Self::new(self.x * other as i32, self.y * other as i32)
    }
}

#[derive(Debug)]
enum Step {
    R(u32),
    L(u32),
}

impl TryFrom<&str> for Step {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dir = &value[..1];
        let val = &value[1..].parse::<u32>().unwrap();

        match dir {
            "R" => Ok(Step::R(*val)),
            "L" => Ok(Step::L(*val)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct State {
    position: Position,
    direction: Direction,
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}

impl State {
    fn exec(&mut self, steps: &[Step]) {
        for s in steps {
            self.step(s);
        }
    }

    fn exec_part2(&mut self, steps: &[Step]) -> Position {
        let mut v = vec![self.position];
        for s in steps {
            for p in self.step(s) {
                if v.contains(&p) {
                    return p;
                }
                v.push(p);
            }
        }

        unreachable!();
    }

    fn step(&mut self, step: &Step) -> Vec<Position> {
        let new_dir = self.direction.rotate(step);
        let dir_pos: Position = new_dir.into();
        let val = match step {
            Step::R(val) => *val,
            Step::L(val) => *val,
        };

        let mut positions = vec![];

        for _ in 0..val {
            self.position = self.position + dir_pos;
            positions.push(self.position);
        }

        self.direction = new_dir;

        positions
    }
}

fn parse(input: &str) -> Vec<Step> {
    input
        .trim()
        .split(", ")
        .map(|step| step.try_into().unwrap())
        .collect_vec()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut state = State::default();
    state.exec(&input);
    state.position.distance()
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut state = State::default();
    let p = state.exec_part2(&input);
    p.distance()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("R2, L3"), 5);
        assert_eq!(part1("R2, R2, R2"), 2);
        assert_eq!(part1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("R8, R4, R4, R8"), 4);
    }
}
