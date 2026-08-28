use std::collections::HashSet;

use adventofcode::util::{
    grid::{Find, Grid, Inbound, Parse},
    position::Vec2,
};

fn part1(input: &str) -> i64 {
    let grid = Grid::parse(input);
    let starts = grid.find_all(&0);
    let mut sum = 0;
    for start in starts {
        let mut set = HashSet::new();
        trailheads(&grid, start, &mut set);
        sum += set.len();
    }
    sum as i64
}

fn trailheads(grid: &Grid<i64>, start: Vec2, visited: &mut HashSet<Vec2>) {
    let current_value = grid[start.x()][start.y()];
    if grid[start.x()][start.y()] == 9 {
        visited.insert(start);
        return;
    }

    for dir in Vec2::DIRECTIONS {
        let new_pos = start + dir;
        if !grid.inbound(&new_pos) {
            continue;
        }
        let new_value = grid[new_pos.x()][new_pos.y()];
        if new_value == current_value + 1 {
            trailheads(grid, new_pos, visited);
        }
    }
}

fn part2(input: &str) -> i64 {
    let grid = Grid::parse(input);
    let starts = grid.find_all(&0);
    let mut sum = 0;
    for start in starts {
        let v = distinct_trailheads(&grid, start);
        sum += v;
    }
    sum
}

fn distinct_trailheads(grid: &Grid<i64>, start: Vec2) -> i64 {
    let current_value = grid[start.x()][start.y()];
    if grid[start.x()][start.y()] == 9 {
        return 1;
    }

    let mut sum = 0;
    for dir in Vec2::DIRECTIONS {
        let new_pos = start + dir;
        if !grid.inbound(&new_pos) {
            continue;
        }
        let new_value = grid[new_pos.x()][new_pos.y()];
        if new_value == current_value + 1 {
            sum += distinct_trailheads(grid, new_pos);
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 81);
    }
}
