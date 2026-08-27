use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn neighbors(x: usize, y: usize, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);
    let mut result = vec![];

    for i in -1..=1 {
        let new_x = x + i;
        if !(0..grid.len() as i32).contains(&new_x) {
            continue;
        }
        for j in -1..=1 {
            let new_y = y + j;
            if (i == 0 && j == 0) || !(0..grid[0].len() as i32).contains(&new_y) {
                continue;
            }
            result.push((new_x as usize, new_y as usize));
        }
    }

    result
}

/// Finds the full number that the digit at `(x, y)` belongs to on its row.
fn number_at(x: usize, row: &[char]) -> i32 {
    let start = (0..=x)
        .rev()
        .take_while(|&i| row[i].is_ascii_digit())
        .last()
        .unwrap();
    let end = (x..row.len())
        .take_while(|&i| row[i].is_ascii_digit())
        .last()
        .unwrap();

    row[start..=end].iter().collect::<String>().parse().unwrap()
}

fn has_adjacent_symbol(x: usize, y: usize, grid: &[Vec<char>]) -> bool {
    neighbors(x, y, grid).iter().any(|&(nx, ny)| {
        let c = grid[nx][ny];
        !c.is_ascii_digit() && c != '.'
    })
}

fn part1(input: &str) -> i32 {
    let grid = parse(input);

    let mut sum = 0;
    let mut last_num = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                if has_adjacent_symbol(x, y, &grid) {
                    let v = number_at(y, row);
                    if v != last_num {
                        sum += v;
                        last_num = v;
                    }
                }
            } else {
                last_num = 0;
            }
        }
    }

    sum
}

fn gear_ratio(x: usize, y: usize, grid: &[Vec<char>]) -> i32 {
    let mut parts = HashSet::new();

    for (nx, ny) in neighbors(x, y, grid) {
        if grid[nx][ny].is_ascii_digit() {
            parts.insert(number_at(ny, &grid[nx]));
            if parts.len() > 2 {
                return 0;
            }
        }
    }

    if parts.len() == 2 {
        parts.iter().product()
    } else {
        0
    }
}

fn part2(input: &str) -> i32 {
    let grid = parse(input);

    let mut sum = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == '*' {
                sum += gear_ratio(x, y, &grid);
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_has_adjacent_symbol() {
        let grid = parse("...\n.63\n.#.");
        assert!(has_adjacent_symbol(1, 1, &grid));

        let grid = parse("...\n.63\n...");
        assert!(!has_adjacent_symbol(1, 1, &grid));
    }

    #[test]
    fn test_number_at() {
        let grid = parse("...\n.63\n...");
        assert_eq!(number_at(2, &grid[1]), 63);

        let grid = parse("...\n763\n...");
        assert_eq!(number_at(0, &grid[1]), 763);
    }

    #[test]
    fn test_gear_ratio() {
        assert_eq!(gear_ratio(1, 1, &parse(SAMPLE)), 16345);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 467835);
    }
}
