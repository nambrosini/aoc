use adventofcode::util::position::Vec2;
use regex::Regex;

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(?P<px>-?\d+),(?P<py>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let p = Vec2::new(caps["px"].parse().unwrap(), caps["py"].parse().unwrap());
            let v = Vec2::new(caps["vx"].parse().unwrap(), caps["vy"].parse().unwrap());
            Robot { pos: p, vel: v }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let mut robots = parse(input);
    let width = robots.iter().max_by_key(|r| r.pos.x).unwrap().pos.x + 1;
    let height = robots.iter().max_by_key(|r| r.pos.y).unwrap().pos.y + 1;

    for _ in 0..100 {
        simulate(&mut robots, width, height);
    }

    let quadrants: (i64, i64, i64, i64) = robots
        .iter()
        .map(|r| {
            (
                (r.pos.x < width / 2 && r.pos.y < height / 2) as i64,
                (r.pos.x > width / 2 && r.pos.y < height / 2) as i64,
                (r.pos.x < width / 2 && r.pos.y > height / 2) as i64,
                (r.pos.x > width / 2 && r.pos.y > height / 2) as i64,
            )
        })
        .fold((0, 0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
        });

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn part2(input: &str) -> Option<u32> {
    let mut robots = parse(input);
    let width = robots.iter().max_by_key(|r| r.pos.x).unwrap().pos.x + 1;
    let height = robots.iter().max_by_key(|r| r.pos.y).unwrap().pos.y + 1;
    for i in 1..=7338 {
        simulate(&mut robots, width, height);
        let count = count_adjacent(&robots);
        if count > 100 {
            return Some(i);
        }
    }

    None
}

fn simulate(robots: &mut [Robot], width: i64, height: i64) {
    for robot in robots.iter_mut() {
        robot.pos.x = (robot.pos.x + robot.vel.x).rem_euclid(width);
        robot.pos.y = (robot.pos.y + robot.vel.y).rem_euclid(height);
    }
}

fn count_adjacent(robots: &[Robot]) -> i64 {
    let mut sum = 0;
    for robot in robots {
        if robots
            .iter()
            .any(|r| r.pos.x == robot.pos.x + 1 && r.pos.y == robot.pos.y)
        {
            sum += 1;
        }
    }

    sum
}

#[allow(dead_code)]
fn print(robots: &[Robot], width: i64, height: i64) {
    for i in 0..width {
        for j in 0..height {
            if robots.iter().any(|r| r.pos.x == i && r.pos.y == j) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 12);
    }

    #[test]
    fn test_part2() {
        let result = part2(SAMPLE);
        assert_eq!(result, None);
    }
}
