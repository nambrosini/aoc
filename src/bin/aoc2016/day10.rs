use itertools::Itertools;
use regex::Regex;
use std::{cmp, collections::HashMap};

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let (target_low, target_high) = (61, 17);
    let mut input = parse(input);
    let mut robots: HashMap<usize, Bot> = HashMap::new();
    let mut output: HashMap<usize, usize> = HashMap::new();

    loop {
        let mut new_input: Vec<Operation> = vec![];

        for op in &input {
            resolve(op, &mut robots, &mut output, &mut new_input);

            if let Some((&id, _)) = robots
                .iter()
                .find(|(_, b)| b.has_values(target_low, target_high))
            {
                return id;
            }
        }

        assert_ne!(new_input.len(), input.len(), "no progress made");
        input = new_input;
    }
}

fn part2(input: &str) -> usize {
    let mut input = parse(input);
    let mut robots: HashMap<usize, Bot> = HashMap::new();
    let mut output: HashMap<usize, usize> = HashMap::new();

    loop {
        let mut new_input: Vec<Operation> = vec![];

        for op in &input {
            resolve(op, &mut robots, &mut output, &mut new_input);

            if let (Some(&a), Some(&b), Some(&c)) = (output.get(&0), output.get(&1), output.get(&2))
            {
                return a * b * c;
            }
        }

        assert_ne!(new_input.len(), input.len(), "no progress made");
        input = new_input;
    }
}

fn resolve(
    op: &Operation,
    robots: &mut HashMap<usize, Bot>,
    output: &mut HashMap<usize, usize>,
    new_input: &mut Vec<Operation>,
) {
    match op {
        Operation::Gets(id, v) => {
            let b = robots.entry(*id).or_insert_with(Bot::new);
            if !b.gets(*v) {
                new_input.push(*op);
            }
        }
        Operation::Gives(id, rec1, rec2) => {
            let mut b: Bot = match robots.get(id) {
                Some(b) if b.has_values_assigned() => *b,
                _ => {
                    new_input.push(*op);
                    return;
                }
            };

            for (rec, value) in [(rec1, Value::Low), (rec2, Value::High)] {
                match rec {
                    Receiver::Bot(idr) => {
                        let r = robots.entry(*idr).or_insert_with(Bot::new);
                        if !b.gives(r, value) {
                            new_input.push(*op);
                            return;
                        }
                    }
                    Receiver::Output(idr) => {
                        if let Some(v) = b.take(value) {
                            output.insert(*idr, v);
                        } else {
                            new_input.push(*op);
                            return;
                        }
                    }
                }
            }

            robots.insert(*id, b);
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    // Gets(id, val)
    Gets(usize, usize),
    // Gives(id, low_receiver, high_receiver)
    Gives(usize, Receiver, Receiver),
}

#[derive(Debug, Clone, Copy)]
enum Receiver {
    Bot(usize),
    Output(usize),
}

impl From<(&str, &str)> for Receiver {
    fn from(s: (&str, &str)) -> Self {
        match s.0 {
            "bot" => Self::Bot(s.1.parse().unwrap()),
            "output" => Self::Output(s.1.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        if s.starts_with("value") {
            let re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
            let cap = re.captures_iter(s).next().unwrap();

            Self::Gets(cap[2].parse().unwrap(), cap[1].parse().unwrap())
        } else {
            let re = Regex::new(
                r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)",
            )
            .unwrap();

            let cap = re.captures_iter(s).next().unwrap();

            let id: usize = cap[1].parse().unwrap();
            let receiver1: Receiver = (&cap[2], &cap[3]).into();
            let receiver2: Receiver = (&cap[4], &cap[5]).into();

            Self::Gives(id, receiver1, receiver2)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Low,
    High,
}

#[derive(Debug, Clone, Copy)]
struct Bot {
    a: Option<usize>,
    b: Option<usize>,
}

impl Bot {
    fn new() -> Self {
        Bot { a: None, b: None }
    }

    fn gets(&mut self, value: usize) -> bool {
        if self.a.is_none() {
            self.a = Some(value);
            return true;
        } else if self.b.is_none() {
            self.b = Some(value);
            return true;
        }
        false
    }

    fn gives(&mut self, other: &mut Self, value: Value) -> bool {
        if self.a.is_none() && self.b.is_none() {
            return false;
        }

        if self.a.is_some() && self.b.is_none() {
            return other.gets(self.a.take().unwrap());
        }

        if self.b.is_some() && self.a.is_none() {
            return other.gets(self.b.take().unwrap());
        }

        match value {
            Value::Low => match self.a.unwrap().cmp(&self.b.unwrap()) {
                cmp::Ordering::Less => {
                    let a = self.a.take().unwrap();
                    if other.gets(a) {
                        return true;
                    }
                }
                cmp::Ordering::Greater => {
                    let b = self.b.take().unwrap();
                    if other.gets(b) {
                        return true;
                    }
                }
                _ => unreachable!(),
            },
            Value::High => match self.a.unwrap().cmp(&self.b.unwrap()) {
                cmp::Ordering::Less => {
                    let b = self.b.take().unwrap();
                    if other.gets(b) {
                        return true;
                    }
                }
                cmp::Ordering::Greater => {
                    let a = self.a.take().unwrap();
                    if other.gets(a) {
                        return true;
                    }
                }
                _ => unreachable!(),
            },
        }

        false
    }

    fn take(&mut self, value: Value) -> Option<usize> {
        if !self.has_values_assigned() {
            return None;
        }
        match value {
            Value::Low => {
                if self.a.unwrap() < self.b.unwrap() {
                    self.a.take()
                } else {
                    self.b.take()
                }
            }
            Value::High => {
                if self.b.unwrap() < self.a.unwrap() {
                    self.b.take()
                } else {
                    self.a.take()
                }
            }
        }
    }

    fn has_values(&self, v1: usize, v2: usize) -> bool {
        if let Some(a) = self.a
            && let Some(b) = self.b
            && (a == v1 && b == v2 || a == v2 && b == v1)
        {
            return true;
        }
        false
    }

    fn has_values_assigned(&self) -> bool {
        self.a.is_some() && self.b.is_some()
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
