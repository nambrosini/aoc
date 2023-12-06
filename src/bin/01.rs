advent_of_code::solution!(1);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| calculate(l.to_string())).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    Some(
        input
            .lines()
            .map(|l| numbers_to_digits(numbers, l))
            .map(calculate)
            .sum(),
    )
}

fn calculate(l: String) -> u32 {
    let it: Vec<char> = l.chars().filter(|c| c.is_ascii_digit()).collect();
    it[0].to_digit(10).unwrap() * 10 + it[it.len() - 1].to_digit(10).unwrap()
}

fn numbers_to_digits(numbers: [&str; 9], l: &str) -> String {
    let mut string = String::new();
    let mut i = 0;
    while i < l.len() {
        let c = l.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            string.push(c);
            i += 1;
            continue;
        }
        for x in 3..=5 {
            if i + x > l.len() {
                break;
            }

            let num = &l[i..i + x];
            if numbers.contains(&num) {
                let n = numbers.iter().find_position(|x| x == &&num).unwrap().0 + 1;
                string.push_str(&format!("{}", n));
                i += x - 2;
                break;
            }
        }
        i += 1;
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
