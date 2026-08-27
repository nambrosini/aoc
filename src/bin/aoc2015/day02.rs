fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|x| {
            x.split('x')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|x| {
            let mut x = x;
            x.sort_unstable();
            x
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);

    input
        .iter()
        .map(|list| {
            2 * list[0] * list[1]
                + 2 * list[1] * list[2]
                + 2 * list[2] * list[0]
                + list[0] * list[1]
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let input = parse(input);

    input
        .iter()
        .map(|l| 2 * l[0] + 2 * l[1] + l[0] * l[1] * l[2])
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }
}
