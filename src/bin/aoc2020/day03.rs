fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_trees(grid: &[Vec<char>], right: usize, down: usize) -> usize {
    let row_len = grid[0].len();
    let mut x = 0;
    let mut count = 0;

    for y in (0..grid.len()).step_by(down) {
        if grid[y][x] == '#' {
            count += 1;
        }

        x = (x + right) % row_len;
    }

    count
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    count_trees(&grid, 3, 1)
}

fn part2(input: &str) -> usize {
    let grid = parse(input);

    [(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |res, &(right, down)| {
            res * count_trees(&grid, right, down)
        })
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 336);
    }
}
