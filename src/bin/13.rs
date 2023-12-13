advent_of_code::solution!(13);

struct Input {
    vals: Vec<Vec<Vec<char>>>,
}

fn parse(input: &str) -> Input {
    let vals = input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.chars().collect()).collect())
        .collect();

    Input { vals }
}

pub fn part_one(input: &str) -> Option<usize> {
    let vals = parse(input).vals;

    vals.iter().map(|grid| calc_reflection(grid, 0)).sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let vals = parse(input).vals;
    vals.iter().map(|grid| calc_reflection(grid, 1)).sum()
}

fn calc_reflection(grid: &[Vec<char>], flex: u32) -> Option<usize> {
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    for line in grid {
        cols.resize(line.len(), 0);
        let mut row = 0;
        for (c, v) in line.iter().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == &'#') as u32);
            row = (row << 1) | ((v == &'#') as u32);
        }
        rows.push(row);
    }
    for c in 1..cols.len() {
        if mirrors(&cols, c, flex) {
            return Some(c);
        }
    }
    for r in 1..rows.len() {
        if mirrors(&rows, r, flex) {
            return Some(100 * r);
        }
    }
    None
}

fn mirrors(v: &[u32], i: usize, flex: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..v.len())
        .map(|(a, b)| (v[a] ^ v[b]).count_ones())
        .sum::<u32>()
        == flex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
