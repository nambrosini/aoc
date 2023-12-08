use std::collections::HashMap;

use num::Integer;

advent_of_code::solution!(8);

struct Input {
    dirs: Vec<char>,
    map: HashMap<String, (String, String)>,
}

fn parse(input: &str) -> Input {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let dirs = chunks[0].chars().collect();
    let mut map = HashMap::new();

    for l in chunks[1].lines() {
        let split: Vec<&str> = l.split(" = ").collect();
        let key = split[0].to_string();
        let vals: Vec<String> = split[1][1..split[1].len() - 1]
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        map.insert(key, (vals[0].clone(), vals[1].clone()));
    }

    Input { dirs, map }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut i = 0;
    let mut current: &str = "AAA";
    let mut count = 0;

    while current != "ZZZ" {
        let dir = input.dirs[i];
        let node = &input.map[current];

        current = match dir {
            'L' => &node.0,
            'R' => &node.1,
            _ => unreachable!(),
        };

        i = (i + 1) % input.dirs.len();
        count += 1;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    let mut current: Vec<String> = get_starting_nodes(&input.map);
    let mut next: Vec<String> = vec![];
    let mut i = 0;
    let mut count = 1;
    let mut steps: Vec<u64> = vec![];

    while !current.is_empty() {
        let dir = input.dirs[i];
        for c in current.iter() {
            let node = &input.map[c];

            let n = match dir {
                'L' => &node.0,
                'R' => &node.1,
                _ => unreachable!(),
            }
            .to_string();

            if n.ends_with('Z') {
                steps.push(count);
            } else {
                next.push(n);
            }
        }
        current = next.clone();
        next.clear();
        i = (i + 1) % input.dirs.len();
        count += 1;
    }

    Some(steps.iter().fold(1u64, |a, b| a.lcm(b)))
}

fn get_starting_nodes(map: &HashMap<String, (String, String)>) -> Vec<String> {
    map.keys().filter(|n| n.ends_with('A')).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
