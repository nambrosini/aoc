fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);

    for i in &input {
        if input.contains(&(2020 - i)) {
            return i * (2020 - i);
        }
    }

    unreachable!();
}

fn part2(input: &str) -> i32 {
    let input = parse(input);

    for i in &input {
        for j in &input {
            if input.contains(&(2020 - i - j)) {
                return i * j * (2020 - i - j);
            }
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1721
979
366
299
675
1456";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 514579);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 241861950);
    }
}
