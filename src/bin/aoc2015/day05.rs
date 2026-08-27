fn part1(input: &str) -> usize {
    let check_rule_1 = |s: &str| -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        s.chars().filter(|x| vowels.contains(x)).count() >= 3
    };

    let check_rule_2 = |s: &str| -> bool {
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                return true;
            }
        }

        false
    };

    let check_rule_3 = |s: &str| -> bool {
        let not = ["ab", "cd", "pq", "xy"];

        not.iter().filter(|x| s.contains(x.to_owned())).count() == 0
    };

    input
        .lines()
        .filter(|x| check_rule_1(x) && check_rule_2(x) && check_rule_3(x))
        .count()
}

fn part2(input: &str) -> usize {
    let check_rule_1 = |s: &str| -> bool {
        for i in 0..s.len() - 2 {
            let pair = &s[i..=i + 1];
            if s[..=i + 2].contains(pair) && s[i + 2..].contains(pair) {
                return true;
            }
        }

        false
    };

    let check_rule_2 = |s: &str| -> bool {
        for i in 0..s.len() - 2 {
            if s[i..=i] == s[i + 2..=i + 2] {
                return true;
            }
        }

        false
    };

    input
        .lines()
        .filter(|x| check_rule_1(x.trim()) && check_rule_2(x.trim()))
        .count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }
}
