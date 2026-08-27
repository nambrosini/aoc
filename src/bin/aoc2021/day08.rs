use std::collections::HashSet;

struct Entry {
    patterns: Vec<String>,
    output: Vec<String>,
}

fn parse(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let (patterns, output) = line.split_once(" | ").unwrap();
            Entry {
                patterns: patterns.split(' ').map(|s| s.to_owned()).collect(),
                output: output.split(' ').map(|s| s.to_owned()).collect(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .flat_map(|e| &e.output)
        .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
        .count()
}

fn part2(input: &str) -> usize {
    parse(input).iter().map(decode_entry).sum()
}

fn decode_entry(entry: &Entry) -> usize {
    let sets: Vec<HashSet<char>> = entry.patterns.iter().map(|w| w.chars().collect()).collect();

    let mut digits: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    digits[1] = sets.iter().find(|s| s.len() == 2).unwrap().clone();
    digits[7] = sets.iter().find(|s| s.len() == 3).unwrap().clone();
    digits[4] = sets.iter().find(|s| s.len() == 4).unwrap().clone();
    digits[8] = sets.iter().find(|s| s.len() == 7).unwrap().clone();
    digits[3] = sets
        .iter()
        .filter(|s| s.len() == 5)
        .find(|s| s.is_superset(&digits[1]))
        .unwrap()
        .clone();
    digits[9] = sets
        .iter()
        .filter(|s| s.len() == 6)
        .find(|s| s.is_superset(&digits[3]))
        .unwrap()
        .clone();

    let bottom_left = *(&digits[8] - &digits[9]).iter().next().unwrap();

    digits[2] = sets
        .iter()
        .filter(|s| s.len() == 5)
        .find(|s| s.contains(&bottom_left))
        .unwrap()
        .clone();
    digits[5] = sets
        .iter()
        .filter(|s| s.len() == 5)
        .find(|s| **s != digits[2] && **s != digits[3])
        .unwrap()
        .clone();
    digits[6] = sets
        .iter()
        .filter(|s| s.len() == 6)
        .find(|s| **s != digits[9] && s.is_superset(&digits[5]))
        .unwrap()
        .clone();
    digits[0] = sets
        .iter()
        .filter(|s| s.len() == 6)
        .find(|s| **s != digits[9] && **s != digits[6])
        .unwrap()
        .clone();

    entry
        .output
        .iter()
        .map(|word| {
            let chars: HashSet<char> = word.chars().collect();
            digits.iter().position(|d| *d == chars).unwrap()
        })
        .fold(0, |acc, digit| acc * 10 + digit)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/08.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 61229);
    }
}
