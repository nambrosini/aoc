use itertools::Itertools;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut tablet = Tablet::new(&input);
    tablet.run()
}

// The original solution never implemented part 2 (naively executing the
// program with register a=1 is intractable; it requires decompiling it into
// the composite-number-counting loop it actually implements). Ported as-is
// rather than fabricating an answer.
fn part2(_input: &str) -> i32 {
    0
}

struct Tablet {
    instructions: Vec<Instruction>,
    memory: [i32; 8],
}

impl Tablet {
    fn new(input: &[Instruction]) -> Tablet {
        Self {
            instructions: input.to_vec(),
            memory: [0; 8],
        }
    }

    fn run(&mut self) -> usize {
        let mut position = 0;
        let mut mults = 0;

        while position < self.instructions.len() {
            match self.instructions[position] {
                Instruction::Set(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] = y;
                    }
                    position += 1;
                }
                Instruction::Sub(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] -= y;
                    }
                    position += 1;
                }
                Instruction::Mul(x, y) => {
                    if let Param::Register(x) = x {
                        let index = Self::get_index(x);
                        let y = self.mem_get(y);

                        self.memory[index] *= y;
                    }
                    position += 1;
                    mults += 1;
                }
                Instruction::Jnz(x, y) => {
                    let x = self.mem_get(x);
                    let y = self.mem_get(y);

                    if x != 0 {
                        position = (position as i64 + y as i64) as usize;
                    } else {
                        position += 1;
                    }
                }
            }
        }

        mults
    }

    fn mem_get(&self, p: Param) -> i32 {
        match p {
            Param::Register(r) => self.memory.get(Self::get_index(r)).copied().unwrap_or(0),
            Param::Value(v) => v,
        }
    }

    fn get_index(register: char) -> usize {
        (register as u8 - 97u8) as usize
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Set(Param, Param),
    Sub(Param, Param),
    Mul(Param, Param),
    Jnz(Param, Param),
}

impl From<&str> for Instruction {
    fn from(c: &str) -> Instruction {
        let split = c.split(' ').collect_vec();

        match split[0] {
            "set" => Instruction::Set(split[1].into(), split[2].into()),
            "sub" => Instruction::Sub(split[1].into(), split[2].into()),
            "mul" => Instruction::Mul(split[1].into(), split[2].into()),
            "jnz" => Instruction::Jnz(split[1].into(), split[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Param {
    Register(char),
    Value(i32),
}

impl From<&str> for Param {
    fn from(s: &str) -> Self {
        match s.parse::<i32>() {
            Ok(v) => Param::Value(v),
            Err(_) => Param::Register(s.chars().next().unwrap()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/23.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
