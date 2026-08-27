use adventofcode::util::assembunny::{Computer, Instruction};
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let mut counter = 0;

    loop {
        let mut memory = HashMap::new();
        memory.insert('a', counter);

        let output = Computer::new(memory, &input).run_collecting_output(100);

        if is_alternating(&output) {
            return counter;
        }

        counter += 1;
    }
}

fn is_alternating(out: &[i32]) -> bool {
    out.windows(2).all(|w| w[0] != w[1])
}

// AoC day 25 has no real part 2 (it's unlocked once every other puzzle is solved).
fn part2() -> &'static str {
    "\u{1F384} (free star)"
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/25.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2());
}
