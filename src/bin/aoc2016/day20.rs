use itertools::Itertools;

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.split('-').collect_vec())
        .map(|l| (l[0].parse().unwrap(), l[1].parse().unwrap()))
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut index = 0;

    loop {
        let mut blocked = false;
        for (start, end) in &input {
            if index >= *start && index <= *end {
                blocked = true;
                break;
            }
        }
        if !blocked {
            return index;
        }
        index += 1;
    }
}

fn part2(input: &str) -> usize {
    const MAX: usize = 4294967295;

    count_allowed_ips(&parse(input), MAX)
}

fn count_allowed_ips(ranges: &[(usize, usize)], max: usize) -> usize {
    let mut input = ranges.to_vec();
    input.sort_unstable();

    let mut input_iter = input.iter();

    let mut count = 0;
    let mut index = 0;
    let mut next = input_iter.next();

    loop {
        if let Some(next_u) = next {
            if index < next_u.0 {
                count += next_u.0 - index - 1;
                index = next_u.1;
            } else if index >= next_u.0 && index <= next_u.1 {
                index = next_u.1;
            }
            next = input_iter.next();
        } else {
            count += max - index;
            break;
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/20.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5-8
0-2
4-7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_allowed_ips(&parse(SAMPLE), 9), 2);
    }
}
