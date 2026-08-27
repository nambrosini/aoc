use adventofcode::util::intcode::{Intcode, parse};

fn run(mem: &[i64], noun: i64, verb: i64) -> i64 {
    let mut pc = Intcode::new(mem);
    pc.poke(1, noun);
    pc.poke(2, verb);
    pc.run_to_halt();
    pc.peek(0)
}

fn part1(input: &str) -> i64 {
    run(&parse(input), 12, 2)
}

fn part2(input: &str) -> i64 {
    let mem = parse(input);

    for noun in 0..99 {
        for verb in 0..99 {
            if run(&mem, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
