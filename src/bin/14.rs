use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;

use advent_of_code::util::grid::Grid;

advent_of_code::solution!(14);
struct Input {
    grid: Grid<char>,
}

fn parse(input: &str) -> Input {
    let grid = input.lines().map(|l| l.chars().collect()).collect();
    Input { grid }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input).grid;
    tilt_north(&mut grid);
    Some(calc_weight(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input).grid;
    let mut visited: HashSet<u64> = HashSet::new();

    let cycle;

    let mut first_duplicate: Option<(usize, u64)> = None;
    let mut i = 0;

    loop {
        step(&mut grid);

        let hash = {
            let mut hasher = DefaultHasher::new();
            grid.hash(&mut hasher);
            hasher.finish()
        };

        if let Some(first) = &first_duplicate {
            if first.1 == hash {
                cycle = i;
                break;
            }
        } else if visited.contains(&hash) {
            first_duplicate = Some((i, hash));
        } else {
            visited.insert(hash);
        }
        i += 1;
    }

    let first = first_duplicate.unwrap();
    let cycles = (1000000000 - first.0) % (cycle - first.0);

    for _ in 1..cycles {
        step(&mut grid);
    }

    Some(calc_weight(&grid))
}

fn step(grid: &mut Grid<char>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn calc_weight(grid: &Grid<char>) -> usize {
    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += grid.len() - i;
            }
        }
    }

    sum
}

fn tilt_north(grid: &mut Grid<char>) {
    for j in 0..grid.len() {
        for i in 0..grid.len() {
            if grid[i][j] == 'O' {
                let open = find_open_spot_north(grid, i, j);
                grid[i][j] = '.';
                grid[open][j] = 'O';
            }
        }
    }
}

fn tilt_south(grid: &mut Grid<char>) {
    for j in (0..grid.len()).rev() {
        for i in (0..grid.len()).rev() {
            if grid[i][j] == 'O' {
                let open = find_open_spot_south(grid, i, j);
                grid[i][j] = '.';
                grid[open][j] = 'O';
            }
        }
    }
}

fn tilt_west(grid: &mut Grid<char>) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 'O' {
                let open = find_open_spot_west(grid, i, j);
                grid[i][j] = '.';
                grid[i][open] = 'O';
            }
        }
    }
}

fn tilt_east(grid: &mut Grid<char>) {
    for i in (0..grid.len()).rev() {
        for j in (0..grid.len()).rev() {
            if grid[i][j] == 'O' {
                let open = find_open_spot_east(grid, i, j);
                grid[i][j] = '.';
                grid[i][open] = 'O';
            }
        }
    }
}

fn find_open_spot_north(grid: &Grid<char>, i: usize, j: usize) -> usize {
    for i in (1..=i).rev() {
        if grid[i - 1][j] == '#' || grid[i - 1][j] == 'O' {
            return i;
        }
    }

    0
}

fn find_open_spot_south(grid: &Grid<char>, i: usize, j: usize) -> usize {
    for i in i..grid.len() - 1 {
        if grid[i + 1][j] == '#' || grid[i + 1][j] == 'O' {
            return i;
        }
    }

    grid.len() - 1
}

fn find_open_spot_west(grid: &Grid<char>, i: usize, j: usize) -> usize {
    for j in (1..=j).rev() {
        if grid[i][j - 1] == '#' || grid[i][j - 1] == 'O' {
            return j;
        }
    }

    0
}

fn find_open_spot_east(grid: &Grid<char>, i: usize, j: usize) -> usize {
    for j in j..grid.len() - 1 {
        if grid[i][j + 1] == '#' || grid[i][j + 1] == 'O' {
            return j;
        }
    }

    grid.len() - 1
}

#[cfg(test)]
mod tests {
    use advent_of_code::util::grid::Print;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_north() {
        let mut grid = parse(&advent_of_code::template::read_file("examples", DAY)).grid;
        let expected = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        tilt_north(&mut grid);

        assert_eq!(expected, grid.string());
    }

    #[test]
    fn test_west() {
        let mut grid = parse(&advent_of_code::template::read_file("examples", DAY)).grid;
        let expected = "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#....";

        tilt_west(&mut grid);

        assert_eq!(expected, grid.string());
    }

    #[test]
    fn test_south() {
        let mut grid = parse(&advent_of_code::template::read_file("examples", DAY)).grid;
        let expected = ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O";

        tilt_south(&mut grid);

        assert_eq!(expected, grid.string());
    }

    #[test]
    fn test_east() {
        let mut grid = parse(&advent_of_code::template::read_file("examples", DAY)).grid;
        let expected = "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#....";

        tilt_east(&mut grid);

        assert_eq!(expected, grid.string());
    }
}
