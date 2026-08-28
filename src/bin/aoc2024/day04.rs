use adventofcode::util::{
    grid::{Grid, Inbound, Parse},
    position::Vec2,
};

const GOAL: &[char; 4] = &['X', 'M', 'A', 'S'];

fn part1(input: &str) -> usize {
    let grid: Grid<char> = Grid::parse(input);

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &c)| c == 'X')
                .map(|(j, _)| find_all_xmas(&grid, i, j))
                .sum::<usize>()
        })
        .sum()
}

fn find_all_xmas(grid: &Grid<char>, i: usize, j: usize) -> usize {
    Vec2::DIR_WITH_DIAGONALS
        .iter()
        .filter(|dir| search_xmas(grid, i as i64, j as i64, dir))
        .count()
}

fn search_xmas(grid: &Grid<char>, x: i64, y: i64, inc: &Vec2) -> bool {
    if !grid.inbound(&Vec2::new(x + inc.x * 3, y + inc.y * 3)) {
        return false;
    }
    GOAL.iter().enumerate().all(|(i, &goal)| {
        grid[(x + inc.x * i as i64) as usize][(y + inc.y * i as i64) as usize] == goal
    })
}

fn part2(input: &str) -> u32 {
    let grid = Grid::parse(input);

    let mut res = 0;
    for (i, row) in grid.iter().enumerate().skip(1) {
        if i == grid.len() - 1 {
            continue;
        }
        for (j, c) in row.iter().enumerate().skip(1) {
            if j == grid.len() - 1 || c != &'A' {
                continue;
            }
            res += find_all_x_mas(&grid, i, j);
        }
    }
    res
}

fn find_all_x_mas(grid: &Grid<char>, i: usize, j: usize) -> u32 {
    let mut res: Vec<char> = Vec::new();
    let mut res1: Vec<char> = Vec::new();
    for k in -1..=1 {
        res.push(grid[(i as i64 + k) as usize][(j as i64 + k) as usize]);
        res1.push(grid[(i as i64 + k) as usize][(j as i64 - k) as usize]);
    }
    res.sort();
    res1.sort();

    if res == res1 {
        return 1;
    }

    0
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 9);
    }
}
