use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(input: &str) -> (HashSet<String>, Vec<String>) {
    let mut split = input.split("\n\n");
    let first = split
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.to_string())
        .collect();
    let second = split
        .next()
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    (first, second)
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let counts = counts(&input.1, &input.0).filter(|x| *x > 0).collect_vec();
    counts.len()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let counts = counts(&input.1, &input.0).filter(|x| *x > 0).collect_vec();
    counts.iter().sum::<usize>()
}

fn counts<'a>(
    targets: &'a [String],
    patterns: &'a HashSet<String>,
) -> impl Iterator<Item = usize> + use<'a> {
    let mut known = HashMap::from([("", 1)]);

    targets
        .iter()
        .map(move |target| count(target.as_str(), patterns, &mut known))
}
fn count<'a>(t: &'a str, a: &'a HashSet<String>, known: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(r) = known.get(t) {
        return *r;
    }
    let r = a
        .iter()
        .filter_map(|p| t.strip_prefix(p))
        .map(|t| count(t, a, known))
        .sum();
    known.entry(t).and_modify(|e| *e = r).or_insert(r);
    r
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 16);
    }
}
