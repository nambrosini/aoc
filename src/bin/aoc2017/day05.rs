fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut cpu = Cpu::new(&input);

    cpu.run()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut cpu = Cpu::new(&input);

    cpu.run_part2()
}

struct Cpu {
    memory: Vec<i32>,
}

impl Cpu {
    fn new(memory: &[i32]) -> Self {
        Self {
            memory: memory.to_vec(),
        }
    }

    fn run(&mut self) -> usize {
        let mut steps = 0;
        let mut address = 0;

        while address < self.memory.len() {
            let new_address = (address as i64 + self.memory[address] as i64) as usize;
            self.memory[address] += 1;
            address = new_address;

            steps += 1;
        }

        steps
    }

    fn run_part2(&mut self) -> usize {
        let mut steps = 0;
        let mut address = 0;

        while address < self.memory.len() {
            let new_address = (address as i64 + self.memory[address] as i64) as usize;
            if self.memory[address] >= 3 {
                self.memory[address] -= 1;
            } else {
                self.memory[address] += 1;
            }
            address = new_address;

            steps += 1;
        }

        steps
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0\n3\n0\n1\n-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 10);
    }
}
