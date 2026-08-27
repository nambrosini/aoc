use md5::{Digest, Md5};
use std::collections::HashMap;

fn md5_hex(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn part1(input: &str) -> usize {
    solve(input, &gen_digest)
}

fn part2(input: &str) -> usize {
    solve(input, &gen_two_digest)
}

fn solve(input: &str, f: &dyn Fn(&str, usize) -> String) -> usize {
    let input = input.trim();
    let mut index = 0;
    let mut keys_count = 0;

    let mut map: HashMap<usize, char> = HashMap::new();

    loop {
        let digest = f(input, index);

        if index > 1000 {
            let index_to_remove: Vec<usize> = map
                .iter()
                .filter(|&(&k, _)| k < index - 1000)
                .map(|(k, _)| *k)
                .collect();

            for i in index_to_remove {
                map.remove(&i);
            }
        }

        if let Some(c) = has_consecutive(&digest, 5) {
            let mut found_keys: Vec<usize> = map
                .iter()
                .filter(|&(_, &v)| v == c)
                .map(|(k, _)| *k)
                .collect();
            found_keys.sort_unstable();

            for k in found_keys {
                keys_count += 1;

                if keys_count == 64 {
                    return k;
                }

                map.remove(&k);
            }
        }

        if let Some(c) = has_consecutive(&digest, 3) {
            map.insert(index, c);
        }

        index += 1;
    }
}

fn gen_digest(seed: &str, index: usize) -> String {
    md5_hex(&format!("{}{}", seed, index))
}

fn gen_two_digest(seed: &str, index: usize) -> String {
    let mut digest = md5_hex(&format!("{}{}", seed, index));

    for _ in 0..2016 {
        digest = md5_hex(&digest);
    }

    digest
}

fn has_consecutive(s: &str, count: usize) -> Option<char> {
    let s: Vec<char> = s.chars().collect();

    for i in 0..=s.len() - count {
        let mut equals = true;
        for j in i + 1..i + count {
            if s[i] != s[j] {
                equals = false;
                break;
            }
        }

        if equals {
            return Some(s[i]);
        }
    }

    None
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        let s = "nasdlfansdcccasdf";

        assert_eq!(has_consecutive(s, 3), Some('c'));
    }

    #[test]
    fn test_five() {
        let s = "asdfasdfhhhhhasdfasdfqwer";

        assert_eq!(has_consecutive(s, 5), Some('h'));
    }

    #[test]
    fn test_five_two() {
        let s = "asdfasdfhhhhh";

        assert_eq!(has_consecutive(s, 5), Some('h'));
    }

    // Slow (brute-force MD5 search); run explicitly with `cargo test -- --ignored`.
    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1("abc"), 22728);
    }
}
