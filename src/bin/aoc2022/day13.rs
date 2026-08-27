use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

fn parse_packet(s: &str) -> (Packet, &str) {
    if let Some(rest) = s.strip_prefix('[') {
        let mut items = vec![];
        let mut rest = rest;

        while !rest.starts_with(']') {
            let (item, r) = parse_packet(rest);
            items.push(item);
            rest = r.strip_prefix(',').unwrap_or(r);
        }

        (Packet::List(items), &rest[1..])
    } else {
        let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
        (Packet::Int(s[..end].parse().unwrap()), &s[end..])
    }
}

fn parse(input: &str) -> Vec<Packet> {
    input
        .split("\n\n")
        .flat_map(|pair| pair.lines().map(|l| parse_packet(l).0))
        .collect()
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(_), Packet::Int(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Int(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(l), Packet::List(r)) => {
                for (e1, e2) in l.iter().zip(r) {
                    match e1.cmp(e2) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                l.len().cmp(&r.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> usize {
    parse(input)
        .chunks(2)
        .enumerate()
        .filter(|(_, v)| v[0] < v[1])
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = parse(input);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.extend_from_slice(&[div1.clone(), div2.clone()]);

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == div1 || **p == div2)
        .map(|(i, _)| i + 1)
        .product()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 140);
    }
}
