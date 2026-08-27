use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    solve(&map, find_min_route)
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    solve(&map, find_min_route2)
}

type RouteFn = fn(&char, &char, &HashMap<(char, char), usize>, &[char]) -> usize;

fn solve(map: &[Vec<char>], route_fn: RouteFn) -> usize {
    let keys = find_keys(map);

    let mut routes: HashMap<(char, char), usize> = HashMap::new();

    for i in 0..keys.len() {
        for j in i + 1..keys.len() {
            let len = check_nodes(&keys[i].1, &keys[j].1, map);
            routes.insert((keys[i].0, keys[j].0), len);
            routes.insert((keys[j].0, keys[i].0), len);
        }
    }

    let keys = keys
        .iter()
        .filter(|(k, _)| k != &'0')
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    let mut min = usize::MAX;

    for k in keys.iter() {
        let new_keys = keys
            .iter()
            .filter(|&x| x != k)
            .map(|&x| *x)
            .collect::<Vec<char>>();

        min = min.min(route_fn(&'0', k, &routes, &new_keys));
    }

    min
}

fn find_min_route(
    start: &char,
    end: &char,
    map: &HashMap<(char, char), usize>,
    keys: &[char],
) -> usize {
    let len = map[&(*start, *end)];

    if keys.is_empty() {
        return len;
    }

    let keys = keys
        .iter()
        .filter(|&x| x != end)
        .copied()
        .collect::<Vec<char>>();

    let mut min = usize::MAX;

    for k in keys.iter() {
        let new_keys = keys
            .iter()
            .filter(|&x| x != k)
            .copied()
            .collect::<Vec<char>>();

        let route = find_min_route(end, k, map, &new_keys);

        min = min.min(route);
    }

    min + len
}

fn find_min_route2(
    start: &char,
    end: &char,
    map: &HashMap<(char, char), usize>,
    keys: &[char],
) -> usize {
    let len = map[&(*start, *end)];

    if keys.is_empty() {
        return len + map[&(*end, '0')];
    }

    let keys = keys
        .iter()
        .filter(|&x| x != end)
        .copied()
        .collect::<Vec<char>>();

    let mut min = usize::MAX;

    for k in keys.iter() {
        let new_keys = keys
            .iter()
            .filter(|&x| x != k)
            .copied()
            .collect::<Vec<char>>();

        let route = find_min_route2(end, k, map, &new_keys);

        min = min.min(route);
    }

    min + len
}

fn find_keys(map: &[Vec<char>]) -> Vec<(char, (i32, i32))> {
    let mut vec = Vec::new();

    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c != '.' && c != '#' {
                vec.push((c, (i as i32, j as i32)));
            }
        }
    }

    vec
}

type Point = (i32, i32);

fn is_open((x, y): &(i32, i32), map: &[Vec<char>]) -> bool {
    (0..map.len() as i32).contains(x)
        && (0..map[0].len() as i32).contains(y)
        && map[*x as usize][*y as usize] != '#'
}

fn check_nodes(start: &Point, end: &Point, map: &[Vec<char>]) -> usize {
    let mut open_list: Vec<(Point, usize)> = Vec::new();
    let mut closed_list: Vec<(Point, usize)> = Vec::new();

    open_list.push((*start, 0));

    while !open_list.is_empty() {
        let mut current_node = open_list[0];
        let mut current_index = 0;

        for (index, &item) in open_list.iter().enumerate() {
            if item.1 < current_node.1 {
                current_node = item;
                current_index = index;
            }
        }

        open_list.remove(current_index);

        if current_node.0 == *end {
            return current_node.1;
        }
        closed_list.push(current_node);

        let children = get_neighbours(&current_node.0, map);

        for child in children {
            if closed_list.iter().any(|(c, _)| c == &child) {
                continue;
            }

            if let Some((i, c)) = open_list.iter().enumerate().find(|(_, c)| c.0 == child) {
                if c.1 > current_node.1 {
                    open_list.remove(i);
                } else {
                    continue;
                }
            }
            open_list.push((child, current_node.1 + 1));
        }
    }

    unreachable!()
}

fn get_neighbours(current: &Point, map: &[Vec<char>]) -> Vec<Point> {
    let neighbours = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    neighbours
        .iter()
        .map(|&p| (current.0 + p.0, current.1 + p.1))
        .filter(|x| is_open(x, map))
        .collect_vec()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/24.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########";

        assert_eq!(part1(s), 14);
    }
}
