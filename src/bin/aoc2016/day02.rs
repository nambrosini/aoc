use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn part1(input: &str) -> String {
    let input = parse(input);
    let mut grid = HashMap::new();

    for i in 0..3i32 {
        for j in 0..3i32 {
            grid.insert((i, j), (i * 3 + j + 1).to_string());
        }
    }

    get_res(&input, &grid)
}

fn part2(input: &str) -> String {
    let input = parse(input);
    let mut grid = HashMap::new();

    grid.insert((0, 2), "1".into());
    grid.insert((1, 1), "2".into());
    grid.insert((1, 2), "3".into());
    grid.insert((1, 3), "4".into());
    grid.insert((2, 0), "5".into());
    grid.insert((2, 1), "6".into());
    grid.insert((2, 2), "7".into());
    grid.insert((2, 3), "8".into());
    grid.insert((2, 4), "9".into());
    grid.insert((3, 1), "A".into());
    grid.insert((3, 2), "B".into());
    grid.insert((3, 3), "C".into());
    grid.insert((4, 2), "D".into());

    get_res(&input, &grid)
}

fn get_res(input: &[Vec<char>], available_positions: &HashMap<(i32, i32), String>) -> String {
    let mut index: (i32, i32) = (1, 1);
    let mut res: String = String::new();

    for line in input {
        for i in line {
            match i {
                'U' => {
                    let new_index = (index.0 - 1, index.1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                }
                'D' => {
                    let new_index = (index.0 + 1, index.1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                }
                'R' => {
                    let new_index = (index.0, index.1 + 1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                }
                'L' => {
                    let new_index = (index.0, index.1 - 1);
                    if available_positions.keys().any(|&x| x == new_index) {
                        index = new_index;
                    }
                }
                _ => unreachable!(),
            }
        }
        res.push_str(&available_positions[&index]);
    }

    res
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "ULL
RRDDD
LURDL
UUUUD";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "1985");
    }
}
