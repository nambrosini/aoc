fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> u32 {
    let input = parse(input);
    let mut checksum = 0u32;

    for l in &input {
        checksum += l.iter().max().unwrap() - l.iter().min().unwrap();
    }

    checksum
}

fn part2(input: &str) -> u32 {
    let input = parse(input);
    let mut checksum = 0u32;

    for l in &input {
        let mut l = l.clone();
        l.sort_unstable();

        for i in 0..l.len() - 1 {
            for j in i + 1..l.len() {
                if l[j] % l[i] == 0 {
                    checksum += l[j] / l[i];
                }
            }
        }
    }

    checksum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8"), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5"), 9);
    }
}
