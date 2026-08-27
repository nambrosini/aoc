use itertools::Itertools;

fn parse(input: &str) -> Vec<Cell> {
    input.trim().chars().map(Cell::from).collect_vec()
}

fn part1(input: &str) -> usize {
    solve(&parse(input), 40)
}

fn part2(input: &str) -> usize {
    solve(&parse(input), 400000)
}

fn solve(input: &[Cell], rows: usize) -> usize {
    let count_open = |row: &[Cell]| -> usize { row.iter().filter(|&&x| x == Cell::Safe).count() };
    let mut row: Vec<Cell> = input.to_vec();
    let mut count = count_open(&row);

    for _ in 1..rows {
        row = gen_next_line(&row);

        count += count_open(&row);
    }

    count
}

fn gen_next_line(previous: &[Cell]) -> Vec<Cell> {
    let mut new = vec![];

    for i in 0..previous.len() {
        let left = if i == 0 { None } else { previous.get(i - 1) };
        new.push(Cell::gen_from_previous(left, previous.get(i + 1)));
    }

    new
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Safe,
    Trap,
}

impl Cell {
    fn gen_from_previous(left: Option<&Cell>, right: Option<&Cell>) -> Self {
        let left = left.copied().unwrap_or(Cell::Safe);
        let right = right.copied().unwrap_or(Cell::Safe);

        if left != right {
            Cell::Trap
        } else {
            Cell::Safe
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Safe,
            '^' => Cell::Trap,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/18.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_line() {
        let input = parse("..^^.");
        let expected = parse(".^^^^");

        assert_eq!(gen_next_line(&input), expected);
    }

    #[test]
    fn test_part1() {
        let input = parse(".^^.^.^^^^");

        assert_eq!(solve(&input, 10), 38);
    }
}
