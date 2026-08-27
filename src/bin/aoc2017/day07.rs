use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn part1(input: &str) -> String {
    let input = parse(input);

    let mut weights: HashMap<&str, u64> = HashMap::new();
    let mut tree: HashMap<&str, &str> = HashMap::new();

    let re1 = Regex::new(r"^(.+?) \((\d+)\) -> (.+)$").unwrap();
    let re2 = Regex::new(r"^(.+?) \((\d+)\)$").unwrap();
    for line in input.iter() {
        let cap = re1.captures(line);
        if let Some(cap) = cap {
            let parent = cap.get(1).unwrap().as_str();
            let weight: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let children = cap.get(3).unwrap().as_str();
            weights.insert(parent, weight);
            for child in children.split(", ") {
                tree.insert(child, parent);
            }
            continue;
        }
        let cap = re2.captures(line);
        if let Some(cap) = cap {
            let parent = cap.get(1).unwrap().as_str();
            let weight = cap.get(2).unwrap().as_str().parse().unwrap();
            weights.insert(parent, weight);
            continue;
        }
        panic!("invalid input: {}", line);
    }

    let mut s: &str = weights.keys().next().unwrap();
    while tree.contains_key(s) {
        s = tree.get(s).unwrap();
    }
    s.to_owned()
}

fn part2(input: &str) -> i64 {
    let lines = parse(input);

    let mut weights: HashMap<&str, i64> = HashMap::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();

    let re1 = Regex::new(r"^(.+?) \((\d+)\) -> (.+)$").unwrap();
    let re2 = Regex::new(r"^(.+?) \((\d+)\)$").unwrap();
    for line in lines.iter() {
        let cap = re1.captures(line);
        if let Some(cap) = cap {
            let parent = cap.get(1).unwrap().as_str();
            let weight: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let children = cap.get(3).unwrap().as_str();
            weights.insert(parent, weight);
            tree.insert(parent, children.split(", ").collect());
            continue;
        }
        let cap = re2.captures(line);
        if let Some(cap) = cap {
            let parent = cap.get(1).unwrap().as_str();
            let weight = cap.get(2).unwrap().as_str().parse().unwrap();
            weights.insert(parent, weight);
            continue;
        }
        panic!("invalid input: {}", line);
    }

    let root = part1(input);

    match check(&tree, &weights, &root) {
        Either::Solution(s) => s,
        _ => panic!("failed to find a solution."),
    }
}

enum Either {
    Weight(i64),
    Solution(i64),
}

// The check function calculates a node's weight + all its children.
fn check(tree: &HashMap<&str, Vec<&str>>, weights: &HashMap<&str, i64>, node: &str) -> Either {
    match tree.get(node) {
        Some(children) => {
            let mut r = vec![];
            for child in children.iter() {
                match check(tree, weights, child) {
                    s @ Either::Solution(_) => return s,
                    Either::Weight(child_weight) => r.push(child_weight),
                }
            }
            for i in 0..children.len() {
                let other1 = (i + 1) % children.len();
                let other2 = (i + 2) % children.len();
                if r[i] != r[other1] && r[i] != r[other2] {
                    return Either::Solution(weights.get(children[i]).unwrap() + r[other1] - r[i]);
                }
            }
            Either::Weight(r[0] * children.len() as i64 + weights.get(node).unwrap())
        }
        None => Either::Weight(*weights.get(node).unwrap()),
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "tknk");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 60);
    }
}
