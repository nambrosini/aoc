use adventofcode::util::intcode::{Intcode, Step, parse};

fn part1(input: &str) -> i64 {
    let mem = parse(input);
    let mut pc = Intcode::new(&mem);
    let mut grid = String::new();

    loop {
        match pc.run() {
            Step::Output(v) => grid.push(v as u8 as char),
            Step::Halted => break,
            Step::NeedInput => unreachable!(),
        }
    }

    let map: Vec<Vec<char>> = grid
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let mut sum = 0;

    for x in 1..map.len() - 1 {
        for y in 1..map[x].len() - 1 {
            if map[x][y] == '#'
                && map[x - 1][y] == '#'
                && map[x + 1][y] == '#'
                && map[x][y - 1] == '#'
                && map[x][y + 1] == '#'
            {
                sum += (x * y) as i64;
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}
