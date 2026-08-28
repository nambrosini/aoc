use std::collections::HashMap;

type Stones = HashMap<i64, i64>;

fn parse(input: &str) -> Stones {
    let v: Vec<i64> = input
        .strip_suffix("\n")
        .unwrap()
        .split(' ')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();
    let mut map = HashMap::new();
    for x in v {
        let entry = map.entry(x).or_insert(0);
        *entry += 1;
    }
    map
}

fn part1(input: &str) -> i64 {
    let input = parse(input);
    calc(input, 25)
}

fn part2(input: &str) -> i64 {
    let input = parse(input);
    calc(input, 75)
}

fn calc(input: Stones, cycles: usize) -> i64 {
    let mut input = input.clone();
    for _ in 0..cycles {
        let mut new: Stones = Stones::new();
        for (stone, n) in input {
            let digits = (stone as f64).log10() as u32 + 1;
            if stone == 0 {
                let entry = new.entry(1).or_insert(0);
                *entry += n;
            } else if digits % 2 == 0 {
                let entry = new.entry(stone / 10i64.pow(digits / 2)).or_insert(0);
                *entry += n;
                let entry = new.entry(stone % 10i64.pow(digits / 2)).or_insert(0);
                *entry += n;
            } else {
                let entry = new.entry(stone * 2024).or_insert(0);
                *entry += n;
            }
        }
        input = new;
    }

    input.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 55312);
    }
}
