use std::sync::LazyLock;

use regex::Regex;

static PAREN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\((\d|\s|[+*])+\)").unwrap());

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn eval_op(a: i64, op: &str, b: i64) -> i64 {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => unreachable!(),
    }
}

fn eval_same_precedence(tokens: &mut Vec<String>) {
    while tokens.len() != 1 {
        let res = eval_op(
            tokens[0].parse().unwrap(),
            &tokens[1],
            tokens[2].parse().unwrap(),
        );
        let mut new = vec![res.to_string()];
        new.extend_from_slice(&tokens[3..]);
        *tokens = new;
    }
}

fn eval_plus_first(tokens: &mut Vec<String>) {
    while tokens.len() != 1 {
        if let Some(i) = (1..tokens.len() - 1).step_by(2).find(|&i| tokens[i] == "+") {
            let res = eval_op(
                tokens[i - 1].parse().unwrap(),
                "+",
                tokens[i + 1].parse().unwrap(),
            );
            let mut new = tokens[..i.saturating_sub(1)].to_vec();
            new.push(res.to_string());
            new.extend_from_slice(&tokens[i + 2..]);
            *tokens = new;
        } else {
            eval_same_precedence(tokens);
        }
    }
}

fn eval(expr: &str, same_precedence: bool) -> i64 {
    let mut expr = expr.to_owned();

    while let Some(m) = PAREN_RE.find(&expr.clone()) {
        let inner = m.as_str().trim_matches(|c| c == '(' || c == ')');
        let res = eval(inner, same_precedence);
        expr = expr.replacen(m.as_str(), &res.to_string(), 1);
    }

    let mut tokens: Vec<String> = expr.split(' ').map(|s| s.to_owned()).collect();

    if same_precedence {
        eval_same_precedence(&mut tokens);
    } else {
        eval_plus_first(&mut tokens);
    }

    tokens[0].parse().unwrap()
}

fn part1(input: &str) -> i64 {
    parse(input).iter().map(|line| eval(line, true)).sum()
}

fn part2(input: &str) -> i64 {
    parse(input).iter().map(|line| eval(line, false)).sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/18.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
