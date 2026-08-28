fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for l in input.lines() {
        let mut split = l.split("   ");
        let l = split.next().unwrap();
        let r = split.next().unwrap();

        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    (left, right)
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    let mut sum = 0;

    for (x, y) in right.iter().zip(left.iter()) {
        sum += (x - y).abs();
    }

    sum
}

fn part2(input: &str) -> u32 {
    let (left, right) = parse(input);

    let mut sum = 0;

    for l in left {
        sum += l * right.iter().filter(|&&x| x == l).count() as i32;
    }

    sum as u32
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
