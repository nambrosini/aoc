use adventofcode::util::parse::ParseOps;

fn ways_to_win(time: u64, dist: u64) -> usize {
    (0..time)
        .map(|t| (time - t) * t)
        .filter(|d| *d > dist)
        .count()
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let (times, dist) = input.split_once('\n').unwrap();
    let times: Vec<u64> = times.iter_unsigned().collect();
    let dist: Vec<u64> = dist.iter_unsigned().collect();

    times.into_iter().zip(dist).collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|&(time, dist)| ways_to_win(time, dist))
        .product()
}

fn part2(input: &str) -> usize {
    let races = parse(input);

    let join_digits = |values: Vec<u64>| {
        values
            .iter()
            .map(u64::to_string)
            .collect::<String>()
            .parse()
            .unwrap()
    };
    let time = join_digits(races.iter().map(|&(t, _)| t).collect());
    let dist = join_digits(races.iter().map(|&(_, d)| d).collect());

    ways_to_win(time, dist)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 71503);
    }
}
