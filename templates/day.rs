fn part1(input: &str) -> i64 {
    todo!()
}

fn part2(input: &str) -> i64 {
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("data/input/__YEAR__/__DAY__.txt").unwrap();

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
