use advent_of_code::util::grid::{Grid, Parse};
use advent_of_code::util::position::Position;
use std::fmt::{Display, Formatter};
advent_of_code::solution!(11);

struct Input {
    vals: Grid<Pixel>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Pixel {
    #[default]
    Empty,
    Galaxy,
}

impl From<char> for Pixel {
    fn from(value: char) -> Self {
        match value {
            '.' => Pixel::Empty,
            '#' => Pixel::Galaxy,
            _ => unreachable!(),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Empty => '.',
                Pixel::Galaxy => '#',
            }
        )
    }
}

fn parse(input: &str) -> Input {
    Input {
        vals: Grid::parse(input),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let grid = parse(input).vals;
    let galaxies = expand_galaxies(&get_galaxies_pos(&grid), 2);
    Some(calc_distance_sum(galaxies))
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = parse(input).vals;
    let galaxies = expand_galaxies(&get_galaxies_pos(&grid), 1_000_000);
    Some(calc_distance_sum(galaxies))
}

fn calc_distance_sum(galaxies: Vec<Position>) -> i64 {
    let mut sum = 0;

    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            let dist = manhattan(g1, g2);
            sum += dist;
        }
    }

    sum
}

fn manhattan(g1: &Position, g2: &Position) -> i64 {
    (g1.x - g2.x).abs() + (g1.y - g2.y).abs()
}

fn expand_galaxies(map: &[Position], times: i64) -> Vec<Position> {
    let mut map = map.to_vec();
    let mut new_map = vec![];
    let max = map.iter().max_by_key(|pos| pos.x).unwrap().x;

    let mut new_x = 0;
    for x in 0..=max {
        if map.iter().any(|pos| pos.x == x) {
            for pos in map.iter().filter(|pos| pos.x == x) {
                new_map.push(Position::new(new_x, pos.y));
            }
        } else {
            new_x += times - 1;
        }
        new_x += 1;
    }

    map = new_map.clone();
    new_map.clear();
    let mut new_y = 0;
    for y in 0..=max {
        if map.iter().any(|pos| pos.y == y) {
            for pos in map.iter().filter(|pos| pos.y == y) {
                new_map.push(Position::new(pos.x, new_y));
            }
        } else {
            new_y += times - 1;
        }
        new_y += 1;
    }

    new_map
}

fn get_galaxies_pos(grid: &Grid<Pixel>) -> Vec<Position> {
    let mut galaxies = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if e == &Pixel::Galaxy {
                galaxies.push(Position::new(i as i64, j as i64));
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let grid = parse(&advent_of_code::template::read_file("examples", DAY)).vals;
        let galaxies = get_galaxies_pos(&grid);

        let galaxies = expand_galaxies(&galaxies, 10);
        assert_eq!(calc_distance_sum(galaxies), 1030);
    }
}
