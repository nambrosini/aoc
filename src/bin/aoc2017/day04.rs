use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.to_owned()).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter(|l| l.len() == l.iter().cloned().collect::<HashSet<String>>().len())
        .count()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .map(|l| {
            l.iter()
                .map(|w| {
                    let mut chars: Vec<char> = w.chars().collect();
                    chars.sort_unstable();
                    chars.into_iter().collect::<String>()
                })
                .collect::<Vec<String>>()
        })
        .filter(|l| l.len() == l.iter().cloned().collect::<HashSet<String>>().len())
        .count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("aa bb cc dd ee"), 1);
        assert_eq!(part1("aa bb cc dd aa"), 0);
        assert_eq!(part1("aa bb cc dd aaa"), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("abcde fghij"), 1);
        assert_eq!(part2("abcde xyz ecdab"), 0);
        assert_eq!(part2("a ab abc abd abf abj"), 1);
        assert_eq!(part2("iiii oiii ooii oooi oooo"), 1);
        assert_eq!(part2("oiii ioii iioi iiio"), 0);
    }
}
