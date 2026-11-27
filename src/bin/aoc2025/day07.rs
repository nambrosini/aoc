use std::collections::{HashSet, VecDeque};

use adventofcode::util::{
    direction::Direction,
    grid::{Find, Grid, Inbound, Parse, Read},
    position::Vec2,
};

const DIR: Direction = Direction::Down;

fn part1(input: &str) -> u64 {
    let grid: Grid<char> = Grid::parse(input);
    let start = grid.find(&'S');

    let mut split_set = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(beam) = queue.pop_front() {
        let new_beam = beam + DIR;

        if !grid.inbound(&new_beam) {
            continue;
        }

        if grid.get(&new_beam).unwrap() == '^' {
            split_set.insert(new_beam);
            let new_beam_left = new_beam + Direction::Left;
            if grid.inbound(&new_beam_left) && !queue.contains(&new_beam_left) {
                queue.push_back(new_beam_left);
            }
            let new_beam_right = new_beam + Direction::Right;
            if grid.inbound(&new_beam_right) && !queue.contains(&new_beam_right) {
                queue.push_back(new_beam_right);
            }
        } else {
            queue.push_back(new_beam);
        }
    }

    split_set.len() as u64
}

fn part2(input: &str) -> u64 {
    let grid: Grid<char> = Grid::parse(input);
    let start = grid.find(&'S');

    let mut count = 1;

    let mut queue: VecDeque<(Vec2, u64)> = VecDeque::new();
    queue.push_back((start, 1));

    while let Some(beam) = queue.pop_front() {
        let new_beam = beam.0 + DIR;

        if !grid.inbound(&new_beam) {
            continue;
        }

        if grid.get(&new_beam).unwrap() == '^' {
            count += beam.1;
            let new_beam_left = new_beam + Direction::Left;
            if grid.inbound(&new_beam_left) {
                if let Some(pos) = queue.iter().position(|b| b.0 == new_beam_left) {
                    queue[pos].1 += beam.1;
                } else {
                    queue.push_back((new_beam_left, beam.1));
                }
            }
            let new_beam_right = new_beam + Direction::Right;
            if grid.inbound(&new_beam_right) {
                if let Some(pos) = queue.iter().position(|b| b.0 == new_beam_right) {
                    queue[pos].1 += beam.1;
                } else {
                    queue.push_back((new_beam_right, beam.1));
                }
            }
        } else if let Some(pos) = queue.iter().position(|b| b.0 == new_beam) {
            queue[pos].1 += beam.1;
        } else {
            queue.push_back((new_beam, beam.1));
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 40);
    }
}
