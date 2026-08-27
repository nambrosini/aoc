use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq)]
enum Winner {
    Player1,
    Player2,
}

fn parse(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let parse_deck = |s: &str| s.lines().skip(1).map(|x| x.parse().unwrap()).collect();

    (parse_deck(p1), parse_deck(p2))
}

fn calc_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| card * (i + 1))
        .sum()
}

fn part1(input: &str) -> usize {
    let (mut p1, mut p2) = parse(input);

    while !p1.is_empty() && !p2.is_empty() {
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    calc_score(if p2.is_empty() { &p1 } else { &p2 })
}

fn recursive_combat(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Winner {
    let mut seen: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if !seen.insert((p1.clone(), p2.clone())) {
            return Winner::Player1;
        }

        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        let round_winner = if c1 <= p1.len() && c2 <= p2.len() {
            let mut sub1: VecDeque<usize> = p1.iter().take(c1).copied().collect();
            let mut sub2: VecDeque<usize> = p2.iter().take(c2).copied().collect();
            recursive_combat(&mut sub1, &mut sub2)
        } else if c1 > c2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        if round_winner == Winner::Player1 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    if p2.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

fn part2(input: &str) -> usize {
    let (mut p1, mut p2) = parse(input);

    match recursive_combat(&mut p1, &mut p2) {
        Winner::Player1 => calc_score(&p1),
        Winner::Player2 => calc_score(&p2),
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/22.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 306);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 291);
    }
}
