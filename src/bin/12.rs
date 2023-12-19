use aoc_util::parse::ParseOps;

advent_of_code::solution!(12);

struct Input {
    condition: Vec<(String, Vec<usize>)>,
}

fn parse(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let condition = lines
        .iter()
        .map(|l| {
            let chunks: Vec<&str> = l.split(' ').collect();
            let values = chunks[1].iter_unsigned().collect();
            (chunks[0].to_string(), values)
        })
        .collect();

    Input { condition }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    Some(
        input
            .condition
            .iter()
            .map(|(c, r)| count_arrangements(c, r))
            .sum(),
    )
}

fn count_arrangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = vec![vec![vec![0; n + 1]; m + 1]; n + 1];

    dp[n][m][0] = 1;
    dp[n][m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for (group, &max_count) in counts.iter().enumerate() {
            for count in 0..=max_count {
                for &c in &[b'.', b'#'] {
                    if line[pos] == c || line[pos] == b'?' {
                        if c == b'.' && count == 0 {
                            dp[pos][group][count] += dp[pos + 1][group][0];
                        } else if c == b'.' && group < m && counts[group] == count {
                            dp[pos][group][count] += dp[pos + 1][group + 1][0];
                        } else if c == b'#' {
                            dp[pos][group][count] += dp[pos + 1][group][count + 1];
                        }
                    }
                }
            }
        }
        if matches!(line[pos], b'.' | b'?') {
            dp[pos][m][0] += dp[pos + 1][m][0];
        }
    }

    dp[0][0][0]
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);

    let count = input
        .condition
        .iter()
        .map(|(pattern, counts)| {
            let pattern = [pattern.as_str(); 5].join("?");
            let counts = counts.repeat(5);
            count_arrangements(&pattern, &counts)
        })
        .sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
    //
    // #[test]
    // fn test_perm() {
    //     let rest = permutations(&("?###????????".chars().collect(), "3,2,1".iter_unsigned().collect()));
    //     assert_eq!(rest, 10);
    // }
}
