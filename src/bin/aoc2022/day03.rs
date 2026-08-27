use std::collections::HashSet;

type Compartment = HashSet<char>;

fn priority(c: char) -> usize {
    if c.is_ascii_uppercase() {
        (c as u8 - b'A' + 27) as usize
    } else {
        (c as u8 - b'a' + 1) as usize
    }
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(|l| {
            let mid = l.len() / 2;
            let a: Compartment = l[..mid].chars().collect();
            let b: Compartment = l[mid..].chars().collect();
            a.intersection(&b).next().map(|&c| priority(c))
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let sacks: Vec<Compartment> = parse(input).iter().map(|l| l.chars().collect()).collect();

    sacks
        .chunks(3)
        .filter_map(|group| {
            group[0]
                .iter()
                .find(|item| group[1].contains(item) && group[2].contains(item))
        })
        .map(|&c| priority(c))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 70);
    }
}
