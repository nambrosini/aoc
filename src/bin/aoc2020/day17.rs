use std::collections::{HashMap, HashSet};

type Coordinates = (i32, i32, i32, i32);

fn parse(input: &str) -> HashSet<Coordinates> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as i32, y as i32, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn step(dims: usize, active: &HashSet<Coordinates>) -> HashSet<Coordinates> {
    let mut neighbor_counts = HashMap::new();

    for &(x, y, z, w) in active {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in if dims > 2 { -1..=1 } else { 0..=0 } {
                    for dw in if dims > 3 { -1..=1 } else { 0..=0 } {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        let p = (x + dx, y + dy, z + dz, w + dw);
                        *neighbor_counts.entry(p).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    neighbor_counts
        .into_iter()
        .filter_map(|(p, count)| {
            if count == 3 || (count == 2 && active.contains(&p)) {
                Some(p)
            } else {
                None
            }
        })
        .collect()
}

fn run(input: &str, dims: usize) -> usize {
    let mut active = parse(input);

    for _ in 0..6 {
        active = step(dims, &active);
    }

    active.len()
}

fn part1(input: &str) -> usize {
    run(input, 3)
}

fn part2(input: &str) -> usize {
    run(input, 4)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".#.
..#
###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 848);
    }
}
