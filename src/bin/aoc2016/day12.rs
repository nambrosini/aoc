use adventofcode::util::assembunny::{Computer, Instruction};
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    Computer::new(HashMap::new(), &input).run()
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut memory = HashMap::new();
    memory.insert('c', 1);

    Computer::new(memory, &input).run()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

        assert_eq!(part1(s), 42);
    }
}
