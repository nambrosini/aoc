use std::collections::HashSet;

type Position = (i32, i32);
type Cave = HashSet<Position>;

fn parse(input: &str) -> Cave {
    let mut cave = HashSet::new();

    for line in input.lines() {
        let points: Vec<Position> = line
            .split(" -> ")
            .map(|c| {
                let (x, y) = c.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        for w in points.windows(2) {
            let (min_x, max_x) = (w[0].0.min(w[1].0), w[0].0.max(w[1].0));
            let (min_y, max_y) = (w[0].1.min(w[1].1), w[0].1.max(w[1].1));
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    cave.insert((x, y));
                }
            }
        }
    }

    cave
}

/// Drops one grain of sand from (500, 0) until it settles.
/// If `void_y` is set, returns `None` once the grain falls past it (no floor case).
fn drop_grain(cave: &Cave, void_y: Option<i32>) -> Option<Position> {
    let mut grain = (500, 0);

    loop {
        if void_y.is_some_and(|void_y| grain.1 > void_y) {
            return None;
        }

        let below = (grain.0, grain.1 + 1);
        if !cave.contains(&below) {
            grain = below;
        } else if !cave.contains(&(below.0 - 1, below.1)) {
            grain = (below.0 - 1, below.1);
        } else if !cave.contains(&(below.0 + 1, below.1)) {
            grain = (below.0 + 1, below.1);
        } else {
            return Some(grain);
        }
    }
}

fn part1(input: &str) -> usize {
    let mut cave = parse(input);
    let max_y = cave.iter().map(|p| p.1).max().unwrap();

    let mut count = 0;
    while let Some(grain) = drop_grain(&cave, Some(max_y)) {
        cave.insert(grain);
        count += 1;
    }

    count
}

fn add_floor(cave: &mut Cave) {
    let floor_y = cave.iter().map(|p| p.1).max().unwrap() + 2;
    let floor_start_x = 500 - floor_y;
    let floor_end_x = 500 + floor_y;

    for x in floor_start_x..=floor_end_x {
        cave.insert((x, floor_y));
    }
}

fn part2(input: &str) -> usize {
    let mut cave = parse(input);
    add_floor(&mut cave);

    let mut count = 0;
    while !cave.contains(&(500, 0)) {
        let grain = drop_grain(&cave, None).unwrap();
        cave.insert(grain);
        count += 1;
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 93);
    }
}
