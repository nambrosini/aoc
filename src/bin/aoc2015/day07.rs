use itertools::Itertools as _;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|x| x.split(" -> ").collect::<Vec<&str>>())
        .map(|l| (l[0].to_owned(), l[1].to_owned()))
        .collect()
}

fn part1(input: &str) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut input = parse(input);

    while !input.is_empty() {
        let mut new_input: Vec<(String, String)> = vec![];

        for next in &input {
            let key = next.1.clone();
            let value = next.0.split_whitespace().collect_vec();
            match_length(&mut map, &mut new_input, next, key, value);
        }

        input = new_input;
    }

    *map.get("a").unwrap()
}

fn part2(input: &str) -> usize {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut input = parse(input);

    while !input.is_empty() {
        let mut new_input: Vec<(String, String)> = vec![];

        for next in &input {
            let key = next.1.clone();
            let value = if key == "b" {
                vec!["46065"]
            } else {
                next.0.split_whitespace().collect_vec()
            };

            match_length(&mut map, &mut new_input, next, key, value);
        }

        input = new_input;
    }

    *map.get("a").unwrap()
}

fn match_length(
    map: &mut HashMap<String, usize>,
    new_input: &mut Vec<(String, String)>,
    next: &(String, String),
    key: String,
    value: Vec<&str>,
) {
    match value.len() {
        1 => {
            if let Ok(v) = value[0].parse() {
                map.insert(key, v);
            } else {
                let a = map.get(value[0]).cloned();
                if let Some(a) = a {
                    map.insert(key, a);
                } else {
                    new_input.push(next.clone());
                }
            }
        }
        2 => {
            let a = map.get(value[1]).cloned();
            if let Some(a) = a {
                map.insert(key, !a);
            } else {
                new_input.push(next.clone());
            }
        }
        3 => {
            let a = value[0].parse::<usize>();
            let b = match value[2].parse::<usize>() {
                Ok(b) => Some(b),
                Err(_) => map.get(value[2]).cloned(),
            };

            if let Some(b) = b {
                if let Ok(a) = a {
                    map.insert(key, calc_val(a, b, value[1]));
                } else if let Some(a) = map.get(value[0]).cloned() {
                    map.insert(key, calc_val(a, b, value[1]));
                } else {
                    new_input.push(next.clone());
                }
            } else {
                new_input.push(next.clone());
            }
        }
        _ => unreachable!(),
    }
}

fn calc_val(a: usize, b: usize, ops: &str) -> usize {
    match ops {
        "AND" => a & b,
        "OR" => a | b,
        "LSHIFT" => a << b,
        "RSHIFT" => a >> b,
        _ => unreachable!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
