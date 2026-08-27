use itertools::Itertools;
use std::cmp;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    get_lowest_quantum(&parse(input), 3)
}

fn part2(input: &str) -> usize {
    get_lowest_quantum(&parse(input), 4)
}

fn get_lowest_quantum(weights: &[usize], n: usize) -> usize {
    let expected_weight: usize = weights.iter().sum::<usize>() / n;
    let mut min = usize::MAX;
    let mut should_break = false;

    for i in 1..weights.len() {
        for group_weights in weights.iter().combinations(i) {
            if group_weights.iter().copied().sum::<usize>() == expected_weight {
                should_break = true;

                let quantum = group_weights.iter().copied().product();
                min = cmp::min(min, quantum);
            }
        }

        if should_break {
            break;
        }
    }

    min
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/24.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [usize; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    #[test]
    fn test_part1() {
        assert_eq!(get_lowest_quantum(&SAMPLE, 3), 99);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_lowest_quantum(&SAMPLE, 4), 44);
    }
}
