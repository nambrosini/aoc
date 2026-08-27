use itertools::Itertools;
use regex::Regex;
use std::ops::AddAssign;

fn parse(input: &str) -> Vec<Particle> {
    input.lines().map(Particle::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let mut particles = parse(input);

    for _ in 0..10000 {
        particles.iter_mut().for_each(|p| p.step());
    }

    particles
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.manhattan()))
        .min_by_key(|(_, e)| *e)
        .unwrap()
        .0
}

fn part2(input: &str) -> usize {
    let mut particles = parse(input);

    for _ in 0..10000 {
        particles.iter_mut().for_each(|p| p.step());

        let mut no_collisions = vec![];

        for p in &particles {
            if particles.iter().filter(|&x| x == p).count() == 1 {
                no_collisions.push(*p);
            }
        }

        particles = no_collisions.clone();
    }

    particles.len()
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i128,
    y: i128,
    z: i128,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl From<&str> for Vector {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"[pva]=<(-\d+|\d+),(-\d+|\d+),(-\d+|\d+)>").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        Self {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            z: cap[3].parse().unwrap(),
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl From<&str> for Particle {
    fn from(s: &str) -> Self {
        let split = s.split(", ").collect_vec();

        Self {
            p: split[0].into(),
            v: split[1].into(),
            a: split[2].into(),
        }
    }
}

impl Particle {
    fn step(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }

    fn manhattan(&self) -> i128 {
        self.p.x.abs() + self.p.y.abs() + self.p.z.abs()
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/20.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

        assert_eq!(part1(s), 0);
    }

    #[test]
    fn test_part2() {
        let s = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

        assert_eq!(part2(s), 1);
    }
}
