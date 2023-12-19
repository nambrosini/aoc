use std::collections::{HashMap, VecDeque};
use derive_more::Constructor;
use itertools::Itertools;
advent_of_code::solution!(19);

type Rules = HashMap<String, Vec<String>>;

type Part = HashMap<String, u64>;

fn parse_parts(s: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    for line in s.lines() {
        let chunks: Vec<&str> = line[1..line.len() - 1].split(',').collect();
        let mut part = Part::new();
        for chunk in chunks {
            let index = &chunk[0..1];
            let val = chunk[2..].parse().unwrap();
            part.insert(index.to_string(), val);
        }
        parts.push(part);
    }
    parts
}

fn parse_rules(s: &str) -> Rules {
    let mut rules = Rules::new();
    for line in s.lines() {
        let chunks: Vec<&str> = line.split('{').collect();
        let index = chunks[0].to_string();
        let chunks = chunks[1][..chunks[1].len() - 1].split(',').map(|s| s.to_string()).collect();
        rules.insert(index, chunks);
    }
    rules
}

trait Calc {
    fn calc_rating(&self) -> u64;
}

impl Calc for Part {
    fn calc_rating(&self) -> u64 {
        self.values().sum()
    }
}

struct Input {
    rules: Rules,
    parts: Vec<Part>
}

fn parse(input: &str) -> Input {
    let chunks: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(chunks[0]);
    let parts = parse_parts(chunks[1]);

    Input { rules, parts }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse(input);

    let rating: u64 = input.parts
        .iter()
        .filter(|p| validate_part(p, &input.rules))
        .map(|p| p.calc_rating())
        .sum();
    Some(rating)
}

type Beam = Vec<RangeInc>;

#[derive(Debug, Eq, PartialEq, Constructor, Copy, Clone)]
struct RangeInc {
    start: u32,
    end: u32,
}

impl RangeInc {
    fn size(&self) -> usize {
        // since this range is inclusive we need to make sure that [3, 3] counts as size 1
        (self.end + 1) as usize - self.start as usize
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Condition {
    // destination
    Always(String),
    // property, is_gt, value to compare against, destination
    Ltgt(usize, bool, u32, String),
}

// returns split, remaining
fn split_beam(beam: &Beam, condition: &Condition) -> (Option<(String, Beam)>, Option<Beam>) {
    match condition {
        Condition::Always(dest) => (Some((dest.clone(), beam.clone())), None),
        Condition::Ltgt(p, gt, v, dest) => {
            let relevant_range = beam[*p];

            let (matching, notmatching) = split_range(relevant_range, *gt, *v);

            let matching_beam = matching.map(|r| (dest.to_string(), replace_range(beam, r, *p)));
            let remaining_beam = notmatching.map(|r| replace_range(beam, r, *p));

            (matching_beam, remaining_beam)
        }
    }
}

fn replace_range(beam: &Beam, new_range: RangeInc, p: usize) -> Beam {
    let mut new_beam = beam.clone();
    new_beam[p] = new_range;
    new_beam
}

#[allow(clippy::collapsible_else_if)]
fn split_range(range: RangeInc, gt: bool, v: u32) -> (Option<RangeInc>, Option<RangeInc>) {
    if gt {
        if range.start > v {
            (Some(range), None)
        } else if range.end < v {
            (None, Some(range))
        } else {
            (
                Some(RangeInc::new(v + 1, range.end)),
                Some(RangeInc::new(range.start, v)),
            )
        }
    } else
    /* lt */
    {
        if range.end < v {
            (Some(range), None)
        } else if range.start > v {
            (None, Some(range))
        } else {
            (
                Some(RangeInc::new(range.start, v - 1)),
                Some(RangeInc::new(v, range.end)),
            )
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _items) = input.trim().split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .trim()
        .lines()
        .map(|l| {
            let (name, rest) = l.split_once('{').unwrap();
            let rest = rest.strip_suffix('}').unwrap();
            let conditions = rest
                .split(',')
                .map(|c| {
                    if let Some((cond, dest)) = c.split_once(':') {
                        let prop = match cond.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!("xmas"),
                        };
                        let gtlt = cond.chars().nth(1).unwrap() == '>';
                        let dest = dest.to_string();
                        let value: u32 = cond[2..].parse().unwrap();

                        Condition::Ltgt(prop, gtlt, value, dest)
                    } else {
                        let dest = c.to_string();
                        Condition::Always(dest)
                    }
                })
                .collect_vec();
            (name, conditions)
        })
        .collect();

    let mut accepted = vec![];
    let starting_beam: Beam = vec![
        RangeInc::new(1, 4000),
        RangeInc::new(1, 4000),
        RangeInc::new(1, 4000),
        RangeInc::new(1, 4000),
    ];

    let mut q = VecDeque::new();
    q.push_front(("in".to_string(), starting_beam));

    while let Some((flow, beam)) = q.pop_front() {
        let workflow = &workflows[&flow.as_str()];

        let mut remaining_beam = beam;
        for work in workflow {
            let (split, remaining) = split_beam(&remaining_beam, work);
            if let Some((dest, split)) = split {
                if dest == "A" {
                    accepted.push(split);
                } else if dest == "R" {
                    // forget about it
                } else {
                    // queue the work for another workflow
                    q.push_front((dest, split));
                }
            }

            if let Some(remaining) = remaining {
                remaining_beam = remaining;
                continue;
            } else {
                break;
            }
        }
    }

    // for each of the accepted beams, we count the number of possibilities
    // it contains (multiplying the ranges together gives the number of ways
    // you could pick a distinct number from each of the ranges)
    let part2 = accepted
        .iter()
        .map(|a| a.iter().map(|x| x.size()).product::<usize>())
        .sum::<usize>();
    Some(part2)
}

fn validate_part(part: &Part, rules: &Rules) -> bool {
    let mut workflow = "in";

    while workflow != "A" && workflow != "R" {
        let rule = &rules[workflow];
        for r in rule {
            if !r.contains(':') {
                workflow = r;
                break;
            }
            let chunks: Vec<&str> = r.split(':').collect();
            let rat = &chunks[0][..1];
            let op = &chunks[0][1..2];
            let val: u64 = chunks[0][2..].parse().unwrap();
            if op == ">" && part[rat] > val || op == "<" && part[rat] < val {
                workflow = chunks[1];
                break;
            }
        }
    }

    workflow == "A"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
