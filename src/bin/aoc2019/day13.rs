use adventofcode::util::intcode::{Intcode, Step, parse};
use std::cmp::Ordering;
use std::collections::HashMap;

fn next_tile(pc: &mut Intcode, joystick: i64) -> Option<((i64, i64), i64)> {
    let mut read = || loop {
        match pc.run() {
            Step::Output(v) => return Some(v),
            Step::Halted => return None,
            Step::NeedInput => pc.push_input(joystick),
        }
    };

    Some(((read()?, read()?), read()?))
}

fn part1(input: &str) -> usize {
    let mem = parse(input);
    let mut pc = Intcode::new(&mem);
    let mut tiles = HashMap::new();

    while let Some((pos, id)) = next_tile(&mut pc, 0) {
        tiles.insert(pos, id);
    }

    tiles.values().filter(|&&id| id == 2).count()
}

fn part2(input: &str) -> i64 {
    let mut mem = parse(input);
    mem[0] = 2;
    let mut pc = Intcode::new(&mem);

    let mut score = 0;
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut joystick = 0;

    while let Some((pos, id)) = next_tile(&mut pc, joystick) {
        if pos == (-1, 0) {
            score = id;
        } else {
            if id == 4 {
                ball_x = pos.0;
            }
            if id == 3 {
                paddle_x = pos.0;
            }
        }

        joystick = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };
    }

    score
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
