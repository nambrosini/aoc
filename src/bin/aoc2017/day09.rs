fn score_and_garbage(input: &str) -> (i32, i32) {
    let mut sum = 0;
    let mut garbage_count = 0;
    let mut group_level = 0;
    let mut is_garbage = false;
    let mut is_canceled = false;

    for character in input.trim().chars() {
        if is_canceled {
            is_canceled = false;
            continue;
        }
        if is_garbage {
            match character {
                '!' => {
                    is_canceled = true;
                }
                '>' => {
                    is_garbage = false;
                }
                _ => {
                    garbage_count += 1;
                }
            }
            continue;
        }
        match character {
            '{' => {
                group_level += 1;
            }
            '}' => {
                sum += group_level;
                group_level -= 1;
            }
            '<' => {
                is_garbage = true;
            }
            _ => {}
        }
    }

    (sum, garbage_count)
}

fn part1(input: &str) -> i32 {
    score_and_garbage(input).0
}

fn part2(input: &str) -> i32 {
    score_and_garbage(input).1
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("{}"), 1);
        assert_eq!(part1("{{{}}}"), 6);
        assert_eq!(part1("{{},{}}"), 5);
        assert_eq!(part1("{{{},{},{{}}}}"), 16);
        assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("<>"), 0);
        assert_eq!(part2("<random characters>"), 17);
        assert_eq!(part2("<<<<>"), 3);
        assert_eq!(part2("<{!>}>"), 2);
        assert_eq!(part2("<!!>"), 0);
        assert_eq!(part2("<!!!>>"), 0);
        assert_eq!(part2("<{o\"i!a,<{i<a>"), 10);
    }
}
