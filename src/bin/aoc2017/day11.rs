use std::ops::Add;

fn parse(input: &str) -> Vec<Direction> {
    input.trim().split(',').map(Direction::from).collect()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut position = Position::new(0, 0);

    for s in &input {
        position = position + s.into();
    }

    Position::new(0, 0).calc_distance(position)
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let start_position = Position::new(0, 0);
    let mut position = start_position;
    let mut max_distance = 0;

    for s in &input {
        position = position + s.into();

        let dist = start_position.calc_distance(position);

        if dist > max_distance {
            max_distance = dist;
        }
    }

    max_distance
}

enum Direction {
    N,
    Ne,
    Se,
    S,
    Sw,
    Nw,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Direction {
        match value {
            "n" => Direction::N,
            "ne" => Direction::Ne,
            "se" => Direction::Se,
            "s" => Direction::S,
            "sw" => Direction::Sw,
            "nw" => Direction::Nw,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    q: i32,
    r: i32,
}

impl Position {
    fn new(q: i32, r: i32) -> Position {
        Self { q, r }
    }

    fn calc_distance(&self, other: Position) -> i32 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::N => Position::new(0, -1),
            Direction::Ne => Position::new(1, -1),
            Direction::Se => Position::new(1, 0),
            Direction::S => Position::new(0, 1),
            Direction::Sw => Position::new(-1, 1),
            Direction::Nw => Position::new(-1, 0),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }
}
