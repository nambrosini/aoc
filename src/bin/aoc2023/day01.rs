use itertools::Itertools;

fn calibration_value(line: &str) -> u32 {
    let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    digits[0].to_digit(10).unwrap() * 10 + digits[digits.len() - 1].to_digit(10).unwrap()
}

/// Replaces spelled-out digits with their numeral, keeping one character of overlap
/// so that e.g. "twone" is read as both "two" and "one".
fn spell_out_digits(line: &str) -> String {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut out = String::new();
    let mut i = 0;

    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            out.push(c);
            i += 1;
            continue;
        }

        for len in 3..=5 {
            if i + len > line.len() {
                break;
            }

            let candidate = &line[i..i + len];
            if let Some(pos) = WORDS.iter().find_position(|w| **w == candidate) {
                out.push_str(&(pos.0 + 1).to_string());
                i += len - 2;
                break;
            }
        }

        i += 1;
    }

    out
}

fn part1(input: &str) -> u32 {
    input.lines().map(calibration_value).sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| calibration_value(&spell_out_digits(l)))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(s), 142);
    }

    #[test]
    fn test_part2() {
        let s = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(s), 281);
    }
}
