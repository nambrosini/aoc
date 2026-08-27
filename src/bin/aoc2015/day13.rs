use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type MapType = HashMap<String, HashMap<String, i32>>;

fn parse(input: &str) -> MapType {
    let lines = input.lines();
    let mut map: MapType = HashMap::new();

    for next in lines {
        let split = next.split(' ').collect_vec();
        let name = split[0];
        let points: i32 = match split[2] {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        } * split[3].parse::<i32>().unwrap();
        let neighbour = split.iter().last().unwrap().replace('.', "");

        let entry = map.entry(name.to_owned()).or_default();
        entry.insert(neighbour, points);
    }

    map
}

fn part1(input: &str) -> i32 {
    let map = parse(input);
    let happiness = calc_happiness(&map);

    *happiness.iter().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut map = parse(input);
    let mut my = HashMap::new();

    for p in map.keys() {
        my.insert(p.to_owned(), 0);
    }

    for v in map.values_mut() {
        v.insert("Me".to_owned(), 0);
    }

    map.insert("Me".to_owned(), my);

    let happiness = calc_happiness(&map);

    *happiness.iter().max().unwrap()
}

fn calc_happiness(map: &MapType) -> HashSet<i32> {
    let perm = map.keys().permutations(map.len()).collect_vec();

    let mut set = HashSet::new();

    for p in perm {
        let mut sum = 0;

        for i in 0..p.len() {
            sum += map[p[i]][p[(i + 1) % p.len()]];
            let rev = p.len() - 1 - i;
            sum += map[p[rev]][p[match rev.checked_sub(1) {
                Some(v) => v,
                None => p.len() - 1,
            }]];
        }

        set.insert(sum);
    }

    set
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 330);
    }
}
