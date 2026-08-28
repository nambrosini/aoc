use std::collections::HashSet;

use adventofcode::util::grid::{Find, Grid, Inbound, Parse};
use itertools::Itertools;

fn part1(input: &str) -> i64 {
    let grid: Grid<char> = Grid::parse(input);
    let symbols: Vec<&char> = grid
        .iter()
        .flatten()
        .filter(|e| e != &&'.')
        .unique()
        .collect();

    let mut set = HashSet::new();
    for symbol in symbols {
        let positions = grid.find_all(symbol);
        let combi: Vec<_> = positions.iter().permutations(2).collect();
        for c in combi {
            let dist = c[0].distance_2d(c[1]);
            let antinode = *c[0] - dist;
            if grid.inbound(&antinode) {
                set.insert(antinode);
            }
        }
    }

    set.len() as i64
}

fn part2(input: &str) -> usize {
    let grid: Grid<char> = Grid::parse(input);
    let mut new_grid: Grid<char> = grid.clone();
    let symbols: Vec<&char> = grid
        .iter()
        .flatten()
        .filter(|e| e != &&'.')
        .unique()
        .collect();

    let mut set = HashSet::new();
    for symbol in symbols {
        let positions = grid.find_all(symbol);
        let combi: Vec<_> = positions.iter().permutations(2).collect();
        for c in combi {
            let dist = c[0].distance_2d(c[1]);
            let mut current = *c[0] + dist;
            while grid.inbound(&current) {
                new_grid[current.x()][current.y()] = '#';
                set.insert(current);
                current += dist;
            }
        }
    }

    set.len()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 34);
    }
}
