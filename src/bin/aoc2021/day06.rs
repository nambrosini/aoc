fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn simulate(fish: &[usize], days: usize) -> usize {
    let mut counts = [0usize; 9];
    for &f in fish {
        counts[f] += 1;
    }

    for _ in 0..days {
        let spawning = counts[0];
        counts.rotate_left(1);
        counts[6] += spawning;
    }

    counts.iter().sum()
}

fn part1(input: &str) -> usize {
    simulate(&parse(input), 80)
}

fn part2(input: &str) -> usize {
    simulate(&parse(input), 256)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 26984457539);
    }
}
