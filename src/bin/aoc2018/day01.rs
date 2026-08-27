fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &str) -> i32 {
    parse(input).iter().sum()
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut v = vec![];

    let mut sum = 0;
    let mut position = 0;

    while !v.contains(&sum) {
        v.push(sum);

        sum += input[position];

        position = (position + 1) % input.len();
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("+1\n+1\n+1"), 3);
        assert_eq!(part1("+1\n+1\n-2"), 0);
        assert_eq!(part1("-1\n-2\n-3"), -6);
    }
}
