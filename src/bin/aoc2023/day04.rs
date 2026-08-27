/// Number of winning numbers matched, per card.
fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let (_, numbers) = l.split_once(": ").unwrap();
            let (winning, have) = numbers.split_once(" | ").unwrap();

            let winning: Vec<u32> = winning
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            have.split_whitespace()
                .filter(|x| winning.contains(&x.parse().unwrap()))
                .count()
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .map(|&matches| {
            if matches == 0 {
                0
            } else {
                2u32.pow(matches as u32 - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = parse(input);
    let mut copies = vec![1u32; cards.len()];

    for (i, &matches) in cards.iter().enumerate() {
        for k in i + 1..=i + matches {
            if k < copies.len() {
                copies[k] += copies[i];
            }
        }
    }

    copies.iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 30);
    }
}
