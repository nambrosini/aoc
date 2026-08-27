use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Register(char),
    Value(i32),
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s.parse::<i32>() {
            Ok(v) => Operator::Value(v),
            Err(_) => Operator::Register(s.chars().next().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Cpy(Operator, Operator),
    Inc(Operator),
    Dec(Operator),
    Jnz(Operator, Operator),
    Tgl(Operator),
    Out(Operator),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(' ').collect();

        match split[0] {
            "cpy" => Instruction::Cpy(split[1].into(), split[2].into()),
            "inc" => Instruction::Inc(split[1].into()),
            "dec" => Instruction::Dec(split[1].into()),
            "jnz" => Instruction::Jnz(split[1].into(), split[2].into()),
            "tgl" => Instruction::Tgl(split[1].into()),
            "out" => Instruction::Out(split[1].into()),
            _ => unreachable!(),
        }
    }
}

pub struct Computer {
    pub position: usize,
    pub memory: HashMap<char, i32>,
    instructions: Vec<Instruction>,
}

impl Computer {
    pub fn new(memory: HashMap<char, i32>, instructions: &[Instruction]) -> Self {
        Self {
            position: 0,
            memory,
            instructions: instructions.to_vec(),
        }
    }

    fn value(&self, op: Operator) -> i32 {
        match op {
            Operator::Register(k) => *self.memory.get(&k).unwrap_or(&0),
            Operator::Value(v) => v,
        }
    }

    /// Runs to completion, honoring `tgl` self-modification, and returns register `a`.
    pub fn run(&mut self) -> i32 {
        self.run_with_output_limit(None);
        *self.memory.get(&'a').unwrap_or(&0)
    }

    /// Runs until `out` has produced `limit` values (or the program halts), returning
    /// whatever was written by `out` along the way.
    pub fn run_collecting_output(&mut self, limit: usize) -> Vec<i32> {
        self.run_with_output_limit(Some(limit))
    }

    fn run_with_output_limit(&mut self, limit: Option<usize>) -> Vec<i32> {
        let mut output = Vec::new();

        while self.position < self.instructions.len() {
            if limit.is_some_and(|limit| output.len() >= limit) {
                break;
            }

            match self.instructions[self.position] {
                Instruction::Cpy(v1, v2) => {
                    if let Operator::Register(k) = v2 {
                        let v = self.value(v1);
                        self.memory.insert(k, v);
                    }
                }
                Instruction::Inc(k) => {
                    if let Operator::Register(k) = k {
                        *self.memory.entry(k).or_insert(0) += 1;
                    }
                }
                Instruction::Dec(k) => {
                    if let Operator::Register(k) = k {
                        *self.memory.entry(k).or_insert(0) -= 1;
                    }
                }
                Instruction::Jnz(x, y) => {
                    let x = self.value(x);
                    let y = self.value(y);
                    if x != 0 {
                        self.position = (self.position as i64 + y as i64 - 1) as usize;
                    }
                }
                Instruction::Out(x) => {
                    output.push(self.value(x));
                }
                Instruction::Tgl(v) => {
                    let position = self.position + self.value(v) as usize;

                    if let Some(ins) = self.instructions.get(position) {
                        self.instructions[position] = match *ins {
                            Instruction::Inc(v) => Instruction::Dec(v),
                            Instruction::Dec(v) | Instruction::Tgl(v) | Instruction::Out(v) => {
                                Instruction::Inc(v)
                            }
                            Instruction::Jnz(v1, v2) => Instruction::Cpy(v1, v2),
                            Instruction::Cpy(v1, v2) => Instruction::Jnz(v1, v2),
                        };
                    }
                }
            }

            self.position += 1;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn instructions(s: &str) -> Vec<Instruction> {
        s.lines().map(Instruction::from).collect()
    }

    #[test]
    fn test_basic_loop() {
        let ins = instructions(
            "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a",
        );
        let mut computer = Computer::new(HashMap::new(), &ins);
        assert_eq!(computer.run(), 42);
    }

    #[test]
    fn test_tgl() {
        let ins = instructions(
            "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a",
        );
        let mut computer = Computer::new(HashMap::new(), &ins);
        assert_eq!(computer.run(), 3);
    }
}
