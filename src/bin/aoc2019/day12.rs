use adventofcode::util::math::lcm;
use std::collections::HashSet;

type Vector3 = [i64; 3];

#[derive(Clone)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn apply_gravity(&mut self, other: &Vector3) {
        for ((v, &p), &o) in self.velocity.iter_mut().zip(&self.position).zip(other) {
            *v += (o - p).signum();
        }
    }

    fn apply_velocity(&mut self) {
        for (p, &v) in self.position.iter_mut().zip(&self.velocity) {
            *p += v;
        }
    }

    fn energy(&self) -> i64 {
        self.position.iter().map(|x| x.abs()).sum::<i64>()
            * self.velocity.iter().map(|x| x.abs()).sum::<i64>()
    }
}

fn parse(input: &str) -> Vec<Moon> {
    input
        .trim()
        .lines()
        .map(|l| {
            let nums: Vec<i64> = l
                .trim_matches(|c: char| !c.is_ascii_digit() && c != '-')
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect();
            Moon {
                position: [nums[0], nums[1], nums[2]],
                velocity: [0; 3],
            }
        })
        .collect()
}

fn step(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            let p = moons[j].position;
            moons[i].apply_gravity(&p);
        }
    }
    for m in moons.iter_mut() {
        m.apply_velocity();
    }
}

fn part1(input: &str) -> i64 {
    let mut moons = parse(input);

    for _ in 0..1000 {
        step(&mut moons);
    }

    moons.iter().map(|m| m.energy()).sum()
}

fn part2(input: &str) -> i64 {
    let mut moons = parse(input);
    let mut seen = [HashSet::new(), HashSet::new(), HashSet::new()];
    let mut period = [None; 3];

    for turn in 0_i64.. {
        for axis in 0..3 {
            if period[axis].is_some() {
                continue;
            }
            let state: Vec<(i64, i64)> = moons
                .iter()
                .map(|m| (m.position[axis], m.velocity[axis]))
                .collect();
            if !seen[axis].insert(state) {
                period[axis] = Some(turn);
            }
        }

        if period.iter().all(|p| p.is_some()) {
            break;
        }

        step(&mut moons);
    }

    period.iter().map(|p| p.unwrap()).fold(1, lcm)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
