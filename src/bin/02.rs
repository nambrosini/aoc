use std::collections::HashMap;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let limits = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    Some(
        input
            .iter()
            .enumerate()
            .filter(|(_, g)| g.iter().all(|h| check_hand(&limits, h)))
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let games: Vec<Hand> = input
        .iter()
        .map(|g| g.iter().flatten().copied().collect())
        .collect();

    let mut res: u32 = 0;
    for game in games {
        let mut r = 1u32;
        for color in [Color::Red, Color::Green, Color::Blue] {
            r *= game
                .iter()
                .filter(|(col, _)| col == &color)
                .max_by_key(|(_, c)| c)
                .unwrap()
                .1;
        }
        res += r;
    }
    Some(res)
}

pub fn parse(input: &str) -> Vec<Game> {
    let mut res = Vec::new();
    for l in input.lines() {
        let input: &str = l.split(": ").nth(1).unwrap();
        let split_hands: Vec<&str> = input.split("; ").collect();
        let mut game = Game::new();
        for hand in split_hands {
            let mut h: Hand = Hand::new();
            let cubes: Vec<&str> = hand.split(", ").collect();
            for cube in cubes {
                let mut split = cube.split(' ');
                let count = split.next().unwrap().parse().unwrap();
                let color = Color::new(split.next().unwrap());
                h.push((color, count));
            }
            game.push(h);
        }
        res.push(game);
    }
    res
}

type Game = Vec<Hand>;
type Hand = Vec<(Color, u32)>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Color {
    Blue,
    Red,
    Green,
}

impl Color {
    fn new(color: &str) -> Self {
        match color {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => unreachable!(),
        }
    }
}

fn check_hand(limit: &HashMap<Color, u32>, hand: &Hand) -> bool {
    for (color, count) in hand {
        if count > &limit[&color] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
