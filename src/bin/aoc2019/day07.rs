use adventofcode::util::intcode::{Intcode, Step, parse};
use itertools::Itertools;

fn run_chain(mem: &[i64], phases: &[i64]) -> i64 {
    let mut amps: Vec<Intcode> = phases
        .iter()
        .map(|&p| {
            let mut pc = Intcode::new(mem);
            pc.push_input(p);
            pc
        })
        .collect();

    let mut signal = 0;
    loop {
        let mut all_halted = true;

        for amp in amps.iter_mut() {
            amp.push_input(signal);
            match amp.run() {
                Step::Output(v) => {
                    signal = v;
                    all_halted = false;
                }
                Step::Halted => {}
                Step::NeedInput => unreachable!(),
            }
        }

        if all_halted {
            return signal;
        }
    }
}

fn part1(input: &str) -> i64 {
    let mem = parse(input);
    (0..5)
        .permutations(5)
        .map(|p| run_chain(&mem, &p))
        .max()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let mem = parse(input);
    (5..10)
        .permutations(5)
        .map(|p| run_chain(&mem, &p))
        .max()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(part1(input), 43210);
    }

    #[test]
    fn test_part2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(part2(input), 139629729);
    }
}
