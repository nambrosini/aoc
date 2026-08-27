#[derive(Clone, Copy)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl From<&str> for Sign {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

impl Sign {
    fn get_move(&self, res: Res) -> Self {
        match (self, res) {
            (s, Res::Draw) => *s,
            (Sign::Rock, Res::Win) => Sign::Paper,
            (Sign::Rock, Res::Loss) => Sign::Scissor,
            (Sign::Paper, Res::Win) => Sign::Scissor,
            (Sign::Paper, Res::Loss) => Sign::Rock,
            (Sign::Scissor, Res::Win) => Sign::Rock,
            (Sign::Scissor, Res::Loss) => Sign::Paper,
        }
    }

    fn outcome_against(&self, other: Sign) -> Res {
        match (self, other) {
            (Sign::Rock, Sign::Scissor)
            | (Sign::Paper, Sign::Rock)
            | (Sign::Scissor, Sign::Paper) => Res::Win,
            (a, b) if *a as usize == b as usize => Res::Draw,
            _ => Res::Loss,
        }
    }
}

#[derive(Clone, Copy)]
enum Res {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<&str> for Res {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once(' ').unwrap()).collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|&(a, b)| {
            let (a, b): (Sign, Sign) = (a.into(), b.into());
            b as usize + b.outcome_against(a) as usize
        })
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|&(a, b)| {
            let a: Sign = a.into();
            let res: Res = b.into();
            a.get_move(res) as usize + res as usize
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
