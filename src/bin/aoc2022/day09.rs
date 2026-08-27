use std::collections::HashSet;

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|l| {
            let (dir, n) = l.split_once(' ').unwrap();
            (dir, n.parse().unwrap())
        })
        .collect()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Knot {
    row: i32,
    col: i32,
}

fn move_head(rope: &mut [Knot], dir: &str) {
    match dir {
        "U" => rope[0].row -= 1,
        "D" => rope[0].row += 1,
        "L" => rope[0].col -= 1,
        "R" => rope[0].col += 1,
        _ => unreachable!(),
    }

    for i in 1..rope.len() {
        let drow = rope[i - 1].row - rope[i].row;
        let dcol = rope[i - 1].col - rope[i].col;

        if drow.abs() > 1 || dcol.abs() > 1 {
            rope[i] = Knot {
                row: rope[i].row + drow.signum(),
                col: rope[i].col + dcol.signum(),
            };
        }
    }
}

fn tail_positions(moves: &[(&str, i32)], length: usize) -> HashSet<Knot> {
    let mut rope = vec![Knot::default(); length];
    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());

    for &(dir, n) in moves {
        for _ in 0..n {
            move_head(&mut rope, dir);
            visited.insert(*rope.last().unwrap());
        }
    }

    visited
}

fn part1(input: &str) -> usize {
    tail_positions(&parse(input), 2).len()
}

fn part2(input: &str) -> usize {
    tail_positions(&parse(input), 10).len()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(s), 13);
    }

    #[test]
    fn test_part2() {
        let s = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(s), 36);
    }
}
