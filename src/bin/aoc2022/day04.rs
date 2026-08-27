use std::ops::RangeInclusive;

fn parse(input: &str) -> Vec<(RangeInclusive<usize>, RangeInclusive<usize>)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let parse_range = |r: &str| {
                let (start, end) = r.split_once('-').unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            };
            (parse_range(a), parse_range(b))
        })
        .collect()
}

fn contains(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.end())
        || (r2.start() <= r1.start() && r2.end() >= r1.end())
}

fn overlaps(r1: &RangeInclusive<usize>, r2: &RangeInclusive<usize>) -> bool {
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(r1, r2)| contains(r1, r2))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(r1, r2)| overlaps(r1, r2))
        .count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
