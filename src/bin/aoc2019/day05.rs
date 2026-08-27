use adventofcode::util::intcode::{Intcode, Step, parse};

fn diagnostic(input: &str, system_id: i64) -> i64 {
    let mem = parse(input);
    let mut pc = Intcode::new(&mem);
    pc.push_input(system_id);

    let mut last = 0;
    loop {
        match pc.run() {
            Step::Output(v) => last = v,
            Step::Halted => return last,
            Step::NeedInput => unreachable!(),
        }
    }
}

fn part1(input: &str) -> i64 {
    diagnostic(input, 1)
}

fn part2(input: &str) -> i64 {
    diagnostic(input, 5)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
