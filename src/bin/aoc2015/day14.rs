use itertools::Itertools;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

#[derive(Debug, Clone)]
struct Race {
    reindeer: Reindeer,
    time: usize,
    km: usize,
}

impl TryFrom<&str> for Reindeer {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split = value.split(' ').collect_vec();

        let name = split[0].to_owned();
        let speed = split[3].parse().unwrap();
        let fly_time = split[6].parse().unwrap();
        let rest_time = split[13].parse().unwrap();

        Ok(Self {
            name,
            speed,
            fly_time,
            rest_time,
        })
    }
}

impl Race {
    fn new(reindeer: Reindeer) -> Self {
        Self {
            reindeer,
            time: 0,
            km: 0,
        }
    }

    fn next_second(&mut self) -> usize {
        if self.time % (self.reindeer.fly_time + self.reindeer.rest_time) < self.reindeer.fly_time {
            self.km += self.reindeer.speed;
        }

        self.time += 1;
        self.km
    }
}

fn parse(input: &str) -> Vec<Race> {
    input
        .lines()
        .map(|l| Race::new(l.try_into().unwrap()))
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);
    (0..2503)
        .flat_map(|_| map.iter_mut().map(|r| r.next_second()).collect_vec())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let mut grid: HashMap<String, usize> = HashMap::new();

    for _ in 0..2503 {
        for x in map.iter_mut() {
            x.next_second();
        }

        let r = map.iter().max_by(|r0, r1| r0.km.cmp(&r1.km)).unwrap();

        let entry = grid.entry(r.reindeer.name.clone()).or_insert(0);
        *entry += 1;
    }

    *grid.values().max().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 2660);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1564);
    }
}
