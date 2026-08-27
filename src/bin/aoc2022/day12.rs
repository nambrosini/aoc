use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Grid<T> = Vec<Vec<T>>;
type Position = (i32, i32);

fn parse(input: &str) -> Grid<char> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_character(map: &Grid<char>, searched: char) -> Vec<Position> {
    map.iter()
        .flatten()
        .enumerate()
        .filter(|(_, c)| c == &&searched)
        .map(|(i, _)| {
            (
                i as i32 / map[0].len() as i32,
                i as i32 % map[0].len() as i32,
            )
        })
        .collect()
}

fn heuristic(current: Position, end: Position) -> i32 {
    (end.0 - current.0).abs() + (end.1 - current.1).abs()
}

fn neighbors(grid: &Grid<char>, current: Position) -> Vec<Position> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(|p| (p.0 + current.0, p.1 + current.1))
        .filter(|p| {
            (0..grid.len() as i32).contains(&p.0) && (0..grid[0].len() as i32).contains(&p.1)
        })
        .filter(|p| {
            grid[current.0 as usize][current.1 as usize] as u8 + 1
                >= grid[p.0 as usize][p.1 as usize] as u8
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    position: Position,
    g: i32,
    f: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path_len(grid: &Grid<char>, start: Position, end: Position) -> Option<usize> {
    let mut open = BinaryHeap::new();
    open.push(Node {
        position: start,
        g: 0,
        f: heuristic(start, end),
    });

    let mut best_g: Grid<i32> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    best_g[start.0 as usize][start.1 as usize] = 0;

    while let Some(current) = open.pop() {
        if current.position == end {
            return Some(current.g as usize);
        }

        if current.g > best_g[current.position.0 as usize][current.position.1 as usize] {
            continue;
        }

        for next in neighbors(grid, current.position) {
            let g = current.g + 1;
            if g < best_g[next.0 as usize][next.1 as usize] {
                best_g[next.0 as usize][next.1 as usize] = g;
                open.push(Node {
                    position: next,
                    g,
                    f: g + heuristic(next, end),
                });
            }
        }
    }

    None
}

fn normalized_grid(input: &Grid<char>, start: Position, end: Position) -> Grid<char> {
    let mut grid = input.to_vec();
    grid[start.0 as usize][start.1 as usize] = 'a';
    grid[end.0 as usize][end.1 as usize] = 'z';
    grid
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let start = find_character(&grid, 'S')[0];
    let end = find_character(&grid, 'E')[0];

    let grid = normalized_grid(&grid, start, end);

    shortest_path_len(&grid, start, end).unwrap()
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let end = find_character(&grid, 'E')[0];
    let start_marker = find_character(&grid, 'S')[0];

    let grid = normalized_grid(&grid, start_marker, end);

    let mut starts = find_character(&grid, 'a');
    starts.push(start_marker);

    starts
        .iter()
        .filter_map(|&start| shortest_path_len(&grid, start, end))
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 29);
    }
}
