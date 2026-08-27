use std::collections::HashMap;

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn apply_value_mask(mask: &str, value: u64) -> u64 {
    let mut result = value;
    for (i, c) in mask.chars().rev().enumerate() {
        match c {
            '1' => result |= 1 << i,
            '0' => result &= !(1 << i),
            _ => {}
        }
    }
    result
}

fn part1(input: &str) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask = "";

    for line in parse(input) {
        let (key, value) = line.split_once(" = ").unwrap();

        if key == "mask" {
            mask = value;
        } else {
            let address: usize = key[4..key.len() - 1].parse().unwrap();
            let value: u64 = value.parse().unwrap();
            mem.insert(address, apply_value_mask(mask, value));
        }
    }

    mem.values().sum()
}

fn floating_addresses(mask: &str, address: u64) -> Vec<u64> {
    let masked: Vec<char> = mask
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| match c {
            '0' => {
                if (address >> i) & 1 == 1 {
                    '1'
                } else {
                    '0'
                }
            }
            c => c,
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let floating: Vec<usize> = masked
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'X')
        .map(|(i, _)| i)
        .collect();

    (0..1u64 << floating.len())
        .map(|combo| {
            let mut bits = masked.clone();
            for (j, &pos) in floating.iter().enumerate() {
                bits[pos] = if (combo >> j) & 1 == 1 { '1' } else { '0' };
            }
            u64::from_str_radix(&bits.iter().collect::<String>(), 2).unwrap()
        })
        .collect()
}

fn part2(input: &str) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";

    for line in parse(input) {
        let (key, value) = line.split_once(" = ").unwrap();

        if key == "mask" {
            mask = value;
        } else {
            let address: u64 = key[4..key.len() - 1].parse().unwrap();
            let value: u64 = value.parse().unwrap();
            for addr in floating_addresses(mask, address) {
                mem.insert(addr, value);
            }
        }
    }

    mem.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(part1(s), 165);
    }

    #[test]
    fn test_part2() {
        let s = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(part2(s), 208);
    }
}
