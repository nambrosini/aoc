use adventofcode::util::position::{Vec2, v};
use regex::Regex;

struct Machine {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}

fn parse(input: &str) -> Vec<Machine> {
    let re_button = Regex::new(r"Button \w: \w\+(?P<x>\d+), \w\+(?P<y>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    input
        .split("\n\n")
        .map(|m| {
            let mut iter = m.lines();

            let caps = re_button.captures(iter.next().unwrap()).unwrap();
            let button_a = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let caps = re_button.captures(iter.next().unwrap()).unwrap();
            let button_b = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            let caps = re_prize.captures(iter.next().unwrap()).unwrap();
            let prize = v(caps["x"].parse().unwrap(), caps["y"].parse().unwrap());

            Machine {
                a: button_a,
                b: button_b,
                prize,
            }
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let machines = parse(input);

    let mut sum = 0;
    for machine in machines {
        let b = (machine.a.x * machine.prize.y - machine.a.y * machine.prize.x)
            / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
        let a = (machine.prize.x - machine.b.x * b) / machine.a.x;

        if machine.a.x * a + machine.b.x * b == machine.prize.x
            && machine.a.y * a + machine.b.y * b == machine.prize.y
        {
            sum += a * 3 + b;
        }
    }

    sum
}

const PRIZE_INCREASE: i64 = 10000000000000;

fn part2(input: &str) -> i64 {
    let machines = parse(input);

    let machines: Vec<Machine> = machines
        .iter()
        .map(|machine| Machine {
            a: machine.a,
            b: machine.b,
            prize: machine.prize + PRIZE_INCREASE,
        })
        .collect();

    let mut sum = 0;
    for machine in machines {
        let b = (machine.a.x * machine.prize.y - machine.a.y * machine.prize.x)
            / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
        let a = (machine.prize.x - machine.b.x * b) / machine.a.x;

        if machine.a.x * a + machine.b.x * b == machine.prize.x
            && machine.a.y * a + machine.b.y * b == machine.prize.y
        {
            sum += a * 3 + b;
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 480);
    }

    // #[test]
    // fn test_part2() {
    //     let result = part2(SAMPLE);
    //     assert_eq!(result, None);
    // }
}
