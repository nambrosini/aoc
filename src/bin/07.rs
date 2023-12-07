use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(7);

const CARDS_ORDER: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS_ORDER_JOKER: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

#[derive(Debug)]
pub struct Input {
    vals: Vec<Hand>
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<Card>,
    weight: u32,
    bid: u64
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Card {
    card: char
}

impl Input {
    fn new(hands: Vec<Hand>, order: [char; 13]) -> Self {
        let hands = hands.iter().sorted_by(|a, b| cmp(a, b, order)).rev().cloned().collect();
        Self { vals: hands }
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u64, f: fn(&[Card]) -> u32) -> Self {
        let weight = f(&cards);
        Hand { cards, weight, bid }
    }

    fn calc_weight(cards: &[Card]) -> u32 {
        let mut map = HashMap::new();
        for card in cards {
            let entry = map.entry(card).or_insert(0);
            *entry += 1;
        }
        if map.len() == 1 {
            1
        } else if map.values().any(|x| x == &4) {
            2
        } else if map.values().any(|&x| x == 3) && map.values().any(|&x| x == 2) {
            3
        } else if map.values().any(|&x| x == 3) {
            4
        } else if map.values().filter(|&x| x == &2).count() == 2 {
            5
        } else if map.values().any(|&x| x == 2) {
            6
        } else {
            7
        }
    }

    fn calc_weight_2(cards: &[Card]) -> u32 {
        let mut map = HashMap::new();
        let mut jokers = 0;
        for card in cards {
            if card.card == 'J' {
                jokers += 1;
                continue
            }
            let entry = map.entry(card).or_insert(0);
            *entry += 1;
        }

        let mut map: Vec<i32> = map.values().copied().sorted().rev().collect();
        map.push(0);
        map[0] += jokers;

        if map[0] == 5 {
            1
        } else if map[0] == 4 {
            2
        } else if map[0] == 3 && map[1] == 2 {
            3
        } else if map[0] == 3 {
            4
        } else if map[0] == 2 && map[1] == 2 {
            5
        } else if map[0] == 2 {
            6
        } else {
            7
        }
    }
}

fn cmp(a: &Hand, b: &Hand, order: [char; 13]) -> Ordering {
    if a.weight == b.weight {
        for (c1, c2) in a.cards.iter().zip(b.cards.iter()) {
            if c1 != c2 {
                let self_pos = order.iter().position(|x| x == &c1.card).unwrap();
                let other_pos = order.iter().position(|x| x == &c2.card).unwrap();

                return self_pos.cmp(&other_pos);
            }
        }
        unreachable!()
    } else {
        a.weight.cmp(&b.weight)
    }
}

fn parse(input: &str, f: fn(&[Card]) -> u32, order: [char; 13]) -> Input {
    let vals = input.lines()
        .map(|hand| {
            let mut split = hand.split_ascii_whitespace();
            let cards: Vec<Card> = split.next().unwrap().chars().map(|c| Card { card: c}).collect();
            let bid = split.next().unwrap().parse().unwrap();
            Hand::new(cards, bid, f)
        })
        .collect();
    Input::new(vals, order)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input, Hand::calc_weight, CARDS_ORDER);
    Some(input.vals.iter()
        .enumerate()
        .map(|(i, e)| (i + 1) as u64 * e.bid)
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input, Hand::calc_weight_2, CARDS_ORDER_JOKER);
    Some(input.vals.iter()
        .enumerate()
        .map(|(i, e)| (i + 1) as u64 * e.bid)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY ));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6350));
    }
}
