use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

type Map = HashMap<String, Vec<String>>;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = Map::new();
    for l in input.lines() {
        let first = l[0..2].to_string();
        let second = l[3..].to_string();

        let entry = map.entry(first.clone()).or_default();
        entry.push(second.clone());
        let entry = map.entry(second).or_default();
        entry.push(first);
    }

    map
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let ts = map.keys().filter(|k| k.starts_with("t"));

    let mut res = vec![];
    for t in ts {
        res.append(&mut find_triplets(&map, t));
    }

    res.iter().unique().count()
}

fn part2(input: &str) -> String {
    let map = parse(input);

    let mut max = 0;
    let mut biggest_party = vec![];

    for k in map.keys() {
        for k2 in map.get(k).unwrap() {
            let party = find_party(&map, std::slice::from_ref(k), k2);
            if max < party.len() {
                max = party.len();
                biggest_party = party;
            }
        }
    }

    biggest_party.sort();

    biggest_party.join(",")
}

fn find_party(map: &Map, party: &[String], pc: &String) -> Vec<String> {
    let conns: Vec<String> = map.get(pc).unwrap().clone();
    let mut vec = party.to_vec();

    for v in party {
        if !conns.contains(v) {
            return vec;
        }
    }

    vec.push(pc.clone());

    for v in conns {
        if !vec.contains(&v) {
            return find_party(map, &vec, &v);
        }
    }
    vec
}

fn find_triplets(map: &Map, key: &String) -> Vec<Triplet> {
    let mut res = Vec::new();
    let vs = map.get(key).unwrap();
    for second in vs {
        let v2 = map.get(second).unwrap();
        for third in v2 {
            if map.get(third).unwrap().contains(key) {
                let t = Triplet::new(key.clone(), second.clone(), third.clone());
                res.push(t);
            }
        }
    }

    res
}

struct Triplet {
    first: String,
    second: String,
    third: String,
}

impl Triplet {
    fn new(first: String, second: String, third: String) -> Self {
        Triplet {
            first,
            second,
            third,
        }
    }
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Self) -> bool {
        let this: HashSet<_> = [&self.first, &self.second, &self.third]
            .into_iter()
            .collect();
        let other: HashSet<_> = [&other.first, &other.second, &other.third]
            .into_iter()
            .collect();
        this == other
    }
}

impl Hash for Triplet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut fields = vec![&self.first, &self.second, &self.third];
        fields.sort();
        for field in fields {
            field.hash(state);
        }
    }
}

impl Eq for Triplet {}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/23.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_find_triplets() {
        let input = "tb-bc\ncd-bc\ntb-cd";
        let res = part1(input);

        assert_eq!(res, 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), "co,de,ka,ta");
    }
}
