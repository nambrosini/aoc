fn parse(input: &str) -> Vec<u8> {
    let mut input = input.trim().as_bytes().to_vec();
    let mut found = false;

    for i in input.iter_mut() {
        if found {
            *i = b'a';
        }
        if *i == b'i' || *i == b'o' || *i == b'l' {
            *i += 1;
            found = true;
        }
    }

    input
}

fn increment(s: &[u8]) -> Vec<u8> {
    let mut s_bytes = s.to_vec();
    let len = s_bytes.len();

    for i in (0..len).rev() {
        if s_bytes[i] + 1 > 122 {
            s_bytes[i] = 97;
        } else {
            if s_bytes[i] + 1 == b'i' || s_bytes[i] + 1 == b'o' || s_bytes[i] + 1 == b'l' {
                s_bytes[i] += 2;
            } else {
                s_bytes[i] += 1;
            }
            break;
        }
    }

    s_bytes
}

fn next_valid_password(input: &[u8]) -> String {
    let rule_1 = |x: &[u8]| -> bool {
        for i in 0..x.len() - 2 {
            if x[i + 1] == x[i] + 1 && x[i + 2] == x[i] + 2 {
                return true;
            }
        }

        false
    };

    let rule_2 = |x: &[u8]| -> bool {
        let mut already_found = false;
        let mut i = 0;
        while i < x.len() - 1 {
            if x[i] == x[i + 1] {
                if already_found {
                    return true;
                } else {
                    i += 1;
                    already_found = true;
                }
            }
            i += 1;
        }
        false
    };

    let mut pass = increment(input);

    while !(rule_1(&pass) && rule_2(&pass)) {
        pass = increment(&pass);
    }

    String::from_utf8(pass).unwrap()
}

fn part1(input: &str) -> String {
    next_valid_password(&parse(input))
}

fn part2(input: &str) -> String {
    next_valid_password(part1(input).as_bytes())
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}
