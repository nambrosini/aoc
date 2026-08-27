fn parse(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut calories = parse(input);
    calories.sort_unstable();

    calories.iter().rev().take(3).sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 45000);
    }
}
