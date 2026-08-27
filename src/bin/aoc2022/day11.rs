#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    test: usize,
    t: usize,
    f: usize,
}

impl Monkey {
    fn apply(&self, old: usize) -> usize {
        let parts: Vec<&str> = self.operation.split_whitespace().collect();
        let operand = |s: &str| s.parse().unwrap_or(old);
        let (a, b) = (operand(parts[0]), operand(parts[2]));

        match parts[1] {
            "*" => a * b,
            "+" => a + b,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.lines().collect();
        let items = parts[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();
        let operation = parts[2].split("= ").last().unwrap().to_string();
        let test = parts[3].split_whitespace().last().unwrap().parse().unwrap();
        let t = parts[4].split_whitespace().last().unwrap().parse().unwrap();
        let f = parts[5].split_whitespace().last().unwrap().parse().unwrap();

        Self {
            items,
            operation,
            test,
            t,
            f,
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::from).collect()
}

fn simulate(monkeys: &[Monkey], relief: impl Fn(usize) -> usize, rounds: usize) -> usize {
    let mut monkeys = monkeys.to_vec();
    let mut counts = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            counts[i] += items.len();

            for item in items {
                let item = relief(monkeys[i].apply(item));
                let target = if item.is_multiple_of(monkeys[i].test) {
                    monkeys[i].t
                } else {
                    monkeys[i].f
                };
                monkeys[target].items.push(item);
            }
        }
    }

    counts.sort_unstable();
    counts.iter().rev().take(2).product()
}

fn part1(input: &str) -> usize {
    let monkeys = parse(input);
    simulate(&monkeys, |w| w / 3, 20)
}

fn part2(input: &str) -> usize {
    let monkeys = parse(input);
    let modulus: usize = monkeys.iter().map(|m| m.test).product();
    simulate(&monkeys, |w| w % modulus, 10_000)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2713310158);
    }
}
