use itertools::Itertools;

fn part1(input: &str) -> String {
    let inverted = generate_initial_state(input.trim(), 272);

    generate_checksum(&inverted).iter().collect()
}

fn part2(input: &str) -> String {
    let inverted = generate_initial_state(input.trim(), 35651584);

    generate_checksum(&inverted).iter().collect()
}

fn generate_initial_state(s: &str, len: usize) -> Vec<char> {
    let mut a: Vec<char> = s.chars().collect();

    while a.len() < len {
        let mut b = invert_bits(&a);
        a.push('0');
        a.append(&mut b);
    }

    a.iter().take(len).copied().collect()
}

fn invert_bits(s: &[char]) -> Vec<char> {
    let mut new_s = vec![];
    for &c in s.iter().rev() {
        if c == '0' {
            new_s.push('1');
        } else {
            new_s.push('0');
        }
    }

    new_s
}

fn generate_checksum(s: &[char]) -> Vec<char> {
    let mut s: Vec<char> = s.to_vec();

    while s.len().is_multiple_of(2) {
        let mut new_s = vec![];

        for (prev, next) in s.iter().tuples() {
            if prev == next {
                new_s.push('1');
            } else {
                new_s.push('0');
            }
        }

        s = new_s;
    }

    s
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/16.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let s: Vec<char> = "10000011110010000111".chars().collect();

        let res: String = generate_checksum(&s).iter().collect();

        assert_eq!(&res, "01100");
    }
}
