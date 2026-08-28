use std::collections::HashMap;

use adventofcode::util::{
    direction::Direction,
    grid::{Find, Grid, Parse, astar},
};

const KEYPAD: &str = "789\n456\n123\nx0A";
const DIRECTIONAL: &str = "x^A\n<v>";

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &str) -> u32 {
    let codes = parse(input);

    let mut sum: usize = 0;

    for code in codes {
        let path = get_shortest_path(code);
        sum += path.len() * code[..code.len() - 1].parse::<usize>().unwrap();
    }

    sum as u32
}

fn get_shortest_path(code: &str) -> String {
    let keypad: Grid<char> = Grid::parse(KEYPAD);
    let directional: Grid<char> = Grid::parse(DIRECTIONAL);

    let path = path_to_code(&keypad, code);
    let path = path_to_code(&directional, &path);
    path_to_code(&directional, &path)
}

fn path_to_code(grid: &Grid<char>, code: &str) -> String {
    let mut start = grid.find(&'A');

    let mut path: String = String::new();
    for c in code.chars() {
        let goal = grid.find(&c);
        let p = astar(grid, start, goal, Some('x')).expect("Path not found");
        start = *p.iter().last().unwrap();
        let mut current_p = String::new();
        for i in 0..p.len() - 1 {
            let dir: Direction = (p[i + 1] - p[i]).into();
            current_p.push_str(&dir.to_string());
        }
        current_p = optimize(&current_p);
        path.push_str(&current_p);
        path.push('A');
    }

    path
}

fn optimize(path: &str) -> String {
    let mut map = HashMap::new();
    for c in path.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }

    if let Some(up) = map.get(&'^')
        && let Some(down) = map.get(&'v')
    {
        if up - down > 0 {
            map.insert('^', up - down);
            map.remove(&'v');
        } else if up - down == 0 {
            map.remove(&'^');
            map.remove(&'v');
        } else {
            map.insert('v', down - up);
            map.remove(&'v');
        }
    }

    if let Some(left) = map.get(&'<')
        && let Some(right) = map.get(&'>')
    {
        if left - right > 0 {
            map.insert('<', left - right);
            map.remove(&'>');
        } else if left - right == 0 {
            map.remove(&'<');
            map.remove(&'>');
        } else {
            map.insert('>', right - left);
            map.remove(&'>');
        }
    }

    let best = ['<', 'v', '^', '>'].iter().rev();
    let mut res = String::new();
    for b in best {
        if let Some(count) = map.get(b) {
            for _ in 0..*count {
                res.push(*b);
            }
        }
    }

    res
}

fn part2(_: &str) -> u32 {
    // Never solved in the original: the recursive-robot-chain approach used
    // for part 1 doesn't scale to part 2's 25 levels of indirection.
    todo!("part 2 was never solved")
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/21.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_029a() {
        let keypad: Grid<char> = Grid::parse(KEYPAD);
        let directional: Grid<char> = Grid::parse(DIRECTIONAL);

        let actual = path_to_code(&keypad, "029A");
        let expected = "<A^A>^^AvvvA";
        assert_eq!(actual, expected);

        // Later stages of directional->directional pathing can hit ties
        // between equally-short paths, so only length is guaranteed to match.
        let actual = path_to_code(&directional, "<A^A>^^AvvvA");
        assert_eq!(actual.len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());

        let actual = path_to_code(&directional, &actual);
        assert_eq!(
            actual.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_179a() {
        let expected = "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        let actual = get_shortest_path("179A");

        assert_eq!(expected.len(), actual.len());
    }
}
