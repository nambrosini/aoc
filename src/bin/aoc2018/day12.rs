use std::collections::HashMap;

fn parse(input: &str) -> Pots {
    input.into()
}

fn part1(input: &str) -> i64 {
    let mut pots = parse(input);
    for _ in 0..20 {
        pots.simulate();
    }

    pots.calc()
}

fn part2(input: &str) -> i64 {
    run(&mut parse(input), 50_000_000_000)
}

/// Simulates generations until the sum-of-pots delta between consecutive
/// generations stabilizes (the live pattern has settled into just shifting
/// position each generation), then extrapolates linearly to `cycles`
/// instead of literally simulating up to 50 billion generations.
fn run(pots: &mut Pots, cycles: usize) -> i64 {
    let mut prev_sum = pots.calc();
    let mut prev_delta = None;

    for i in 0..cycles {
        pots.simulate();
        let sum = pots.calc();
        let delta = sum - prev_sum;

        if prev_delta == Some(delta) {
            let remaining = (cycles - i - 1) as i64;
            return sum + delta * remaining;
        }

        prev_delta = Some(delta);
        prev_sum = sum;
    }

    pots.calc()
}

#[derive(Clone)]
struct Pots {
    pots: HashMap<i64, char>,
    states: Vec<State>,
}

impl Pots {
    fn simulate(&mut self) {
        let mut pots_clone = self.pots.clone();

        let min = *self.pots.keys().min().unwrap();
        let max = *self.pots.keys().max().unwrap();

        for i in min - 2..=max + 2 {
            let mut string = String::new();
            for j in i - 2..=i + 2 {
                if let Some(c) = self.pots.get(&j) {
                    string.push(*c);
                } else {
                    string.push('.');
                }
            }
            let val = self.get_pot_from_states(string);
            if !((i < min || i > max) && val == '.') {
                let entry = pots_clone.entry(i).or_insert('.');
                *entry = val;
            }
        }

        self.pots = pots_clone;
    }

    fn get_pot_from_states(&self, string: String) -> char {
        if let Some(x) = self.states.iter().find(|state| state.left == string) {
            x.right
        } else {
            '.'
        }
    }

    fn calc(&self) -> i64 {
        self.pots
            .iter()
            .filter(|(_, c)| c == &&'#')
            .map(|(i, _)| i)
            .sum()
    }
}

impl From<&str> for Pots {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split("\n\n").collect();

        let pots = split[0]
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .map(|(i, e)| (i as i64, e))
            .collect();
        let states = split[1].lines().map(State::from).collect();

        Self { pots, states }
    }
}

#[derive(Clone)]
struct State {
    left: String,
    right: char,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" => ").collect();
        let left = split[0].to_string();
        let right = split[1].chars().next().unwrap();

        Self { left, right }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_part1() {
        assert_eq!(325, part1(SAMPLE));
    }
}
