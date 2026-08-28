fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &str) -> u64 {
    let input = parse(input);

    let mut dial: i32 = 50;
    let mut res = 0;

    for rotation in input {
        let direction = &rotation[0..1];
        let count: i32 = rotation[1..].parse().unwrap();

        if direction == "L" {
            dial = (dial - count) % 100;
        } else {
            dial = (dial + count) % 100;
        }

        if dial == 0 {
            res += 1;
        }
    }

    res
}

fn part2(input: &str) -> i32 {
    let input = parse(input);

    let mut dial: i32 = 50;
    let mut res = 0;

    for rotation in input {
        let direction = &rotation[0..1];
        let count: i32 = rotation[1..].parse().unwrap();

        for _ in 0..count {
            if direction == "L" {
                dial -= 1;
                if dial == 0 {
                    res += 1;
                }
                if dial < 0 {
                    dial += 100;
                }
            } else {
                dial = (dial + 1) % 100;
                if dial == 0 {
                    res += 1;
                }
            }
        }
    }

    res
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
