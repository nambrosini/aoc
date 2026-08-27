use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Robot {
    let grid = input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect_vec())
        .collect_vec();

    Robot::new(grid)
}

fn part1(input: &str) -> usize {
    let mut robot = parse(input);

    robot.run(10000)
}

fn part2(input: &str) -> usize {
    let mut robot = parse(input);

    robot.run2(10000000)
}

#[derive(Debug, Clone)]
struct Robot {
    grid: HashMap<(i32, i32), Cell>,
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    fn new(grid: Vec<Vec<Cell>>) -> Self {
        let starting_position = ((grid.len() / 2) as i32, (grid[0].len() / 2) as i32);

        let mut map = HashMap::new();

        for (i, r) in grid.iter().enumerate() {
            for (j, e) in r.iter().enumerate() {
                map.insert((i as i32, j as i32), *e);
            }
        }

        Self {
            grid: map,
            position: starting_position,
            direction: Direction::Up,
        }
    }

    fn run(&mut self, total: usize) -> usize {
        let mut total_infected = 0;

        for _ in 0..total {
            let entry = self.grid.entry(self.position).or_insert(Cell::Clean);
            if *entry == Cell::Infected {
                self.direction = self.direction.rotate_right();
                *entry = Cell::Clean;
            } else {
                self.direction = self.direction.rotate_left();
                *entry = Cell::Infected;
                total_infected += 1;
            }
            self.move_forward();
        }

        total_infected
    }

    fn run2(&mut self, total: usize) -> usize {
        let mut total_infected = 0;

        for _ in 0..total {
            let entry = self.grid.entry(self.position).or_insert(Cell::Clean);

            match *entry {
                Cell::Clean => {
                    self.direction = self.direction.rotate_left();
                    *entry = Cell::Weakened;
                }
                Cell::Weakened => {
                    *entry = Cell::Infected;
                    total_infected += 1;
                }
                Cell::Infected => {
                    self.direction = self.direction.rotate_right();
                    *entry = Cell::Flagged;
                }
                Cell::Flagged => {
                    self.direction = self.direction.reverse();
                    *entry = Cell::Clean;
                }
            }
            self.move_forward();
        }

        total_infected
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.0 -= 1,
            Direction::Down => self.position.0 += 1,
            Direction::Right => self.position.1 += 1,
            Direction::Left => self.position.1 -= 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Clean,
            '#' => Cell::Infected,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/22.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..#
#..
...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5587);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2511944);
    }
}
