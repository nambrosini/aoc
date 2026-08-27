fn parse(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> usize {
    winner(parse(input))
}

fn part2(input: &str) -> usize {
    winner2(parse(input))
}

fn next_power_of_three(mut n: usize) -> usize {
    let mut result = 1;
    while n > 0 {
        n /= 3;
        result *= 3;
    }
    result
}

fn winner2(elves: usize) -> usize {
    let next = next_power_of_three(elves);
    let prev = next / 3;
    if elves == prev {
        elves
    } else if elves > prev * 2 {
        elves * 2 - next
    } else {
        elves - prev
    }
}

fn winner(elves: usize) -> usize {
    if elves.is_power_of_two() {
        1
    } else {
        ((elves << 1) ^ elves.next_power_of_two()) | 1
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/19.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(winner(5), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(winner2(5), 2);
    }
}
