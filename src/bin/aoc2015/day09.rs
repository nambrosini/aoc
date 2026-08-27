use itertools::Itertools as _;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    vertex: String,
    distance: usize,
}

impl Edge {
    fn new(vertex: &str, distance: usize) -> Self {
        Self {
            vertex: vertex.to_owned(),
            distance,
        }
    }
}

type MapType = HashMap<String, Vec<Edge>>;

fn parse(input: &str) -> MapType {
    let mut map: MapType = HashMap::new();

    for l in input.lines() {
        let split = l.split(' ').collect_vec();
        let dist = split.last().unwrap().parse::<usize>().unwrap();
        let from = split[0].to_string();
        let to = split[2].to_string();

        let val = map.entry(from.clone()).or_default();
        val.push(Edge::new(&to, dist));

        let val = map.entry(to).or_default();
        val.push(Edge::new(&from, dist));
    }

    map
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let distances = calc_distances(&map);

    *distances.iter().min().unwrap()
}

fn calc_distances(map: &MapType) -> HashSet<usize> {
    let mut distances = HashSet::new();
    let mut current_town: String;

    for (start, vertex) in map {
        for perm in vertex.iter().permutations(vertex.len()).unique() {
            current_town = start.to_owned();
            let mut sum = 0;
            for edge in perm {
                sum += map
                    .get(&current_town)
                    .unwrap()
                    .iter()
                    .filter(|e| e.vertex == edge.vertex)
                    .fold(0, |acc, e| acc + e.distance);
                current_town = edge.vertex.to_owned();
            }

            distances.insert(sum);
        }
    }
    distances
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let distances = calc_distances(&map);

    *distances.iter().max().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 605);
    }
}
