use md5::{Digest, Md5};

fn part1(input: &str) -> usize {
    let key = input.trim().as_bytes();
    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.update(key);
        hasher.update(i.to_string().as_bytes());

        let output = hasher.finalize();

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            return i;
        }
    }

    unreachable!();
}

fn part2(input: &str) -> usize {
    let key = input.trim().as_bytes();
    for i in 0.. {
        let mut hasher = Md5::new();
        hasher.update(key);
        hasher.update(i.to_string().as_bytes());

        let output = hasher.finalize();

        let first_five = output[0] as i32 + output[1] as i32 + output[2] as i32;

        if first_five == 0 {
            return i;
        }
    }

    unreachable!();
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdef"), 609043);
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
