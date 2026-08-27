use adventofcode::util::knot_hash::{hex_hash, knot_round};

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let lengths = parse(input);

    let mut list: Vec<u16> = (0..256).collect();
    let mut position = 0;
    let mut skip = 0;

    knot_round(&mut list, &lengths, &mut position, &mut skip);

    list[0] as usize * list[1] as usize
}

fn part2(input: &str) -> String {
    hex_hash(input)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/10.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lengths = parse("3,4,1,5");

        let mut list: Vec<u16> = (0..5).collect();
        let mut position = 0;
        let mut skip = 0;
        knot_round(&mut list, &lengths, &mut position, &mut skip);

        assert_eq!(list[0] as usize * list[1] as usize, 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }
}
