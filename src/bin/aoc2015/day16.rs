use itertools::Itertools;
use std::collections::HashMap;

type MapType = HashMap<usize, HashMap<String, usize>>;

fn parse(input: &str) -> MapType {
    let mut map: MapType = MapType::new();

    for l in input.lines() {
        let l = l.replace([':', ','], "");
        let split = l.split(' ').collect_vec();
        let id = split[1].parse().unwrap();

        let mut aunt = HashMap::new();

        for i in (2..split.len()).step_by(2) {
            aunt.insert(split[i].to_owned(), split[i + 1].parse().unwrap());
        }

        map.insert(id, aunt);
    }

    map
}

fn known_aunt() -> HashMap<String, usize> {
    HashMap::from([
        ("children".to_owned(), 3),
        ("cats".to_owned(), 7),
        ("samoyeds".to_owned(), 2),
        ("pomeranians".to_owned(), 3),
        ("akitas".to_owned(), 0),
        ("vizslas".to_owned(), 0),
        ("goldfish".to_owned(), 5),
        ("trees".to_owned(), 3),
        ("cars".to_owned(), 2),
        ("perfumes".to_owned(), 2),
    ])
}

fn part1(input: &str) -> usize {
    let aunts = parse(input);
    let aunt = known_aunt();

    let mut max = 0;
    let mut aunt_id = 0;

    for (id, fields) in &aunts {
        let count = fields.iter().filter(|(k, v)| aunt[*k] == **v).count();

        if count > max {
            aunt_id = *id;
            max = count;
        }
    }

    aunt_id
}

fn part2(input: &str) -> usize {
    let aunts = parse(input);
    let aunt = known_aunt();

    let mut max = 0;
    let mut aunt_id = 0;

    for (id, fields) in &aunts {
        let mut count = 0;

        for (k, v) in fields {
            let check = match k.as_str() {
                "cats" | "trees" => aunt[k] < *v,
                "pomeranians" | "goldfish" => aunt[k] > *v,
                _ => aunt[k] == *v,
            };

            if check {
                count += 1;
            }
        }

        if count > max {
            aunt_id = *id;
            max = count;
        }
    }

    aunt_id
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/16.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
