use itertools::Itertools;

fn look_and_say(input: &str, rounds: usize) -> usize {
    let mut s = input.trim().to_owned();
    for _ in 0..rounds {
        let v = group_runs(&s);
        s = String::new();
        for i in v {
            s.push_str(&i.len().to_string());
            s.push_str(&i[0..1]);
        }
    }

    s.len()
}

fn group_runs(s: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut i = 0;
    let chars = s.chars().collect_vec();

    while i < s.len() {
        let c = chars[i];
        let mut j = i + 1;
        while j < s.len() {
            if chars[j] != c {
                break;
            }
            j += 1;
        }

        res.push(chars[i..j].iter().collect());
        i = j;
    }

    res
}

fn part1(input: &str) -> usize {
    look_and_say(input, 40)
}

fn part2(input: &str) -> usize {
    look_and_say(input, 50)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
