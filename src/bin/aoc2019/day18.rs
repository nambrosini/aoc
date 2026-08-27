use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Wall,
    Open,
    Door(char),
    Key(char),
    Entrance,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Open,
            '@' => Cell::Entrance,
            c if c.is_lowercase() => Cell::Key(c),
            c if c.is_uppercase() => Cell::Door(c),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect()
}

fn find_entrance(map: &[Vec<Cell>]) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Cell::Entrance {
                return (x, y);
            }
        }
    }
    unreachable!()
}

/// BFS from `start`, returning every reachable, not-yet-held key with its distance.
fn reachable_keys(
    start: (usize, usize),
    map: &[Vec<Cell>],
    keys: &HashSet<char>,
) -> Vec<(char, usize, (usize, usize))> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::from([start]);
    let mut found = vec![];

    while let Some(((x, y), dist)) = queue.pop_front() {
        match map[y][x] {
            Cell::Key(k) if !keys.contains(&k) => {
                found.push((k, dist, (x, y)));
                continue;
            }
            Cell::Door(d) if !keys.contains(&d.to_ascii_lowercase()) => continue,
            Cell::Wall => continue,
            _ => {}
        }

        for (dx, dy) in [(-1i32, 0), (1, 0), (0, -1i32), (0, 1)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || ny as usize >= map.len() || nx as usize >= map[0].len() {
                continue;
            }
            let next = (nx as usize, ny as usize);
            if visited.insert(next) {
                queue.push_back((next, dist + 1));
            }
        }
    }

    found
}

fn fewest_steps(start: (usize, usize), map: &[Vec<Cell>], total_keys: usize) -> usize {
    let mut cache = std::collections::HashMap::new();
    search(start, map, &HashSet::new(), total_keys, &mut cache)
}

fn search(
    pos: (usize, usize),
    map: &[Vec<Cell>],
    keys: &HashSet<char>,
    total_keys: usize,
    cache: &mut std::collections::HashMap<(Vec<char>, (usize, usize)), usize>,
) -> usize {
    if keys.len() == total_keys {
        return 0;
    }

    let mut sorted_keys: Vec<char> = keys.iter().copied().collect();
    sorted_keys.sort_unstable();
    let cache_key = (sorted_keys, pos);
    if let Some(&v) = cache.get(&cache_key) {
        return v;
    }

    let best = reachable_keys(pos, map, keys)
        .into_iter()
        .map(|(k, dist, next_pos)| {
            let mut next_keys = keys.clone();
            next_keys.insert(k);
            dist + search(next_pos, map, &next_keys, total_keys, cache)
        })
        .min()
        .unwrap_or(usize::MAX);

    cache.insert(cache_key, best);
    best
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let start = find_entrance(&map);
    let total_keys = map
        .iter()
        .flatten()
        .filter(|c| matches!(c, Cell::Key(_)))
        .count();

    fewest_steps(start, &map, total_keys)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/18.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "#########\n#b.A.@.a#\n#########";
        assert_eq!(part1(input), 8);
    }
}
