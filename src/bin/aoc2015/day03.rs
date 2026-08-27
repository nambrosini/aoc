use std::collections::HashSet;

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut pos = (0, 0);

    let mut visited: HashSet<(i32, i32)> = input
        .iter()
        .map(|next| {
            pos = match next {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                '^' => (pos.0, pos.1 + 1),
                'v' => (pos.0, pos.1 - 1),
                _ => unreachable!(),
            };

            pos
        })
        .collect();

    visited.insert((0, 0));
    visited.len()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut santa_pos = (0i32, 0i32);
    let mut robo_pos = (0i32, 0i32);

    let mut visited: HashSet<(i32, i32)> = input
        .iter()
        .enumerate()
        .map(|(i, next)| {
            let m = match next {
                '>' => (1, 0),
                '<' => (-1, 0),
                '^' => (0, 1),
                'v' => (0, -1),
                _ => unreachable!(),
            };

            if i % 2 == 1 {
                santa_pos = (santa_pos.0 + m.0, santa_pos.1 + m.1);
                santa_pos
            } else {
                robo_pos = (robo_pos.0 + m.0, robo_pos.1 + m.1);
                robo_pos
            }
        })
        .collect();

    visited.insert((0, 0));
    visited.len()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(">"), 2);
        assert_eq!(part1("^>v<"), 4);
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("^v"), 3);
        assert_eq!(part2("^>v<"), 3);
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}
