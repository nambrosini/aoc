use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags = HashMap::new();

    for line in input.lines() {
        let line = line.replace("bags", "").replace("bag", "").replace('.', "");
        let parts: Vec<&str> = line.trim().split("contain ").collect();

        let color = parts[0].trim().to_owned();

        let contents = parts[1]
            .split(',')
            .filter_map(|s| {
                let s = s.trim();
                if s == "no other" {
                    None
                } else {
                    let (n, rest) = s.split_at(1);
                    Some((n.parse().unwrap(), rest.trim().to_owned()))
                }
            })
            .collect();

        bags.insert(color, contents);
    }

    bags
}

fn part1(input: &str) -> usize {
    let bags = parse(input);

    let mut containing_gold: HashSet<String> = HashSet::new();
    let mut frontier = vec!["shiny gold".to_owned()];

    while let Some(color) = frontier.pop() {
        for (outer, contents) in &bags {
            if contents.iter().any(|(_, c)| *c == color) && containing_gold.insert(outer.clone()) {
                frontier.push(outer.clone());
            }
        }
    }

    containing_gold.len()
}

fn count_contents(color: &str, bags: &HashMap<String, Vec<(usize, String)>>) -> usize {
    bags[color]
        .iter()
        .map(|(n, c)| n * (1 + count_contents(c, bags)))
        .sum()
}

fn part2(input: &str) -> usize {
    let bags = parse(input);

    count_contents("shiny gold", &bags)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(part1(s), 4);
    }

    #[test]
    fn test_part2() {
        let s = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part2(s), 126);
    }
}
