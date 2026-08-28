use std::ops::RangeInclusive;

use adventofcode::util::ranges::merge_ranges;

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut split = input.trim_end_matches("\n").split("\n\n");

    let fresh: Vec<RangeInclusive<usize>> = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| {
            let mut l = l.split("-");
            l.next().unwrap().parse().unwrap()..=l.next().unwrap().parse().unwrap()
        })
        .collect();

    let ing = split
        .next()
        .unwrap()
        .split("\n")
        .map(|l| l.parse().unwrap())
        .collect();

    (fresh, ing)
}

fn part1(input: &str) -> usize {
    let (fresh, ing) = parse(input);

    let fresh = merge_ranges(&fresh);

    ing.iter()
        .filter(|i| fresh.iter().any(|f| f.contains(i)))
        .count()
}

fn part2(input: &str) -> usize {
    let (fresh, _) = parse(input);

    merge_ranges(&fresh)
        .iter()
        .fold(0, |acc, x| acc + x.clone().count())
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 14);
    }
}
