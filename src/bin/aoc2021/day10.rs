fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn other(ch: char) -> char {
    match ch {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => unreachable!(),
    }
}

enum LineResult {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn check_line(line: &[char]) -> LineResult {
    let mut stack = vec![];

    for &ch in line {
        if matches!(ch, ')' | ']' | '}' | '>') {
            match stack.pop() {
                Some(c) if other(ch) == c => {}
                _ => return LineResult::Corrupted(ch),
            }
        } else {
            stack.push(ch);
        }
    }

    LineResult::Incomplete(stack)
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(|line| match check_line(line) {
            LineResult::Corrupted(ch) => Some(match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            }),
            LineResult::Incomplete(_) => None,
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = parse(input)
        .iter()
        .filter_map(|line| match check_line(line) {
            LineResult::Incomplete(stack) => Some(stack.iter().rev().fold(0, |score, &c| {
                score * 5
                    + match other(c) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })),
            LineResult::Corrupted(_) => None,
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]><";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 288957);
    }
}
