use std::fmt::{Debug, Display};

use adventofcode::util::grid::Grid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty,
    PaperRoll,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '@' => Cell::PaperRoll,
            _ => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "@"),
            Cell::PaperRoll => write!(f, "."),
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::<Cell>::parse_from_chars(input);

    let neighbours_count = grid.count_neighbours(Cell::PaperRoll);

    neighbours_count
        .iter()
        .flatten()
        .filter(|c| c < &&4)
        .count()
}

fn part2(input: &str) -> u64 {
    let mut grid = Grid::<Cell>::parse_from_chars(input);
    let mut neighbours_count = grid.count_neighbours(Cell::PaperRoll);
    let mut removed = 0;

    while neighbours_count
        .iter()
        .flatten()
        .filter(|c| c < &&4)
        .count()
        != 0
    {
        let mut grid_clone = grid.clone();
        for (i, row) in neighbours_count.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell < &4 {
                    grid_clone[i][j] = Cell::Empty;
                    removed += 1;
                }
            }
        }
        grid = grid_clone;
        neighbours_count = grid.count_neighbours(Cell::PaperRoll);
    }

    removed
}

fn main() {
    let input = std::fs::read_to_string("data/input/2025/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 43);
    }
}
