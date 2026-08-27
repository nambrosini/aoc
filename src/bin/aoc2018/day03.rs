use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<Square> {
    input.lines().map(Square::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    for s in &input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map.entry((s.x + i, s.y + j)).or_insert(0);
                *e += 1;
            }
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut map: HashMap<(usize, usize), String> = HashMap::new();

    let mut destroyed: HashSet<usize> = HashSet::new();

    for s in &input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map
                    .entry((s.x + i, s.y + j))
                    .or_insert_with(|| ".".to_string());
                if e == "." {
                    *e = format!("{}", s.id);
                } else if e == "X" {
                    destroyed.insert(s.id);
                } else {
                    destroyed.insert(e.clone().parse().unwrap());
                    destroyed.insert(s.id);
                    *e = "X".to_string();
                }
            }
        }
    }

    input
        .iter()
        .map(|s| s.id)
        .find(|s| !destroyed.contains(s))
        .unwrap()
}

struct Square {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Square {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        Self {
            id: cap[1].parse().unwrap(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            width: cap[4].parse().unwrap(),
            height: cap[5].parse().unwrap(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 3);
    }
}
