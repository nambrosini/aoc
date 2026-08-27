enum Instruction {
    DealIntoNewStack,
    Cut(i64),
    DealWithIncrement(i64),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| {
            if line == "deal into new stack" {
                Instruction::DealIntoNewStack
            } else if let Some(n) = line.strip_prefix("deal with increment ") {
                Instruction::DealWithIncrement(n.parse().unwrap())
            } else {
                Instruction::Cut(line.strip_prefix("cut ").unwrap().parse().unwrap())
            }
        })
        .collect()
}

fn shuffle(instructions: &[Instruction], deck_size: i64) -> Vec<i64> {
    let mut deck: Vec<i64> = (0..deck_size).collect();

    for instr in instructions {
        deck = match instr {
            Instruction::DealIntoNewStack => deck.into_iter().rev().collect(),
            Instruction::Cut(n) => {
                let n = n.rem_euclid(deck_size) as usize;
                deck[n..].iter().chain(&deck[..n]).copied().collect()
            }
            Instruction::DealWithIncrement(n) => {
                let mut new_deck = vec![0; deck.len()];
                let mut pos = 0usize;
                for card in deck {
                    new_deck[pos] = card;
                    pos = (pos + *n as usize) % deck_size as usize;
                }
                new_deck
            }
        };
    }

    deck
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);
    let deck = shuffle(&instructions, 10007);
    deck.iter().position(|&c| c == 2019).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/22.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        let deck = shuffle(&parse("deal into new stack"), 10);
        assert_eq!(deck, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut() {
        let deck = shuffle(&parse("cut 3"), 10);
        assert_eq!(deck, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let deck = shuffle(&parse("cut -4"), 10);
        assert_eq!(deck, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let deck = shuffle(&parse("deal with increment 3"), 10);
        assert_eq!(deck, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
