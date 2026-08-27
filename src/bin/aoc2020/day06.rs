use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|group| {
            group
                .iter()
                .fold(HashSet::new(), |acc, p| acc.union(p).copied().collect())
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|group| {
            let mut iter = group.iter();
            let first = iter.next().unwrap().clone();
            iter.fold(first, |acc, p| acc.intersection(p).copied().collect())
                .len()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
