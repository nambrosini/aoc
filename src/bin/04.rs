use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    Some(
        input
            .iter()
            .map(|x| if *x == 0 { 0 } else { 2u32.pow(x - 1) })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let mut map: HashMap<usize, u32> = HashMap::new();
    for (i, card) in input.iter().enumerate() {
        let entry = map.entry(i).or_insert(0);
        *entry += 1;
        let c = *entry;
        for k in i + 1..i + 1 + *card as usize {
            let entry = map.entry(k).or_insert(0);
            *entry += c;
        }
    }
    Some(map.values().sum())
}

pub fn parse(input: &str) -> Vec<u32> {
    let cards = input
        .lines()
        .map(|l| {
            let card: Vec<&str> = l.split(": ").collect();
            let card = card[1];
            let numbers: String = card.replace(" |", "");
            let numbers: Vec<u32> = numbers
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let count = numbers.len();
            (count - numbers.iter().unique().count()) as u32
        })
        .collect();

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
