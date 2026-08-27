use std::sync::LazyLock;

use regex::Regex;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+)-(\d+)\s([a-z]):\s([a-z]+)").unwrap());

#[derive(Debug, Clone)]
struct Password {
    first: usize,
    last: usize,
    letter: char,
    password: String,
}

impl Password {
    fn check_policy1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();

        count >= self.first && count <= self.last
    }

    fn check_policy2(&self) -> bool {
        let first_letter = self.password.chars().nth(self.first - 1).unwrap();
        let last_letter = self.password.chars().nth(self.last - 1).unwrap();

        first_letter == self.letter && last_letter != self.letter
            || first_letter != self.letter && last_letter == self.letter
    }
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        let cap = RE.captures(value).unwrap();

        Password {
            first: cap[1].parse().unwrap(),
            last: cap[2].parse().unwrap(),
            letter: cap[3].chars().next().unwrap(),
            password: cap[4].to_owned(),
        }
    }
}

fn parse(input: &str) -> Vec<Password> {
    input.lines().map(Password::from).collect()
}

fn part1(input: &str) -> usize {
    parse(input).iter().filter(|p| p.check_policy1()).count()
}

fn part2(input: &str) -> usize {
    parse(input).iter().filter(|p| p.check_policy2()).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1);
    }
}
