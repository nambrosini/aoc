use std::collections::HashSet;

fn parse(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

fn part1(input: &str) -> usize {
    collapse(&parse(input)).len()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let elements: HashSet<char> = input
        .iter()
        .copied()
        .filter(|&c| c as usize >= 'a' as usize && c as usize <= 'z' as usize)
        .collect();

    let mut min = usize::MAX;

    for c in elements {
        let input = input
            .iter()
            .filter(|&&x| x != c && x as usize != c as usize - 32)
            .copied()
            .collect::<Vec<_>>();
        let collapsed_len = collapse(&input).len();

        if collapsed_len < min {
            min = collapsed_len;
        }
    }

    min
}

fn collapse(input: &[char]) -> Vec<char> {
    let mut input = input.to_vec();
    let mut destroyed = true;

    while destroyed {
        destroyed = false;

        let mut v = vec![];
        let mut count = 0;

        while count < input.len() - 1 {
            if (input[count] as i32 - input[count + 1] as i32).abs() != 32 {
                v.push(input[count]);
            } else {
                destroyed = true;
                count += 1;
            }

            count += 1;
        }

        if count == input.len() - 1 {
            v.push(input[count]);
        }

        input = v.clone();
    }

    input
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
    }
}
