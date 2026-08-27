use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn play(starting: &[usize], rounds: usize) -> usize {
    let mut last_seen: HashMap<usize, usize> = HashMap::new();

    for (i, &n) in starting.iter().take(starting.len() - 1).enumerate() {
        last_seen.insert(n, i + 1);
    }

    let mut last_number = *starting.last().unwrap();

    for turn in starting.len()..rounds {
        let next = match last_seen.get(&last_number) {
            Some(&prev_turn) => turn - prev_turn,
            None => 0,
        };
        last_seen.insert(last_number, turn);
        last_number = next;
    }

    last_number
}

fn part1(input: &str) -> usize {
    play(&parse(input), 2020)
}

fn part2(input: &str) -> usize {
    play(&parse(input), 30_000_000)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0,3,6"), 436);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0,3,6"), 175594);
    }
}
