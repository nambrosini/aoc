use itertools::Itertools;
use md5::{Digest, Md5};

fn md5_hex(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn part1(input: &str) -> String {
    let input = input.trim();
    let mut s = String::new();

    for i in 0.. {
        let digest = md5_hex(&format!("{}{}", input, i));
        if &digest[..5] == "00000" {
            s.push_str(&digest[5..6]);

            if s.len() == 8 {
                return s;
            }
        }
    }

    unreachable!();
}

fn part2(input: &str) -> String {
    let input = input.trim();
    const VAL: String = String::new();
    let mut s: [String; 8] = [VAL; 8];

    for i in 0.. {
        let digest = md5_hex(&format!("{}{}", input, i));
        if &digest[..5] == "00000" {
            let index: usize = if let Ok(v) = digest[5..6].parse::<usize>() {
                if v < s.len() {
                    v
                } else {
                    continue;
                }
            } else {
                continue;
            };

            if s[index] == String::new() {
                s[index] = digest[6..7].to_string();
            }

            if !s.iter().any(|c| c.is_empty()) {
                return s.iter().join("");
            }
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Slow (brute-force MD5 search); run explicitly with `cargo test -- --ignored`.
    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(&part1("abc"), "18f47a30");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(&part2("abc"), "05ace8e3");
    }
}
