use std::ops::{Add, AddAssign};

fn parse(input: &str) -> Diagram {
    Diagram::new(input.lines().map(|x| x.chars().collect()).collect())
}

fn part1(input: &str) -> String {
    let mut diagram = parse(input);
    diagram.run().0
}

fn part2(input: &str) -> usize {
    let mut diagram = parse(input);
    diagram.run().1
}

#[derive(Debug, Clone)]
struct Diagram {
    path: Vec<Vec<char>>,
    position: Point,
    direction: Direction,
}

impl Diagram {
    fn new(path: Vec<Vec<char>>) -> Self {
        let position = Self::find_start(&path);

        Self {
            path,
            position,
            direction: Direction::S,
        }
    }

    fn find_start(path: &[Vec<char>]) -> Point {
        let (i, _) = path[0]
            .iter()
            .enumerate()
            .find(|&(_, &c)| c == '|')
            .unwrap();

        Point::new(i as i32, 0)
    }

    fn run(&mut self) -> (String, usize) {
        let mut res = String::new();
        let mut count = 1;

        loop {
            count += 1;
            self.position += self.direction.into();

            let val = self.path[self.position.y as usize][self.position.x as usize];

            match val {
                '|' | '-' => continue,
                '+' => self.direction = self.get_new_direction(),
                ' ' => unreachable!(),
                _ => {
                    res.push(val);
                    let next = self.position + self.direction.into();
                    if self.path[next.y as usize][next.x as usize] == ' ' {
                        return (res, count);
                    }
                }
            }
        }
    }

    fn get_new_direction(&self) -> Direction {
        let vals = [Direction::N, Direction::E, Direction::S, Direction::W];

        for d in vals {
            if d == self.direction || d == self.direction.rev() {
                continue;
            }

            let check = self.position + d.into();

            match self.path[check.y as usize][check.x as usize] {
                '-' | '|' => return d,
                _ => continue,
            }
        }

        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl From<Direction> for Point {
    fn from(d: Direction) -> Self {
        match d {
            Direction::N => Point::new(0, -1),
            Direction::E => Point::new(1, 0),
            Direction::S => Point::new(0, 1),
            Direction::W => Point::new(-1, 0),
        }
    }
}

impl Direction {
    fn rev(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

impl Add for Point {
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }

    type Output = Self;
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
