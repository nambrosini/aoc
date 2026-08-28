use regex::Regex;

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mults: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    mults.iter().map(|m| eval_mul(m)).sum()
}

fn part2(input: &str) -> u32 {
    let re = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();

    let mults: Vec<&str> = re.find_iter(input).map(|m| m.as_str()).collect();

    let mut res = 0;
    let mut doo = true;

    for statement in mults {
        if statement.starts_with("don't") {
            doo = false;
        } else if statement.starts_with("do") {
            doo = true;
        } else if doo {
            res += eval_mul(statement);
        }
    }

    res
}

fn eval_mul(statement: &str) -> u32 {
    statement[4..statement.len() - 1]
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .product::<u32>()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 48);
    }
}
