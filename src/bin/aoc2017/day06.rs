use itertools::Itertools;

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut debugger = Debugger::new(&input);

    let (history, _) = debugger.run();
    history.len() - 1
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut debugger = Debugger::new(&input);

    let (history, repeat_index) = debugger.run();
    history.len() - 1 - repeat_index
}

struct Debugger {
    memory: Vec<usize>,
}

impl Debugger {
    fn new(memory: &[usize]) -> Self {
        Self {
            memory: memory.to_vec(),
        }
    }

    fn step(&mut self) {
        let max = get_max_index(&self.memory);
        let val = max.1;

        let mut address = (max.0 + 1) % self.memory.len();
        self.memory[max.0] = 0;

        for _ in 0..val {
            self.memory[address] += 1;
            address = (address + 1) % self.memory.len();
        }
    }

    /// Runs until a configuration repeats, returning the full history
    /// (including that final, repeated configuration) along with the index
    /// of its first occurrence.
    fn run(&mut self) -> (Vec<String>, usize) {
        let mut v = vec![self.memory.iter().map(|x| x.to_string()).join(" ")];

        loop {
            self.step();

            let joined = self.memory.iter().map(|x| x.to_string()).join(" ");

            if let Some(repeat_index) = v.iter().position(|x| x == &joined) {
                v.push(joined);
                return (v, repeat_index);
            }

            v.push(joined);
        }
    }
}

fn get_max_index(memory: &[usize]) -> (usize, usize) {
    let mut max = memory[0];
    let mut index = 0;

    for (i, e) in memory.iter().enumerate().skip(1) {
        if *e > max {
            max = *e;
            index = i;
        }
    }

    (index, max)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("0 2 7 0"), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("0 2 7 0"), 4);
    }
}
