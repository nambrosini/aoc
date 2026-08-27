fn part1(input: &str) -> i32 {
    input.chars().fold(0, |sum, c| {
        sum + match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        }
    })
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;

    for (i, c) in input.chars().enumerate() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };

        if sum == -1 {
            return i as i32 + 1;
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
