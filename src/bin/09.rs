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
    for v in &vals {
        let mut lasts = vec![v[v.len() - 1]];
        let mut list = v.clone();

        while !list.iter().all(|&x| x == 0) {
            list = calc_diffs(&list);
            lasts.push(list[list.len() - 1]);
        }

        res += lasts.iter().sum::<i32>()
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let vals = parse(input).vals;
    let mut res = 0;

    for v in &vals {
        let mut firsts = vec![v[0]];
        let mut list = v.clone();

        while !list.iter().all(|&x| x == 0) {
            list = calc_diffs(&list);
            firsts.push(list[0]);
        }

        res += firsts.iter().rev()
            .fold(0, |acc, x| x - acc);
    }
    Some(res)
}

fn calc_diffs(list: &[i32]) -> Vec<i32> {
    list.windows(2).map(|w| w[1] - w[0]).collect()
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
