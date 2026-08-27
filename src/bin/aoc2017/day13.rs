use std::collections::HashMap;

type Firewall = HashMap<usize, Layer>;

fn parse(input: &str) -> Firewall {
    input
        .lines()
        .map(|l| {
            let depth = l.split(':').next().unwrap().parse().unwrap();
            (depth, Layer::from(l))
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input.iter().fold(0, |acc, x| {
        acc + if x.1.caught(0) { x.1.get_severity() } else { 0 }
    })
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut delay = 0;
    let mut sum = usize::MAX;

    while sum > 0 {
        sum = input
            .iter()
            .fold(0, |acc, x| acc + usize::from(x.1.caught(delay)));

        delay += 1;
    }

    delay - 1
}

struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn new(depth: usize, range: usize) -> Self {
        Self { depth, range }
    }

    fn caught(&self, delay: usize) -> bool {
        let period = 2 * (self.range - 1);
        (self.depth + delay).is_multiple_of(period)
    }

    fn get_severity(&self) -> usize {
        self.depth * self.range
    }
}

impl From<&str> for Layer {
    fn from(value: &str) -> Self {
        let mut split = value.split(": ");
        let depth = split.next().unwrap().parse().unwrap();
        let range = split.next().unwrap().parse().unwrap();
        Layer::new(depth, range)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 10);
    }
}
