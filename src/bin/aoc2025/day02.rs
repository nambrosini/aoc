use std::ops::RangeInclusive;

use fancy_regex::Regex;

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .filter_map(|range_str| {
            let (start, end) = range_str.split_once('-')?;
            Some(start.parse().ok()?..=end.parse().ok()?)
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\1$").unwrap();
    let input = parse(input);
    let mut sum = 0;
    for r in input {
        for id in r {
            if re.is_match(&id.to_string()).unwrap() {
                sum += id;
            }
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let re = Regex::new(r"^(\d+)\1+$").unwrap();
    let input = parse(input);
    let mut sum = 0;
    for r in input {
        for id in r {
            if re.is_match(&id.to_string()).unwrap() {
                sum += id;
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4174379265);
    }
}
