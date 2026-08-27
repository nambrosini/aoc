fn parse(input: &str) -> Vec<u32> {
    let mut adapters: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort_unstable();
    adapters
}

fn part1(input: &str) -> u32 {
    let adapters = parse(input);

    let diffs: Vec<u32> = adapters.windows(2).map(|w| w[1] - w[0]).collect();
    let diff1 = diffs.iter().filter(|&&x| x == 1).count() as u32;
    let diff3 = diffs.iter().filter(|&&x| x == 3).count() as u32;

    match adapters[0] {
        1 => (diff1 + 1) * (diff3 + 1),
        3 => diff1 * (diff3 + 2),
        _ => panic!(),
    }
}

fn part2(input: &str) -> u64 {
    let adapters = parse(input);

    let max = *adapters.last().unwrap() as usize;
    let mut combinations = vec![0u64; max + 3];
    combinations[max] = 1;

    for rating in adapters.iter().rev().skip(1) {
        for i in 1..=3 {
            combinations[*rating as usize] += combinations[(rating + i) as usize];
        }
    }
    for i in 1..=3 {
        combinations[0] += combinations[i];
    }

    combinations[0]
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 8);
    }
}
