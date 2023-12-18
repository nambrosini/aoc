use std::str::FromStr;

use advent_of_code::util::position::Vec2;

advent_of_code::solution!(18);

type Instruction = (Vec2, i64);

struct Input {
    vals: Vec<(Vec2, i64, String)>,
}

fn parse(input: &str) -> Input {
    let vals = input
        .lines()
        .map(|line| {
            let chunks: Vec<&str> = line.split(' ').collect();
            let dir: Vec2 = Vec2::from_str(chunks[0]).unwrap();
            let val = chunks[1].parse().unwrap();
            let color = chunks[2][1..chunks[2].len() - 1].to_string();
            (dir, val, color)
        })
        .collect();
    Input { vals }
}

pub fn part_one(input: &str) -> Option<u64> {
    let vals = parse(input).vals;
    let vals: Vec<Instruction> = vals.iter().map(|(dir, len, _)| (*dir, *len)).collect();
    let vertices = get_vertices(&vals);

    Some(calculate_area(&vertices))
}

pub fn part_two(input: &str) -> Option<u64> {
    let vals = parse(input).vals;
    let mut instructions = vec![];
    for (_, _, hex) in vals {
        let v = i64::from_str_radix(&hex[1..6], 16).unwrap();
        let dir = match &hex[6..] {
            "0" => Vec2::EAST,
            "1" => Vec2::SOUTH,
            "2" => Vec2::WEST,
            "3" => Vec2::NORTH,
            _ => unreachable!(),
        };
        instructions.push((dir, v));
    }
    let vertices = get_vertices(&instructions);

    Some(calculate_area(&vertices))
}

fn calculate_area(polygon: &[Vec2]) -> u64 {
    (0..polygon.len())
        .map(|i| {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];

            p1.x * p2.y - p1.y * p2.x + (p1 - p2).abs()
        })
        .sum::<i64>()
        .unsigned_abs()
        / 2
        + 1
}

fn get_vertices(instructions: &[Instruction]) -> Vec<Vec2> {
    let mut vertices = Vec::new();
    let mut current = Vec2::default();

    for (dir, v) in instructions {
        current = current + (*dir * *v);
        vertices.push(current);
    }

    vertices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
