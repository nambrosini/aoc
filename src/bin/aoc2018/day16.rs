use adventofcode::util::elfcode::{OpCode, execute};
use std::collections::{HashMap, HashSet};

type Check = (Vec<usize>, Vec<usize>, Vec<usize>);

fn parse(input: &str) -> (Vec<Check>, Vec<Vec<usize>>) {
    let split: Vec<&str> = input.split("\n\n\n\n").collect();
    let first = split[0]
        .split("\n\n")
        .map(|ins| {
            let vec: Vec<&str> = ins.lines().collect();
            let before: Vec<usize> = vec[0][9..19]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            let instruction: Vec<usize> = vec[1].split(' ').map(|x| x.parse().unwrap()).collect();
            let after: Vec<usize> = vec[2][9..19]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect();
            (instruction, before, after)
        })
        .collect();
    let second = split[1]
        .lines()
        .map(|ins| {
            ins.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (first, second)
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input
        .0
        .iter()
        .filter(|x| get_matching(&x.0, &x.1, &x.2).len() >= 3)
        .count()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut map: HashMap<usize, HashSet<OpCode>> = HashMap::new();
    for (instruction, before, after) in input.0.iter() {
        let ops = get_matching(instruction, before, after);
        let entry = map.entry(instruction[0]).or_default();
        if entry.is_empty() {
            *entry = ops;
        } else {
            *entry = entry.intersection(&ops).copied().collect();
        }
    }

    let mut codes: HashMap<usize, OpCode> = HashMap::new();

    while !map.is_empty() {
        let mut new_map = map.clone();
        let singles = map.iter().filter(|(_, k)| k.len() == 1);

        for single in singles {
            let op = *single.1.iter().next().unwrap();
            codes.insert(*single.0, op);
            new_map.remove(single.0);

            for v in new_map.values_mut() {
                v.remove(&op);
            }
        }
        map = new_map;
    }

    let mut regs = vec![0; 4];

    for instruction in &input.1 {
        regs = execute(
            codes[&instruction[0]],
            &regs,
            instruction[1],
            instruction[2],
            instruction[3],
        );
    }

    regs[0]
}

fn get_matching(instruction: &[usize], before: &[usize], after: &[usize]) -> HashSet<OpCode> {
    OpCode::ALL
        .into_iter()
        .filter(|&op| execute(op, before, instruction[1], instruction[2], instruction[3]) == after)
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/16.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matching() {
        let expected = [OpCode::Mulr, OpCode::Addi, OpCode::Seti];
        let got = get_matching(&[9, 2, 1, 2], &[3, 2, 1, 1], &[3, 2, 2, 1]);

        for e in expected.iter() {
            assert!(got.contains(e));
        }
    }
}
