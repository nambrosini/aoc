use aoc_util::parse::ParseOps;
advent_of_code::solution!(9);

struct Input {
    vals: Vec<Vec<i32>>,
}

fn parse(input: &str) -> Input {
    let vals = input.lines().map(|l| l.iter_signed().collect()).collect();

    Input { vals }
}

pub fn part_one(input: &str) -> Option<i32> {
    let vals = parse(input).vals;
    let mut res = 0;
    for v in vals {
        let mut lasts = vec![v[v.len() - 1]];
        let mut list = v.clone();
        while !list.iter().all(|&x| x == 0) {
            list = calc_diffs(&list);
            lasts.push(list[list.len() - 1]);
        }
        let mut v = 0;

        for l in lasts.iter().rev() {
            v += l;
        }
        res += v;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let vals = parse(input).vals;
    let mut res = 0;
    for v in vals {
        let mut firsts = vec![v[0]];
        let mut list = v.clone();
        while !list.iter().all(|&x| x == 0) {
            list = calc_diffs(&list);
            firsts.push(list[0]);
        }
        let mut v = 0;

        for l in firsts.iter().rev() {
            v = l - v;
        }
        res += v;
    }
    Some(res)
}

fn calc_diffs(list: &[i32]) -> Vec<i32> {
    list[1..]
        .iter()
        .zip(&list[..list.len() - 1])
        .map(|(a, b)| a - b)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
