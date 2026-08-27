use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

type Position = (i64, i64);

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Sensor at x=(?P<x1>-?\d+), y=(?P<y1>-?\d+): closest beacon is at x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap()
});

struct Sensor {
    sensor: Position,
    beacon: Position,
}

impl Sensor {
    fn distance(&self, other: Position) -> i64 {
        (self.sensor.0 - other.0).abs() + (self.sensor.1 - other.1).abs()
    }

    fn radius(&self) -> i64 {
        self.distance(self.beacon)
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|l| {
            let caps = RE.captures(l).unwrap();
            Sensor {
                sensor: (caps["x1"].parse().unwrap(), caps["y1"].parse().unwrap()),
                beacon: (caps["x2"].parse().unwrap(), caps["y2"].parse().unwrap()),
            }
        })
        .collect()
}

/// Returns the sensor's excluded x-range on `row`, clamped to `[0, upper]`, if any.
fn row_range(sensor: &Sensor, upper: i64, row: i64) -> Option<(i64, i64)> {
    let dist = sensor.radius() - (sensor.sensor.1 - row).abs();
    if dist < 0 {
        return None;
    }

    let x_min = (sensor.sensor.0 - dist).max(0);
    let x_max = (sensor.sensor.0 + dist).min(upper);
    (x_min <= x_max).then_some((x_min, x_max))
}

fn cannot_exist(sensors: &[Sensor], row: i64) -> usize {
    let x_min = sensors
        .iter()
        .map(|s| s.sensor.0 - s.radius())
        .min()
        .unwrap();
    let x_max = sensors
        .iter()
        .map(|s| s.sensor.0 + s.radius())
        .max()
        .unwrap();

    let mut set: HashSet<i64> = (x_min..=x_max)
        .filter(|&x| sensors.iter().any(|s| s.distance((x, row)) <= s.radius()))
        .collect();

    for beacon in sensors.iter().map(|s| s.beacon).filter(|b| b.1 == row) {
        set.remove(&beacon.0);
    }

    set.len()
}

fn find_beacon(sensors: &[Sensor], max: i64) -> i64 {
    for y in 0..=max {
        let mut ranges: Vec<(i64, i64)> = sensors
            .iter()
            .filter_map(|s| row_range(s, max, y))
            .collect();
        ranges.sort_unstable();

        let mut x = 0;
        for (x_min, x_max) in ranges {
            if x_min <= x {
                x = x.max(x_max + 1);
            }
        }

        if x <= max {
            return x * 4_000_000 + y;
        }
    }

    unreachable!()
}

fn part1(input: &str) -> usize {
    cannot_exist(&parse(input), 2_000_000)
}

fn part2(input: &str) -> i64 {
    find_beacon(&parse(input), 4_000_000)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(cannot_exist(&parse(SAMPLE), 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(find_beacon(&parse(SAMPLE), 20), 56000011);
    }
}
