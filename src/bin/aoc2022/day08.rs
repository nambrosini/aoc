fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn sightlines(grid: &[Vec<usize>], x: usize, y: usize) -> [Vec<usize>; 4] {
    let row = &grid[x];
    let col: Vec<usize> = grid.iter().map(|r| r[y]).collect();

    [
        row[..y].iter().rev().copied().collect(),
        row[y + 1..].to_vec(),
        col[..x].iter().rev().copied().collect(),
        col[x + 1..].to_vec(),
    ]
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    let mut count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let cell = grid[x][y];
            if sightlines(&grid, x, y)
                .iter()
                .any(|line| line.iter().all(|&t| t < cell))
            {
                count += 1;
            }
        }
    }

    count
}

fn viewing_distance(line: &[usize], cell: usize) -> usize {
    let blocked = line.iter().position(|&t| t >= cell);
    blocked.map_or(line.len(), |i| i + 1)
}

fn part2(input: &str) -> usize {
    let grid = parse(input);

    let mut best = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let cell = grid[x][y];
            let score: usize = sightlines(&grid, x, y)
                .iter()
                .map(|line| viewing_distance(line, cell))
                .product();
            if score > best {
                best = score;
            }
        }
    }

    best
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 8);
    }
}
