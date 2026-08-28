fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| l.split(' ').map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut res = 0;
    for report in input {
        if is_safe(&report) {
            res += 1;
        }
    }
    res
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut res = 0;
    for report in input {
        let mut safe = false;
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if is_safe(&report) {
                safe = true;
                break;
            }
        }
        if safe {
            res += 1;
        }
    }
    res
}

fn is_safe(report: &[i32]) -> bool {
    let x: Vec<_> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(x, y)| x - y)
        .collect();
    x.iter().all(|x| (1..=3).contains(x)) || x.iter().all(|x| (-3..=-1).contains(x))
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
