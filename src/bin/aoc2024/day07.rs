fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|x| {
            let mut split = x.split(": ");
            let res = split.next().unwrap().parse().unwrap();
            let vals = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (res, vals)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let input = parse(input);

    let mut sum = 0;
    for (target, nums) in input {
        if recurse_part1(nums[0], 1, &nums, '+', target)
            || recurse_part1(nums[0], 1, &nums, '*', target)
        {
            sum += target;
        }
    }

    sum
}

fn recurse_part1(start: i64, index: usize, nums: &[i64], op: char, target: i64) -> bool {
    let res = match op {
        '+' => start + nums[index],
        '*' => start * nums[index],
        _ => unreachable!(),
    };

    if index == nums.len() - 1 {
        return res == target;
    }

    recurse_part1(res, index + 1, nums, '+', target)
        || recurse_part1(res, index + 1, nums, '*', target)
}

fn part2(input: &str) -> i64 {
    let input = parse(input);
    let mut sum = 0;
    for (target, nums) in input {
        if recurse_part2(nums[0], 1, &nums, '+', target)
            || recurse_part2(nums[0], 1, &nums, '*', target)
            || recurse_part2(nums[0], 1, &nums, '|', target)
        {
            sum += target;
        }
    }

    sum
}

fn recurse_part2(start: i64, index: usize, nums: &[i64], op: char, target: i64) -> bool {
    let res = match op {
        '+' => start + nums[index],
        '*' => start * nums[index],
        '|' => start * 10i64.pow((nums[index] as f64).log10() as u32 + 1) + nums[index],
        _ => unreachable!(),
    };

    if index == nums.len() - 1 {
        return res == target;
    }

    recurse_part2(res, index + 1, nums, '+', target)
        || recurse_part2(res, index + 1, nums, '*', target)
        || recurse_part2(res, index + 1, nums, '|', target)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 11387);
    }
}
