use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<Message> {
    let mut input = input.lines().collect_vec();
    input.sort_unstable();

    input.iter().copied().map(Message::from).collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let guards = parse_guards(&input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().sum::<usize>())
        .unwrap();

    minutes_asleep.0
        * minutes_asleep
            .1
            .iter()
            .enumerate()
            .max_by_key(|x| x.1)
            .unwrap()
            .0
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let guards = parse_guards(&input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().max().unwrap())
        .unwrap();

    minutes_asleep.0
        * minutes_asleep
            .1
            .iter()
            .enumerate()
            .max_by_key(|x| x.1)
            .unwrap()
            .0
}

fn parse_guards(input: &[Message]) -> HashMap<usize, Guard> {
    let input = input.iter();
    let mut guards: HashMap<usize, Guard> = HashMap::new();

    let mut guard: Option<Guard> = None;

    for next in input {
        match next.action {
            Action::Begins => {
                let id = next.id.unwrap();
                if let Some(g) = guard.take() {
                    guards.insert(g.id, g);
                }

                guard = Some(guards.entry(id).or_insert_with(|| Guard::new(id)).clone());
            }
            _ => {
                if let Some(g) = guard.take() {
                    let mut g = g.clone();
                    g.update(next);
                    guard = Some(g);
                }
            }
        }
    }

    if let Some(g) = guard.take() {
        guards.insert(g.id, g);
    }

    guards
}

#[derive(Debug, Clone)]
struct Guard {
    id: usize,
    map: HashMap<(usize, usize), [bool; 60]>,
}

impl Guard {
    fn new(id: usize) -> Self {
        Self {
            id,
            map: HashMap::new(),
        }
    }

    fn update(&mut self, message: &Message) {
        let key = (message.day, message.month);
        match message.action {
            Action::Asleep => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in e.iter_mut().take(60).skip(message.minute) {
                    *m = true;
                }
            }
            Action::Wakes => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in e.iter_mut().take(60).skip(message.minute) {
                    *m = false;
                }
            }
            _ => {}
        }
    }

    fn get_freq_min(&self) -> Vec<usize> {
        let mut v: [usize; 60] = [0; 60];

        for d in self.map.values() {
            for i in 0..d.len() {
                v[i] += d[i] as usize;
            }
        }

        v.to_vec()
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    Begins,
    Asleep,
    Wakes,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        if s.contains("begins") {
            Action::Begins
        } else if s.contains("wakes") {
            Action::Wakes
        } else if s.contains("asleep") {
            Action::Asleep
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Message {
    month: usize,
    day: usize,
    minute: usize,
    action: Action,
    id: Option<usize>,
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        let re =
            Regex::new(r"\[1518-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard #(\d+).+|.+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        let id = cap.get(6).map(|x| x.as_str().parse::<usize>().unwrap());

        Self {
            month: cap[1].parse().unwrap(),
            day: cap[2].parse().unwrap(),
            minute: cap[4].parse().unwrap(),
            action: cap[5].into(),
            id,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4455);
    }
}
