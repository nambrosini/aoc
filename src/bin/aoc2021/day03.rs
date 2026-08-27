fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> usize {
    let report = parse(input);

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..report[0].len() {
        let ones = report.iter().filter(|l| l[i] == 1).count();

        if ones > report.len() - ones {
            gamma += 1;
        } else {
            epsilon += 1;
        }
        if i < report[0].len() - 1 {
            epsilon <<= 1;
            gamma <<= 1;
        }
    }

    gamma * epsilon
}

fn part2(input: &str) -> usize {
    let report = parse(input);

    let mut o_two = report.clone();
    let mut co_two = report;

    for i in 0..o_two[0].len() {
        if o_two.len() > 1 {
            let ones = o_two.iter().filter(|l| l[i] == 1).count();
            if o_two.len() - ones <= ones {
                o_two.retain(|x| x[i] == 1);
            } else {
                o_two.retain(|x| x[i] == 0);
            }
        }

        if co_two.len() == 1 {
            continue;
        }

        let ones = co_two.iter().filter(|l| l[i] == 1).count();
        if co_two.len() - ones <= ones {
            co_two.retain(|x| x[i] == 0);
        } else {
            co_two.retain(|x| x[i] == 1);
        }
    }

    to_decimal(&o_two[0]) * to_decimal(&co_two[0])
}

fn to_decimal(bits: &[u32]) -> usize {
    bits.iter().fold(0, |acc, &b| acc * 2 + b as usize)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 230);
    }
}
