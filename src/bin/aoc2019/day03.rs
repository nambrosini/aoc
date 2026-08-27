use std::collections::{HashMap, HashSet};

type Wire = Vec<(i64, i64)>;

fn parse(input: &str) -> (Wire, Wire) {
    let mut lines = input.trim().lines().map(wire_path);
    (lines.next().unwrap(), lines.next().unwrap())
}

fn wire_path(line: &str) -> Vec<(i64, i64)> {
    let mut path = vec![(0i64, 0i64)];

    for step in line.split(',') {
        let (dx, dy): (i64, i64) = match &step[..1] {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            dir => panic!("unknown direction: {dir}"),
        };
        let len: i64 = step[1..].parse().unwrap();
        let &(x, y) = path.last().unwrap();

        path.extend((1..=len).map(|d| (x + dx * d, y + dy * d)));
    }

    path
}

fn part1(input: &str) -> i64 {
    let (wire0, wire1) = parse(input);
    let points0: HashSet<_> = wire0.iter().skip(1).collect();
    let points1: HashSet<_> = wire1.iter().skip(1).collect();

    points0
        .intersection(&points1)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let (wire0, wire1) = parse(input);
    let steps0: HashMap<(i64, i64), usize> = wire0
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .rev()
        .collect();

    wire1
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(d1, p)| steps0.get(p).map(|d0| d0 + d1))
        .min()
        .unwrap() as i64
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part1(input), 159);
    }

    #[test]
    fn test_part2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(part2(input), 610);
    }
}
