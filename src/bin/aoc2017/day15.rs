fn parse(input: &str) -> (usize, usize) {
    let starts: Vec<usize> = input
        .lines()
        .map(|l| l.split(' ').next_back().unwrap().parse().unwrap())
        .collect();

    (starts[0], starts[1])
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut generator_a = Generator::new(input.0, 16807);
    let mut generator_b = Generator::new(input.1, 48271);

    let mut count = 0;

    for _ in 0..40_000_000 {
        let a = generator_a.next().unwrap();
        let b = generator_b.next().unwrap();

        if a & 65_535 == b & 65_535 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut generator_a = SecondGenerator::new(input.0, 16807, 4);
    let mut generator_b = SecondGenerator::new(input.1, 48271, 8);

    let mut count = 0;

    for _ in 0..5_000_000 {
        let a = generator_a.next().unwrap();
        let b = generator_b.next().unwrap();

        if a & 65_535 == b & 65_535 {
            count += 1;
        }
    }

    count
}

#[derive(Debug, Copy, Clone)]
struct Generator {
    factor: usize,
    value: usize,
}

impl Generator {
    fn new(value: usize, factor: usize) -> Self {
        Generator { factor, value }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let new_val = (self.value * self.factor) % 2147483647;

        self.value = new_val;
        Some(new_val)
    }
}

struct SecondGenerator {
    value: usize,
    factor: usize,
    criteria: usize,
}

impl SecondGenerator {
    fn new(value: usize, factor: usize, criteria: usize) -> Self {
        Self {
            value,
            factor,
            criteria,
        }
    }
}

impl Iterator for SecondGenerator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut new_val = (self.value * self.factor) % 2147483647;
        while !new_val.is_multiple_of(self.criteria) {
            self.value = new_val;
            new_val = (self.value * self.factor) % 2147483647;
        }
        self.value = new_val;

        Some(new_val)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Generator A starts with 65\nGenerator B starts with 8921";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 309);
    }
}
