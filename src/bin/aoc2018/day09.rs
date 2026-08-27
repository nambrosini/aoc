use regex::Regex;
use std::collections::VecDeque;

fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"(?P<players>\d+)[\sa-z;]+(?P<points>\d+)").unwrap();

    let caps = re.captures(input).unwrap();

    let players = caps["players"].parse().unwrap();
    let points = caps["points"].parse().unwrap();

    (players, points)
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    solve(input.0, input.1)
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    solve(input.0, input.1 * 100)
}

/// The current marble is always kept at the back of `circle`. Rotating left
/// by 1 moves the front marble to the back, i.e. advances one position
/// clockwise. This keeps every operation O(1) amortized instead of the
/// O(n) `Vec::insert`/`remove` a naive version needs.
fn solve(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut circle: VecDeque<usize> = VecDeque::with_capacity(last_marble + 1);
    circle.push_back(0);

    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            let removed = circle.pop_back().unwrap();
            scores[marble % players] += marble + removed;
            circle.rotate_left(1);
        } else {
            circle.rotate_left(1);
            circle.push_back(marble);
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(32, solve(9, 25));
        assert_eq!(8317, solve(10, 1618));
        assert_eq!(146373, solve(13, 7999));
        assert_eq!(2764, solve(17, 1104));
        assert_eq!(54718, solve(21, 6111));
        assert_eq!(37305, solve(30, 5807));
    }
}
