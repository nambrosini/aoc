use std::collections::HashSet;

use adventofcode::util::{
    direction::{Direction, Rotation},
    grid::{Find, Grid, Inbound, Parse},
    position::Vec2,
};

const START: char = '^';

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);
    find_visited(&grid).len()
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);
    let start = grid.find(&START);
    let visited: HashSet<Vec2> = find_visited(&grid)
        .iter()
        .filter(|x| x != &&start)
        .cloned()
        .collect();
    visited.iter().filter(|x| is_loop(&grid, &start, x)).count()
}

fn is_loop(grid: &Grid<char>, start: &Vec2, block: &Vec2) -> bool {
    let mut current = *start;
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();

    loop {
        visited.insert((current, dir));
        let next = current + dir;
        if !grid.inbound(&next) {
            break;
        }
        if visited.contains(&(next, dir)) {
            return true;
        }
        if grid[next.x()][next.y()] == '#' || &next == block {
            dir = dir.rotate(&Rotation::Clock);
            continue;
        }
        current = next;
    }
    false
}

fn find_visited(grid: &Grid<char>) -> HashSet<Vec2> {
    let mut current = grid.find(&START);
    let mut dir = Direction::Up;
    let mut visited = HashSet::new();

    loop {
        visited.insert(current);
        let next = current + dir;
        if !grid.inbound(&next) {
            break;
        }
        if grid[next.x()][next.y()] == '#' {
            dir = dir.rotate(&Rotation::Clock);
            continue;
        }
        current = next;
    }

    visited
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
