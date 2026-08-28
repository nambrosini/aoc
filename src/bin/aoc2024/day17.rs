fn parse(input: &str) -> (i64, i64, i64, Vec<i64>) {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .parse()
        .unwrap();

    let program = lines
        .last()
        .unwrap()
        .split(' ')
        .next_back()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    (a, b, c, program)
}

fn part1(input: &str) -> String {
    let (a, b, c, program) = parse(input);
    let res: Vec<String> = simulate_whole(a, b, c, program)
        .iter()
        .map(|x| x.to_string())
        .collect();
    res.join(",")
}

struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<i64>,
    pos: usize,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, program: Vec<i64>) -> Self {
        Self {
            a,
            b,
            c,
            program,
            pos: 0,
        }
    }

    fn run(&mut self) -> Option<i64> {
        while self.pos < self.program.len() {
            let instruction: Instruction = self.program[self.pos].into();
            let operand = self.program[self.pos + 1];
            match instruction {
                Instruction::Adv => self.a /= 2i64.pow(self.get_combo(operand) as u32),
                Instruction::Bxl => self.b ^= operand,
                Instruction::Bst => self.b = self.get_combo(operand) % 8,
                Instruction::Jnz => {
                    if self.a != 0 {
                        self.pos = operand as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.b ^= self.c,
                Instruction::Out => {
                    let val = self.get_combo(operand) % 8;
                    self.pos += 2;
                    return Some(val);
                }
                Instruction::Dbv => self.b = self.a / 2i64.pow(self.get_combo(operand) as u32),
                Instruction::Cdv => self.c = self.a / 2i64.pow(self.get_combo(operand) as u32),
            }
            self.pos += 2;
        }
        None
    }

    fn get_combo(&self, op: i64) -> i64 {
        match op {
            0..=3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Dbv,
    Cdv,
}

impl From<i64> for Instruction {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Dbv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

fn simulate_whole(a: i64, b: i64, c: i64, program: Vec<i64>) -> Vec<i64> {
    let mut computer = Computer::new(a, b, c, program);
    let mut result = Vec::new();
    while let Some(output) = computer.run() {
        result.push(output);
    }

    result
}

fn part2(input: &str) -> i64 {
    let (_, b, c, program) = parse(input);
    let mut a = 0;
    for n in 1..=program.len() {
        let target: Vec<i64> = program[program.len() - n..].to_vec();

        let mut new_a = a << 3;
        loop {
            let output = simulate_whole(new_a, b, c, program.clone());
            if output == target {
                a = new_a;
                break;
            }
            new_a += 1;
        }
    }

    a
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const SAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE2), 117440);
    }
}
