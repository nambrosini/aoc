fn parse(input: &str) -> Vec<String> {
    let input: Vec<u64> = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| {
            c.to_digit(10)
                .unwrap_or_else(|| panic!("Digit not valid: {}", c)) as u64
        })
        .collect();
    let mut v = vec![];
    for (i, n) in input.iter().enumerate() {
        for _ in 0..*n {
            v.push(if i % 2 == 1 {
                String::from(".")
            } else {
                (i / 2).to_string()
            });
        }
    }
    v
}

fn part1(input: &str) -> u64 {
    let input = parse(input);
    let mut res = input.clone();

    let mut count = 0;
    'outer: for (i, x) in input.iter().enumerate().rev() {
        if x == "." {
            continue;
        }
        if i == count {
            break;
        }
        while res[count] != "." {
            if i == count {
                break 'outer;
            }
            count += 1;
        }
        res[count] = x.clone();
        res[i] = ".".to_string();
    }

    res.iter()
        .enumerate()
        .filter(|(_, x)| x != &&String::from("."))
        .fold(0, |acc, (i, x)| {
            acc + (i * x.parse::<usize>().unwrap()) as u64
        })
}

fn parse_part2(input: &str) -> Vec<(u64, String)> {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .map(|(i, c)| {
            (
                c.to_digit(10).unwrap() as u64,
                if i % 2 == 0 {
                    (i / 2).to_string()
                } else {
                    ".".to_string()
                },
            )
        })
        .filter(|(i, _)| i != &0)
        .collect()
}

fn part2(input: &str) -> u64 {
    let input = parse_part2(input);
    let mut res = input.clone();

    for (_, (c, id)) in input.iter().enumerate().rev() {
        if id == &".".to_string() {
            continue;
        }
        let pos = res
            .iter()
            .position(|(count, idx)| c <= count && idx == &".".to_string());

        if let Some(pos) = pos {
            let (count, idx) = &res[pos].clone();
            let old = res
                .iter()
                .position(|(count, idx)| count == c && idx == id)
                .unwrap();
            if pos > old {
                continue;
            }
            res[pos] = (*c, id.to_string());
            res[old] = (*c, ".".to_string());
            if c < count {
                res.insert(pos + 1, (count - c, idx.to_string()));
            }
        }
    }

    let mut checksum = 0;
    let mut count = 0;
    for (c, id) in res {
        if id == *".".to_string() {
            count += c;
            continue;
        }
        let id: u64 = id.parse().unwrap();
        for _ in 0..c {
            checksum += count * id;
            count += 1;
        }
    }

    checksum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_parse() {
        let expected: Vec<String> = "0..111....22222".chars().map(|c| c.to_string()).collect();

        assert_eq!(parse("12345"), expected);
    }

    #[test]
    fn test_parse_part2() {
        let expected: Vec<(u64, String)> = vec![
            (1, "0".to_string()),
            (2, ".".to_string()),
            (3, "1".to_string()),
            (4, ".".to_string()),
            (5, "2".to_string()),
        ];

        assert_eq!(parse_part2("12345"), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2858);
    }
}
