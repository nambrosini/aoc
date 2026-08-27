use adventofcode::util::math::gcd;
use std::collections::HashSet;
use std::f64::consts::PI;

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect()
}

fn visible_count(from: (i64, i64), asteroids: &[(i64, i64)]) -> usize {
    let dirs: HashSet<(i64, i64)> = asteroids
        .iter()
        .filter(|&&a| a != from)
        .map(|&a| {
            let (dx, dy) = (a.0 - from.0, a.1 - from.1);
            let g = gcd(dx, dy);
            (dx / g, dy / g)
        })
        .collect();

    dirs.len()
}

fn best_station(asteroids: &[(i64, i64)]) -> (i64, i64) {
    *asteroids
        .iter()
        .max_by_key(|&&a| visible_count(a, asteroids))
        .unwrap()
}

fn part1(input: &str) -> usize {
    let asteroids = parse(input);
    let station = best_station(&asteroids);
    visible_count(station, &asteroids)
}

fn part2(input: &str) -> i64 {
    let asteroids = parse(input);
    let station = best_station(&asteroids);

    let mut by_angle: Vec<((i64, i64), i32, i64)> = asteroids
        .iter()
        .filter(|&&a| a != station)
        .map(|&a| {
            let (dx, dy) = ((a.0 - station.0) as f64, (a.1 - station.1) as f64);
            let angle = dy.atan2(dx) + PI / 2.0;
            let angle = if angle < 0.0 { angle + 2.0 * PI } else { angle };
            let dist = (dx * dx + dy * dy) as i64;
            (a, (angle * 1_000_000.0) as i32, dist)
        })
        .collect();

    by_angle.sort_by_key(|&(_, angle, dist)| (angle, dist));

    let mut order = vec![];
    while !by_angle.is_empty() {
        let mut last_angle = None;
        by_angle.retain(|&(a, angle, _)| {
            if last_angle == Some(angle) {
                return true;
            }
            last_angle = Some(angle);
            order.push(a);
            false
        });
    }

    let (x, y) = order[199];
    x * 100 + y
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        assert_eq!(part1(input), 8);
    }
}
