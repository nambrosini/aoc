fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn min_fuel(positions: &[i32], cost: impl Fn(i32) -> i32) -> i32 {
    let max = *positions.iter().max().unwrap();

    (0..=max)
        .map(|target| positions.iter().map(|&p| cost((p - target).abs())).sum())
        .min()
        .unwrap()
}

fn part1(input: &str) -> i32 {
    min_fuel(&parse(input), |d| d)
}

fn part2(input: &str) -> i32 {
    min_fuel(&parse(input), |d| d * (d + 1) / 2)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 168);
    }
}
