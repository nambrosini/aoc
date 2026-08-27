fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

fn part1(input: &str) -> u32 {
    let input = parse(input);
    let mut sum = 0u32;

    for i in 0..input.len() {
        if input[i] == input[(i + 1) % input.len()] {
            sum += input[i]
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let input = parse(input);
    let mut sum = 0u32;

    let halfway = input.len() / 2;

    for i in 0..input.len() {
        if input[i] == input[(i + halfway) % input.len()] {
            sum += input[i]
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/01.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 0);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
    }
}
