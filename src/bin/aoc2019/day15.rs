use adventofcode::util::intcode::{Intcode, parse};
use std::collections::{HashMap, VecDeque};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Add<(i64, i64)> for Position {
    type Output = Self;

    fn add(self, (dx, dy): (i64, i64)) -> Self::Output {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

fn delta(dir: i64) -> (i64, i64) {
    match dir {
        1 => (0, -1),
        2 => (0, 1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => unreachable!(),
    }
}

fn explore(mem: &[i64]) -> HashMap<Position, i64> {
    let origin = Position { x: 0, y: 0 };
    let mut map = HashMap::from([(origin, 0)]);
    move_robot(&mut map, &Intcode::new(mem), origin);
    map
}

fn move_robot(map: &mut HashMap<Position, i64>, pc: &Intcode, pos: Position) {
    for dir in 1..=4 {
        let new_pos = pos + delta(dir);
        if map.contains_key(&new_pos) {
            continue;
        }

        let mut next = pc.clone();
        let status = next.run_to_output(dir).unwrap();
        map.insert(new_pos, status);

        if status != 0 {
            move_robot(map, &next, new_pos);
        }
    }
}

fn bfs(map: &HashMap<Position, i64>, start: Position, is_target: impl Fn(i64) -> bool) -> i64 {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = std::collections::HashSet::from([start]);
    let mut farthest = 0;

    while let Some((pos, dist)) = queue.pop_front() {
        if is_target(map[&pos]) {
            return dist;
        }
        farthest = dist;

        for dir in 1..=4 {
            let next = pos + delta(dir);
            if map.get(&next).is_some_and(|&v| v != 0) && visited.insert(next) {
                queue.push_back((next, dist + 1));
            }
        }
    }

    farthest
}

fn part1(input: &str) -> i64 {
    let map = explore(&parse(input));
    bfs(&map, Position { x: 0, y: 0 }, |v| v == 2)
}

fn part2(input: &str) -> i64 {
    let map = explore(&parse(input));
    let start = *map.iter().find(|&(_, &v)| v == 2).unwrap().0;
    bfs(&map, start, |_| false)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
