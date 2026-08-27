use std::collections::{HashMap, VecDeque};

pub fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub enum Step {
    Output(i64),
    NeedInput,
    Halted,
}

#[derive(Clone)]
pub struct Intcode {
    mem: HashMap<i64, i64>,
    pos: i64,
    relative_base: i64,
    inputs: VecDeque<i64>,
}

impl Intcode {
    pub fn new(program: &[i64]) -> Self {
        Self {
            mem: program
                .iter()
                .enumerate()
                .map(|(i, &v)| (i as i64, v))
                .collect(),
            pos: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
        }
    }

    pub fn poke(&mut self, addr: i64, value: i64) {
        self.mem.insert(addr, value);
    }

    pub fn peek(&self, addr: i64) -> i64 {
        self.get(addr)
    }

    pub fn push_input(&mut self, value: i64) {
        self.inputs.push_back(value);
    }

    /// Runs until the next output, a blocking need for input, or halt.
    pub fn run(&mut self) -> Step {
        loop {
            let instr = self.get(self.pos);
            let opcode = instr % 100;
            let m1 = instr / 100 % 10;
            let m2 = instr / 1000 % 10;
            let m3 = instr / 10000 % 10;

            match opcode {
                1 => {
                    let v = self.read(1, m1) + self.read(2, m2);
                    self.write(3, m3, v);
                    self.pos += 4;
                }
                2 => {
                    let v = self.read(1, m1) * self.read(2, m2);
                    self.write(3, m3, v);
                    self.pos += 4;
                }
                3 => {
                    let Some(v) = self.inputs.pop_front() else {
                        return Step::NeedInput;
                    };
                    self.write(1, m1, v);
                    self.pos += 2;
                }
                4 => {
                    let v = self.read(1, m1);
                    self.pos += 2;
                    return Step::Output(v);
                }
                5 => {
                    if self.read(1, m1) != 0 {
                        self.pos = self.read(2, m2);
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    if self.read(1, m1) == 0 {
                        self.pos = self.read(2, m2);
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    let v = (self.read(1, m1) < self.read(2, m2)) as i64;
                    self.write(3, m3, v);
                    self.pos += 4;
                }
                8 => {
                    let v = (self.read(1, m1) == self.read(2, m2)) as i64;
                    self.write(3, m3, v);
                    self.pos += 4;
                }
                9 => {
                    self.relative_base += self.read(1, m1);
                    self.pos += 2;
                }
                99 => return Step::Halted,
                _ => unreachable!("unknown opcode {opcode}"),
            }
        }
    }

    /// Runs to completion, ignoring any output produced along the way.
    pub fn run_to_halt(&mut self) {
        loop {
            match self.run() {
                Step::Halted => return,
                Step::Output(_) => {}
                Step::NeedInput => unreachable!("run_to_halt hit a blocking input read"),
            }
        }
    }

    /// Pushes `input`, then runs until the next output or halt.
    pub fn run_to_output(&mut self, input: i64) -> Option<i64> {
        self.push_input(input);
        match self.run() {
            Step::Output(v) => Some(v),
            Step::Halted => None,
            Step::NeedInput => unreachable!("run_to_output starved for input"),
        }
    }

    fn get(&self, addr: i64) -> i64 {
        *self.mem.get(&addr).unwrap_or(&0)
    }

    fn read(&self, offset: i64, mode: i64) -> i64 {
        match mode {
            0 => self.get(self.get(self.pos + offset)),
            1 => self.get(self.pos + offset),
            2 => self.get(self.get(self.pos + offset) + self.relative_base),
            _ => unreachable!("unknown mode {mode}"),
        }
    }

    fn write(&mut self, offset: i64, mode: i64, value: i64) {
        let addr = match mode {
            0 => self.get(self.pos + offset),
            2 => self.get(self.pos + offset) + self.relative_base,
            _ => unreachable!("unknown mode {mode}"),
        };
        self.mem.insert(addr, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut pc = Intcode::new(&program);
        let mut out = vec![];
        loop {
            match pc.run() {
                Step::Output(v) => out.push(v),
                Step::Halted => break,
                Step::NeedInput => unreachable!(),
            }
        }
        assert_eq!(out, program);
    }

    #[test]
    fn large_number() {
        let mut pc = Intcode::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(pc.run_to_output(0), Some(1219070632396864));
    }

    #[test]
    fn immediate_large_number() {
        let mut pc = Intcode::new(&[104, 1125899906842624, 99]);
        assert_eq!(pc.run_to_output(0), Some(1125899906842624));
    }

    #[test]
    fn day02_style() {
        let mut pc = Intcode::new(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        pc.run_to_halt();
        assert_eq!(pc.peek(0), 3500);
    }
}
