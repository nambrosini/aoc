use std::cmp::Ordering;

fn parse(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let map = split[0]
        .lines()
        .map(|l| {
            let s: Vec<i64> = l.split('|').map(|x| x.parse().unwrap()).collect();
            (s[0], s[1])
        })
        .collect();

    let orders = split[1]
        .lines()
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();
    (map, orders)
}

fn part1(input: &str) -> i64 {
    let (rules, orders) = parse(input);
    is_valid(&rules, &orders[0]);

    orders
        .iter()
        .filter(|x| is_valid(&rules, x))
        .map(|x| x[x.len() / 2])
        .sum()
}

fn is_valid(rules: &[(i64, i64)], order: &[i64]) -> bool {
    order[..order.len() - 1]
        .iter()
        .zip(order.iter().skip(1))
        .all(|(&a, &b)| rules.contains(&(a, b)))
}

fn part2(input: &str) -> i64 {
    let (rules, orders) = parse(input);

    orders
        .iter()
        .filter(|x| !is_valid(&rules, x))
        .map(|x| {
            let mut x: Vec<i64> = x.to_vec();
            x.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            x.iter().rev().copied().collect::<Vec<i64>>()
        })
        .map(|x| x[x.len() / 2])
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 123);
    }
}
