use itertools::Itertools;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> i128 {
    let input = parse(input);
    let mut processor = Processor::new([0, 0], &input);
    processor.run();
    processor.registers[1]
}

fn part2(input: &str) -> i128 {
    let input = parse(input);
    let mut processor = Processor::new([1, 0], &input);
    processor.run();
    processor.registers[1]
}

#[derive(Debug, Clone)]
enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(i128),
    Jie(usize, i128),
    Jio(usize, i128),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let instruction = &s[..3];
        let other = &s[4..];

        match instruction {
            "hlf" => Instruction::Hlf(if other == "a" {
                0
            } else if other == "b" {
                1
            } else {
                unreachable!()
            }),
            "tpl" => Instruction::Tpl(if other == "a" {
                0
            } else if other == "b" {
                1
            } else {
                unreachable!()
            }),
            "inc" => Instruction::Inc(if other == "a" {
                0
            } else if other == "b" {
                1
            } else {
                unreachable!()
            }),
            "jmp" => Instruction::Jmp(other.parse().unwrap()),
            "jie" => {
                let split: Vec<&str> = other.split(", ").collect();
                let register = if split[0] == "a" {
                    0
                } else if other == "b" {
                    1
                } else {
                    unreachable!()
                };
                let offset = split[1].parse().unwrap();
                Instruction::Jie(register, offset)
            }
            "jio" => {
                let split: Vec<&str> = other.split(", ").collect();
                let register = if split[0] == "a" {
                    0
                } else if other == "b" {
                    1
                } else {
                    unreachable!()
                };
                let offset = split[1].parse().unwrap();
                Instruction::Jio(register, offset)
            }
            _ => unreachable!(),
        }
    }
}

struct Processor {
    registers: [i128; 2],
    instructions: Vec<Instruction>,
}

impl Processor {
    fn new(registers: [i128; 2], instructions: &[Instruction]) -> Self {
        Self {
            registers,
            instructions: instructions.to_vec(),
        }
    }

    fn run(&mut self) {
        let mut position: i128 = 0;

        while position < self.instructions.len() as i128 {
            match self.instructions[position as usize] {
                Instruction::Hlf(r) => {
                    self.registers[r] /= 2;
                    position += 1;
                }
                Instruction::Tpl(r) => {
                    self.registers[r] *= 3;
                    position += 1;
                }
                Instruction::Inc(r) => {
                    self.registers[r] += 1;
                    position += 1;
                }
                Instruction::Jmp(o) => {
                    position += o;
                }
                Instruction::Jie(r, o) => {
                    if self.registers[r] % 2 == 0 {
                        position += o;
                    } else {
                        position += 1;
                    }
                }
                Instruction::Jio(r, o) => {
                    if self.registers[r] == 1 {
                        position += o;
                    } else {
                        position += 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/23.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "inc a
jio a, +2
tpl a
inc a";

        let s = parse(s);
        let mut processor = Processor::new([0, 0], &s);
        processor.run();
        assert_eq!(processor.registers[0], 2);
    }
}
