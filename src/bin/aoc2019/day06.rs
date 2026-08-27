use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.trim().lines() {
        let (center, orbiter) = line.split_once(')').unwrap();
        map.entry(center.to_string())
            .or_default()
            .push(orbiter.to_string());
    }

    map
}

fn count_orbits(start: &str, map: &HashMap<String, Vec<String>>, depth: usize) -> usize {
    let Some(children) = map.get(start) else {
        return depth;
    };

    depth
        + children
            .iter()
            .map(|c| count_orbits(c, map, depth + 1))
            .sum::<usize>()
}

fn path_to_root<'a>(mut node: &'a str, parents: &HashMap<&'a str, &'a str>) -> Vec<&'a str> {
    let mut path = vec![];
    while let Some(&parent) = parents.get(node) {
        path.push(parent);
        node = parent;
    }
    path
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    count_orbits("COM", &map, 0)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let parents: HashMap<&str, &str> = map
        .iter()
        .flat_map(|(center, orbiters)| orbiters.iter().map(move |o| (o.as_str(), center.as_str())))
        .collect();

    let you_path = path_to_root("YOU", &parents);
    let san_path: HashSet<&str> = path_to_root("SAN", &parents).into_iter().collect();

    let you_dist = you_path.iter().position(|p| san_path.contains(p)).unwrap();
    let common = you_path[you_dist];
    let san_dist = path_to_root("SAN", &parents)
        .iter()
        .position(|&p| p == common)
        .unwrap();

    you_dist + san_dist
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(part1(input), 42);
    }

    #[test]
    fn test_part2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        assert_eq!(part2(input), 4);
    }
}
