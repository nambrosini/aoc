use std::cmp::Ordering;

fn parse(input: &str) -> (usize, usize) {
    let mut parts = input.trim().split('-').map(|x| x.parse().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

fn digits(x: usize) -> [u8; 6] {
    let mut v = [0u8; 6];
    for (i, e) in v.iter_mut().enumerate() {
        *e = (x / 10usize.pow(5 - i as u32) % 10) as u8;
    }
    v
}

fn is_non_decreasing_with_pair(x: usize) -> bool {
    let v = digits(x);
    let mut has_pair = false;

    for i in 0..v.len() - 1 {
        match v[i].cmp(&v[i + 1]) {
            Ordering::Greater => return false,
            Ordering::Equal => has_pair = true,
            Ordering::Less => {}
        }
    }

    has_pair
}

fn has_exact_pair(x: usize) -> bool {
    let v = digits(x);

    for i in 0..v.len() - 1 {
        if v[i] > v[i + 1] {
            return false;
        }
    }

    let mut i = 0;
    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j] == v[i] {
            j += 1;
        }
        if j - i == 2 {
            return true;
        }
        i = j;
    }
    false
}

fn part1(input: &str) -> usize {
    let (lo, hi) = parse(input);
    (lo..=hi)
        .filter(|&x| is_non_decreasing_with_pair(x))
        .count()
}

fn part2(input: &str) -> usize {
    let (lo, hi) = parse(input);
    (lo..=hi).filter(|&x| has_exact_pair(x)).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert!(is_non_decreasing_with_pair(111111));
        assert!(!is_non_decreasing_with_pair(223450));
        assert!(!is_non_decreasing_with_pair(123789));
    }

    #[test]
    fn test_part2() {
        assert!(has_exact_pair(112233));
        assert!(!has_exact_pair(123444));
        assert!(has_exact_pair(111122));
    }
}
