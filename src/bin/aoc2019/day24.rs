use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_neighbours(x: usize, y: usize, grid: &[Vec<bool>]) -> usize {
    let (x, y) = (x as i32, y as i32);
    let len = grid.len() as i32;

    [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)]
        .into_iter()
        .filter(|&(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            nx >= 0 && ny >= 0 && nx < len && ny < len && grid[ny as usize][nx as usize]
        })
        .count()
}

fn step(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &alive)| {
                    let n = count_neighbours(x, y, grid);
                    if alive { n == 1 } else { n == 1 || n == 2 }
                })
                .collect()
        })
        .collect()
}

fn biodiversity(grid: &[Vec<bool>]) -> u64 {
    grid.iter()
        .flatten()
        .enumerate()
        .filter(|&(_, &alive)| alive)
        .map(|(i, _)| 1u64 << i)
        .sum()
}

fn part1(input: &str) -> u64 {
    let mut grid = parse(input);
    let mut seen = HashSet::new();

    loop {
        let score = biodiversity(&grid);
        if !seen.insert(score) {
            return score;
        }
        grid = step(&grid);
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/24.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biodiversity() {
        let input = ".....\n.....\n.....\n#....\n.#...";
        assert_eq!(biodiversity(&parse(input)), 2129920);
    }
}
