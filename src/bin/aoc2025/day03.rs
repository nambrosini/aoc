fn part1(input: &str) -> u64 {
    let banks: Vec<&str> = input.lines().collect();

    max_joltage(&banks, 2)
}

fn part2(input: &str) -> u64 {
    let banks: Vec<&str> = input.lines().collect();

    max_joltage(&banks, 12)
}

fn max_joltage(banks: &[&str], digits: usize) -> u64 {
    let mut sum = 0;

    for b in banks {
        if b.is_empty() {
            continue;
        }
        let mut removals = b.len() - digits;
        let mut stack: Vec<char> = Vec::new();

        for battery in b.chars() {
            while removals > 0 {
                if let Some(&last) = stack.last()
                    && last < battery
                {
                    stack.pop();
                    removals -= 1;
                    continue;
                }
                break;
            }
            stack.push(battery);
        }
        while removals > 0 {
            stack.pop();
            removals -= 1;
        }

        let res: String = stack.iter().collect();
        sum += res.parse::<u64>().unwrap();
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 3121910778619);
    }
}
