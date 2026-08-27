use std::collections::HashMap;

struct Line {
    from: (i32, i32),
    to: (i32, i32),
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (from, to) = s.split_once(" -> ").unwrap();
        let parse_point = |p: &str| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        };

        Self {
            from: parse_point(from),
            to: parse_point(to),
        }
    }
}

impl Line {
    fn is_hv_line(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }

    fn get_points(&self) -> Vec<(i32, i32)> {
        let dx = (self.to.0 - self.from.0).signum();
        let dy = (self.to.1 - self.from.1).signum();
        let dist = (self.to.0 - self.from.0)
            .abs()
            .max((self.to.1 - self.from.1).abs());

        (0..=dist)
            .map(|i| (self.from.0 + dx * i, self.from.1 + dy * i))
            .collect()
    }
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(Line::from).collect()
}

fn count_overlaps(lines: &[Line], hv_only: bool) -> usize {
    let mut map: HashMap<(i32, i32), usize> = HashMap::new();

    for line in lines {
        if hv_only && !line.is_hv_line() {
            continue;
        }

        for p in line.get_points() {
            *map.entry(p).or_insert(0) += 1;
        }
    }

    map.values().filter(|&&v| v > 1).count()
}

fn part1(input: &str) -> usize {
    count_overlaps(&parse(input), true)
}

fn part2(input: &str) -> usize {
    count_overlaps(&parse(input), false)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
