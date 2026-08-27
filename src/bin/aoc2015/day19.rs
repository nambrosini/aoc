use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};

type MapType = (String, HashMap<String, Vec<String>>);

fn parse(input: &str) -> MapType {
    let split = input.split("\n\n").collect_vec();
    let molecule = split[1].trim().to_owned();

    let v: Vec<(String, String)> = split[0]
        .lines()
        .map(|x| x.split(" => ").collect_vec())
        .map(|x| (x[0].to_owned(), x[1].to_owned()))
        .collect();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in v {
        let entry = map.entry(k).or_default();
        entry.push(v);
    }

    (molecule, map)
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut set = HashSet::new();

    for (k, v) in map.1.iter() {
        for v in v {
            if k.len() == 1 {
                for i in 0..map.0.len() {
                    if map.0[i..=i] == k[..] {
                        let s = format!(
                            "{}{}{}",
                            map.0[..i].to_owned(),
                            v,
                            map.0[(i + 1)..].to_owned()
                        )
                        .to_owned();
                        set.insert(s);
                    }
                }
            } else {
                for i in 0..map.0.len() - 1 {
                    if &map.0[i..=(i + 1)] == k {
                        let s = format!(
                            "{}{}{}",
                            map.0[..i].to_owned(),
                            v,
                            map.0[(i + 2)..].to_owned()
                        )
                        .to_owned();
                        set.insert(s);
                    }
                }
            }
        }
    }

    set.len()
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut replacements = vec![];
    for (k, val) in &map.1 {
        for e in val {
            replacements.push((k.clone(), e));
        }
    }

    replacements.sort_by_key(|(_, v)| cmp::Reverse(v.len()));

    let mut total = 0;
    let mut medicine = map.0.clone();

    while medicine != "e" {
        for (rhs, lhs) in &replacements {
            if medicine.contains(*lhs) {
                medicine = medicine.replacen(*lhs, rhs, 1);
                total += 1;
                break;
            }
        }
    }

    total
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO";

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
