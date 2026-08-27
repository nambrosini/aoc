use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|l| {
            l.replace(['[', ']'], " ")
                .split(' ')
                .map(|x| x.to_string())
                .collect_vec()
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .map(|p| {
            (
                p.iter().step_by(2).join(" "),
                p.iter().skip(1).step_by(2).join(" "),
            )
        })
        .filter(|(x, y)| abba(x) && !abba(y))
        .count()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .map(|p| {
            (
                p.iter().step_by(2).join(" "),
                p.iter().skip(1).step_by(2).join(" "),
            )
        })
        .filter(|(x, y)| {
            x.chars()
                .zip(x.chars().skip(1))
                .zip(x.chars().skip(2))
                .map(|((c0, c1), c2)| (c0, c1, c2))
                .any(|(c0, c1, c2)| {
                    c0 == c2 && c0 != c1 && y.contains(&format!("{}{}{}", c1, c0, c1))
                })
        })
        .count()
}

fn abba(s: &str) -> bool {
    let zips = s
        .chars()
        .zip(s.chars().skip(1))
        .zip(s.chars().skip(2))
        .zip(s.chars().skip(3))
        .map(|(((c0, c1), c2), c3)| (c0, c1, c2, c3))
        .collect_vec();

    zips.iter()
        .any(|(c0, c1, c2, c3)| c0 == c3 && c1 == c2 && c0 != c1)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/07.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "abbaasdf[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";

        assert_eq!(part1(s), 2);
    }
}
