fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &str) -> i64 {
    let input = parse(input);

    input.iter().map(|x| x / 3 - 2).sum()
}

fn part2(input: &str) -> i64 {
    let input = parse(input);

    input.iter().fold(0, |acc, x| {
        let mut sum = 0;
        let mut num = *x;
        while num > 6 {
            num = num / 3 - 2;
            sum += num
        }
        acc + sum
    })
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
