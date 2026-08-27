use std::collections::HashMap;

struct Reaction {
    inputs: Vec<(String, usize)>,
    output_amount: usize,
}

fn parse(input: &str) -> HashMap<String, Reaction> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (ins, out) = line.split_once("=>").unwrap();
            let inputs = ins
                .split(',')
                .map(|part| {
                    let (n, name) = part.trim().split_once(' ').unwrap();
                    (name.to_string(), n.parse().unwrap())
                })
                .collect();

            let (n, name) = out.trim().split_once(' ').unwrap();
            (
                name.to_string(),
                Reaction {
                    inputs,
                    output_amount: n.parse().unwrap(),
                },
            )
        })
        .collect()
}

fn ore_needed(
    name: &str,
    amount: usize,
    rules: &HashMap<String, Reaction>,
    surplus: &mut HashMap<String, usize>,
) -> usize {
    if name == "ORE" {
        return amount;
    }

    let have = surplus.entry(name.to_string()).or_insert(0);
    if *have >= amount {
        *have -= amount;
        return 0;
    }
    let amount = amount - *have;
    *have = 0;

    let reaction = &rules[name];
    let times = amount.div_ceil(reaction.output_amount);
    let produced = times * reaction.output_amount;
    *surplus.get_mut(name).unwrap() += produced - amount;

    reaction
        .inputs
        .iter()
        .map(|(n, c)| ore_needed(n, c * times, rules, surplus))
        .sum()
}

fn part1(input: &str) -> usize {
    let rules = parse(input);
    ore_needed("FUEL", 1, &rules, &mut HashMap::new())
}

fn part2(input: &str) -> usize {
    let rules = parse(input);
    let target: usize = 1_000_000_000_000;
    let ore_per_fuel = ore_needed("FUEL", 1, &rules, &mut HashMap::new());

    let mut lo = target / ore_per_fuel;
    let mut hi = 2 * target / ore_per_fuel;

    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        let ore = ore_needed("FUEL", mid, &rules, &mut HashMap::new());
        if ore <= target {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }

    lo
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        assert_eq!(part1(input), 31);
    }
}
