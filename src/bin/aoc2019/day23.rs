use adventofcode::util::intcode::{Intcode, Step, parse};

fn run_network(input: &str, use_nat: bool) -> i64 {
    let mem = parse(input);
    let mut computers: Vec<Intcode> = (0..50)
        .map(|i| {
            let mut pc = Intcode::new(&mem);
            pc.push_input(i);
            pc
        })
        .collect();

    let mut queues: Vec<Vec<i64>> = vec![Vec::new(); 50];
    let mut nat: Option<(i64, i64)> = None;
    let mut last_nat_y = None;

    loop {
        let mut idle = true;

        for id in 0..50 {
            if queues[id].is_empty() {
                computers[id].push_input(-1);
            } else {
                idle = false;
                for v in queues[id].drain(..) {
                    computers[id].push_input(v);
                }
            }

            loop {
                match computers[id].run() {
                    Step::NeedInput | Step::Halted => break,
                    Step::Output(dest) => {
                        idle = false;
                        let x = match computers[id].run() {
                            Step::Output(v) => v,
                            _ => unreachable!(),
                        };
                        let y = match computers[id].run() {
                            Step::Output(v) => v,
                            _ => unreachable!(),
                        };

                        if dest == 255 {
                            if !use_nat {
                                return y;
                            }
                            nat = Some((x, y));
                        } else {
                            queues[dest as usize].push(x);
                            queues[dest as usize].push(y);
                        }
                    }
                }
            }
        }

        if use_nat
            && idle
            && let Some((x, y)) = nat
        {
            if last_nat_y == Some(y) {
                return y;
            }
            last_nat_y = Some(y);
            queues[0].push(x);
            queues[0].push(y);
        }
    }
}

fn part1(input: &str) -> i64 {
    run_network(input, false)
}

fn part2(input: &str) -> i64 {
    run_network(input, true)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/23.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
