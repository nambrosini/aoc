fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let depths = parse(input);

    depths.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(input: &str) -> usize {
    let depths = parse(input);

    let sums: Vec<usize> = depths.windows(3).map(|w| w.iter().sum()).collect();

    sums.windows(2).filter(|w| w[1] > w[0]).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 5);
    }
}
