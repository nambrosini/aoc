use aoc_util::parse::ParseOps;
use itertools::Itertools;
advent_of_code::solution!(6);

pub struct Input {
    races: Vec<[u64; 2]>,
}

pub fn parse(input: &str) -> Input {
    let chunks: Vec<_> = input.lines().collect();
    let times: Vec<u64> = chunks[0].iter_unsigned().collect();
    let dist: Vec<u64> = chunks[1].iter_unsigned().collect();

    Input {
        races: times.iter().zip(dist).map(|(&t, d)| [t, d]).collect(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut total = 1;
    for &[time, dist] in &input.races {
        total *= (0..time)
            .map(|t| (time - t) * t)
            .filter(|d| d > &dist)
            .count() as u64
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let time = get_number(&input, 0);
    let dist = get_number(&input, 1);

    Some(
        (0..time)
            .map(|t| (time - t) * t)
            .filter(|d| d > &dist)
            .count() as u64,
    )
}

fn get_number(input: &Input, index: usize) -> u64 {
    input
        .races
        .iter()
        .map(|x| x[index].to_string())
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
