const MODULUS: u64 = 20_201_227;

fn parse(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines.next().unwrap().parse().unwrap(),
    )
}

fn find_loop_size(public_key: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;

    while value != public_key {
        value = (value * 7) % MODULUS;
        loop_size += 1;
    }

    loop_size
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    (0..loop_size).fold(1, |value, _| (value * subject) % MODULUS)
}

fn part1(input: &str) -> u64 {
    let (door_public, key_public) = parse(input);

    let key_loop_size = find_loop_size(key_public);

    transform(door_public, key_loop_size)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/25.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("17807724\n5764801"), 14897079);
    }
}
