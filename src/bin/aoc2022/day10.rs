enum Instruction {
    Noop,
    Add(i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        match split[0] {
            "noop" => Self::Noop,
            "addx" => Self::Add(split[1].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn cycles(instructions: &[Instruction]) -> Vec<(i32, i32)> {
    let mut res = vec![];
    let (mut cycle, mut x) = (0, 1);

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                res.push((cycle, x));
            }
            Instruction::Add(val) => {
                cycle += 1;
                res.push((cycle, x));
                cycle += 1;
                res.push((cycle, x));
                x += val;
            }
        }
    }

    res
}

fn part1(input: &str) -> i32 {
    let target_cycles = [20, 60, 100, 140, 180, 220];

    cycles(&parse(input))
        .iter()
        .filter(|(cycle, _)| target_cycles.contains(cycle))
        .map(|(cycle, x)| cycle * x)
        .sum()
}

fn part2(input: &str) -> String {
    let mut screen = String::from("\n");

    for (cycle, x) in cycles(&parse(input)) {
        let screen_column = (cycle - 1) % 40;

        screen.push(if (x - screen_column).abs() < 2 {
            '█'
        } else {
            '.'
        });

        if screen_column == 39 {
            screen.push('\n');
        }
    }

    screen
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13140);
    }
}
