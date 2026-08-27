use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();

        if b != "start" {
            map.entry(a.to_owned()).or_default().push(b.to_owned());
        }
        if a != "start" {
            map.entry(b.to_owned()).or_default().push(a.to_owned());
        }
    }

    map
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn count_paths(
    map: &HashMap<String, Vec<String>>,
    current: &str,
    visited: &[String],
    allow_revisit: bool,
) -> usize {
    if current == "end" {
        return 1;
    }

    let Some(neighbours) = map.get(current) else {
        return 0;
    };

    let mut count = 0;
    for next in neighbours {
        let mut path = visited.to_vec();
        path.push(current.to_owned());

        if is_small(next) && visited.contains(next) {
            if allow_revisit {
                count += count_paths(map, next, &path, false);
            }
        } else {
            count += count_paths(map, next, &path, allow_revisit);
        }
    }

    count
}

fn part1(input: &str) -> usize {
    count_paths(&parse(input), "start", &[], false)
}

fn part2(input: &str) -> usize {
    count_paths(&parse(input), "start", &[], true)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const MEDIUM: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const LARGE: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SMALL), 10);
        assert_eq!(part1(MEDIUM), 19);
        assert_eq!(part1(LARGE), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SMALL), 36);
        assert_eq!(part2(MEDIUM), 103);
    }
}
