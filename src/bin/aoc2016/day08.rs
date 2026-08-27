use itertools::Itertools;
use regex::Regex;
use std::convert::From;
use std::fmt;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let screen = calc(&input);

    screen.map.iter().flatten().filter(|&&x| x).count()
}

fn part2(input: &str) -> String {
    let input = parse(input);
    let screen = calc(&input);

    format!("\n{}", screen)
}

fn calc(instructions: &[Instruction]) -> Screen {
    let mut screen = Screen::new();

    for i in instructions {
        match i.operation {
            Operation::Rect => screen.rect(i.a, i.b),
            Operation::RotateRow => screen.rotate_row(i.a, i.b),
            Operation::RotateColumn => screen.rotate_column(i.a, i.b),
        }
    }

    screen
}

struct Screen {
    map: [[bool; 50]; 6],
}

impl Screen {
    fn new() -> Screen {
        Self {
            map: [[false; 50]; 6],
        }
    }

    fn rect(&mut self, a: usize, b: usize) {
        for i in 0..b {
            for j in 0..a {
                self.map[i][j] = true;
            }
        }
    }

    fn rotate_row(&mut self, a: usize, b: usize) {
        let row = self.map[a];

        let mut new_row = [false; 50];

        for i in 0..row.len() {
            let field = row[i];

            let new_index = (i + b) % row.len();

            new_row[new_index] = field;
        }

        self.map[a] = new_row;
    }

    fn rotate_column(&mut self, a: usize, b: usize) {
        let mut column = [false; 6];

        for (i, el) in column.iter_mut().enumerate() {
            *el = self.map[i][a];
        }

        let mut new_column = [false; 6];

        for i in 0..column.len() {
            let field = column[i];

            let new_index = (i + b) % column.len();

            new_column[new_index] = field;
        }

        for (i, el) in new_column.iter().enumerate() {
            self.map[i][a] = *el;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                write!(f, "{}", if self.map[i][j] { "#" } else { " " })?;
            }
            writeln!(f,)?;
        }
        writeln!(f,)
    }
}

enum Operation {
    Rect,
    RotateRow,
    RotateColumn,
}

struct Instruction {
    operation: Operation,
    a: usize,
    b: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"(rotate column|rotate row|rect) ((\d+)x(\d+)|(x|y)=(\d+) by (\d+))")
            .unwrap();
        let cap = re.captures_iter(s).next().unwrap();

        let operation = cap[1].into();

        let (a, b) = match operation {
            Operation::Rect => (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            _ => (cap[6].parse().unwrap(), cap[7].parse().unwrap()),
        };

        Self { operation, a, b }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "rect" => Self::Rect,
            "rotate row" => Self::RotateRow,
            "rotate column" => Self::RotateColumn,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
