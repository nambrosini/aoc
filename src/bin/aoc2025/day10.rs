use std::{collections::VecDeque, str::FromStr};

struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    cost: Vec<usize>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{s}");
        let split: Vec<&str> = s.split(' ').collect();
        let target: Vec<bool> = split[0][1..split[0].len() - 1]
            .chars()
            .map(|c| c == '#')
            .collect();

        let mut buttons = Vec::new();
        for i in (1..split.len() - 1) {
            let button = split[i];
            let button: Vec<usize> = button[1..button.len() - 1]
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect();
            buttons.push(button);
        }

        let last = split[split.len() - 1];
        let cost: Vec<usize> = last[1..last.len() - 1]
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();

        Ok(Machine {
            target,
            buttons,
            cost,
        })
    }
}

impl Machine {
    fn shortest_path(&self) -> usize {
        let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
        queue.push_back((vec![false; self.target.len()], 0));

        while let Some((status, steps)) = queue.pop_front() {
            for button in &self.buttons {
                let res = Machine::button_click(&status, &button);
                if res == self.target {
                    return steps + 1;
                }
                queue.push_back((res, steps + 1));
            }
        }

        unreachable!()
    }

    fn button_click(status: &[bool], button: &[usize]) -> Vec<bool> {
        let mut status = status.to_vec();

        for i in button {
            status[*i] = !status[*i];
        }

        status
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| Machine::from_str(l).unwrap())
        .map(|m| m.shortest_path())
        .sum()
}

fn part2(input: &str) -> i64 {
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
