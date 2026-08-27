fn seat_id(code: &str) -> usize {
    code.chars()
        .fold(0, |acc, c| (acc << 1) | matches!(c, 'B' | 'R') as usize)
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(seat_id).collect()
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let ids = parse(input);
    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();

    (min..=max).find(|x| !ids.contains(x)).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "FBFBBFFRLR\nBFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 820);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 120);
    }
}
