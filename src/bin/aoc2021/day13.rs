use std::collections::HashSet;

type Point = (usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}

fn parse(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = folds
        .lines()
        .map(|l| {
            let (axis, val) = l.split_once('=').unwrap();
            let val = val.parse().unwrap();
            match axis.chars().last().unwrap() {
                'x' => Fold::X(val),
                'y' => Fold::Y(val),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, folds)
}

fn fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points
        .iter()
        .map(|&(x, y)| match fold {
            Fold::X(line) if x > *line => (line - (x - line), y),
            Fold::Y(line) if y > *line => (x, line - (y - line)),
            _ => (x, y),
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let (points, folds) = parse(input);

    fold(&points, &folds[0]).len()
}

fn part2(input: &str) -> String {
    let (points, folds) = parse(input);

    let points = folds.iter().fold(points, |points, f| fold(&points, f));

    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    let mut s = String::from("\n");
    for y in 0..=max_y {
        for x in 0..=max_x {
            s.push(if points.contains(&(x, y)) { '█' } else { ' ' });
        }
        s.push('\n');
    }

    s
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 17);
    }
}
