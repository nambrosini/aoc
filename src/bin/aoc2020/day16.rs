use std::collections::HashMap;

type Fields = HashMap<String, Vec<(usize, usize)>>;

fn parse(input: &str) -> (Fields, Vec<usize>, Vec<Vec<usize>>) {
    let split: Vec<&str> = input.split("\n\n").collect();

    let fields: Fields = split[0]
        .lines()
        .map(|l| {
            let (name, ranges) = l.split_once(": ").unwrap();
            let ranges = ranges
                .split(" or ")
                .map(|r| {
                    let (a, b) = r.split_once('-').unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                })
                .collect();
            (name.to_owned(), ranges)
        })
        .collect();

    let my_ticket: Vec<usize> = split[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let nearby: Vec<Vec<usize>> = split[2]
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (fields, my_ticket, nearby)
}

fn matches_any_field(fields: &Fields, value: usize) -> bool {
    fields
        .values()
        .any(|ranges| ranges.iter().any(|&(a, b)| value >= a && value <= b))
}

fn part1(input: &str) -> usize {
    let (fields, _, nearby) = parse(input);

    nearby
        .iter()
        .flatten()
        .filter(|&&v| !matches_any_field(&fields, v))
        .sum()
}

fn part2(input: &str) -> usize {
    let (fields, my_ticket, nearby) = parse(input);

    let valid_tickets: Vec<&Vec<usize>> = nearby
        .iter()
        .filter(|t| t.iter().all(|&v| matches_any_field(&fields, v)))
        .collect();

    let num_fields = my_ticket.len();
    let mut candidates: Vec<Vec<&String>> = (0..num_fields)
        .map(|i| {
            fields
                .iter()
                .filter(|(_, ranges)| {
                    valid_tickets
                        .iter()
                        .all(|t| ranges.iter().any(|&(a, b)| t[i] >= a && t[i] <= b))
                })
                .map(|(name, _)| name)
                .collect()
        })
        .collect();

    let mut assigned: HashMap<usize, &String> = HashMap::new();
    while assigned.len() < num_fields {
        let (i, name) = candidates
            .iter()
            .enumerate()
            .find_map(|(i, c)| if c.len() == 1 { Some((i, c[0])) } else { None })
            .unwrap();

        assigned.insert(i, name);
        for c in candidates.iter_mut() {
            c.retain(|&n| n != name);
        }
    }

    assigned
        .iter()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(&i, _)| my_ticket[i])
        .product()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/16.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part1(s), 71);
    }
}
