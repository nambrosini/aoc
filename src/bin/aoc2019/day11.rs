use adventofcode::util::intcode::{Intcode, Step, parse};
use std::collections::HashMap;

fn paint(mem: &[i64], start_color: i64) -> HashMap<(i64, i64), i64> {
    let mut pc = Intcode::new(mem);
    let mut panels = HashMap::new();
    let mut pos = (0i64, 0i64);
    let mut dir = (0i64, -1i64);
    panels.insert(pos, start_color);

    loop {
        let color = *panels.get(&pos).unwrap_or(&0);
        pc.push_input(color);

        let paint = match pc.run() {
            Step::Output(v) => v,
            Step::Halted => break,
            Step::NeedInput => unreachable!(),
        };
        let turn = match pc.run() {
            Step::Output(v) => v,
            Step::Halted => break,
            Step::NeedInput => unreachable!(),
        };

        panels.insert(pos, paint);
        dir = if turn == 0 {
            (dir.1, -dir.0)
        } else {
            (-dir.1, dir.0)
        };
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }

    panels
}

fn render(panels: &HashMap<(i64, i64), i64>) -> String {
    let min_x = panels.keys().map(|p| p.0).min().unwrap();
    let max_x = panels.keys().map(|p| p.0).max().unwrap();
    let min_y = panels.keys().map(|p| p.1).min().unwrap();
    let max_y = panels.keys().map(|p| p.1).max().unwrap();

    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            s.push(if panels.get(&(x, y)) == Some(&1) {
                '#'
            } else {
                ' '
            });
        }
        s.push('\n');
    }
    s
}

fn part1(input: &str) -> usize {
    paint(&parse(input), 0).len()
}

fn part2(input: &str) -> String {
    render(&paint(&parse(input), 1))
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}
