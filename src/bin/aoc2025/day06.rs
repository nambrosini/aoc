use std::fmt::Display;

#[derive(Default, Clone)]
struct Problem {
    numbers: Vec<i128>,
    operator: String,
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.numbers.iter().enumerate() {
            write!(f, "{n} ")?;
            if i == self.numbers.len() - 1 {
                write!(f, "= ")?;
            } else {
                write!(f, "{} ", self.operator)?;
            }
        }
        write!(f, "{}", self.solve())
    }
}

impl Problem {
    fn solve(&self) -> i128 {
        let op: &str = &self.operator;
        match op {
            "*" => self.numbers.iter().product(),
            "+" => self.numbers.iter().sum(),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.trim_end_matches("\n").lines().collect();
    let operations: Vec<&str> = lines
        .last()
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .collect();

    let mut problems = vec![Problem::default(); operations.len()];
    operations
        .iter()
        .enumerate()
        .for_each(|(i, o)| problems[i].operator = o.to_string());

    for l in &lines[..lines.len() - 1] {
        l.split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse().unwrap())
            .enumerate()
            .for_each(|(i, x)| problems[i].numbers.push(x));
    }

    problems
}

fn part1(input: &str) -> i128 {
    let input = parse(input);

    input.iter().map(|p| p.solve()).sum()
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let len = lines[0].len();
    let count = lines.len();

    let mut lines = lines.iter().map(|&l| l.chars().rev()).collect::<Vec<_>>();

    let mut buf: Vec<i64> = Vec::new();
    let mut total = 0;
    (0..len).for_each(|_| {
        let num = (0..count - 1)
            .map(|i| lines[i].next().unwrap())
            .filter(|&c| c != ' ')
            .collect::<String>();

        if !num.is_empty() {
            let num = num.parse::<i64>().unwrap();
            buf.push(num);
        }

        match lines[count - 1].next().unwrap() {
            '+' => {
                total += buf.iter().sum::<i64>();
                buf.clear();
            }
            '*' => {
                total += buf.iter().product::<i64>();
                buf.clear();
            }
            _ => {}
        }
    });

    total
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 3263827);
    }
}
