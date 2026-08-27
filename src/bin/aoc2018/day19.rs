use adventofcode::util::elfcode::{OpCode, execute};

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let lines: Vec<&str> = input.lines().collect();
    let ip_register = lines[0].split_whitespace().last().unwrap().parse().unwrap();
    let instructions = lines
        .iter()
        .skip(1)
        .map(|&l| Instruction::from(l))
        .collect();

    (ip_register, instructions)
}

fn part1(input: &str) -> usize {
    let (ip_register, instructions) = parse(input);
    let mut device = Device::new(ip_register);
    device.simulate(&instructions);
    device.regs[0]
}

struct Instruction {
    op_code: OpCode,
    values: Vec<usize>,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        let op_code = split[0].into();
        let values = split.iter().skip(1).map(|l| l.parse().unwrap()).collect();
        Self { op_code, values }
    }
}

struct Device {
    regs: Vec<usize>,
    ip_register: usize,
    position: usize,
}

impl Device {
    fn new(ip_register: usize) -> Self {
        Self {
            regs: vec![0; 6],
            ip_register,
            position: 0,
        }
    }

    fn simulate(&mut self, instructions: &[Instruction]) {
        while self.position < instructions.len() {
            let instruction = &instructions[self.position];
            self.regs[self.ip_register] = self.position;
            self.regs = execute(
                instruction.op_code,
                &self.regs,
                instruction.values[0],
                instruction.values[1],
                instruction.values[2],
            );
            self.position = self.regs[self.ip_register] + 1;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test() {
        let s = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

        assert_eq!(6, part1(s));
    }
}
