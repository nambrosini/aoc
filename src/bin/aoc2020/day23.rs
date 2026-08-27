struct Game {
    next: Vec<usize>,
    current: usize,
}

impl Game {
    fn new(cups: &[usize]) -> Self {
        let mut next = vec![0; cups.len() + 1];
        for w in cups.windows(2) {
            next[w[0]] = w[1];
        }
        next[cups[cups.len() - 1]] = cups[0];

        Self {
            next,
            current: cups[0],
        }
    }

    fn play_round(&mut self) {
        let picked = [
            self.next[self.current],
            self.next[self.next[self.current]],
            self.next[self.next[self.next[self.current]]],
        ];

        let mut dest = if self.current == 1 {
            self.next.len() - 1
        } else {
            self.current - 1
        };
        while picked.contains(&dest) {
            dest = if dest == 1 {
                self.next.len() - 1
            } else {
                dest - 1
            };
        }

        self.next[self.current] = self.next[picked[2]];
        self.next[picked[2]] = self.next[dest];
        self.next[dest] = picked[0];

        self.current = self.next[self.current];
    }

    fn play(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.play_round();
        }
    }

    fn representation(&self) -> String {
        let mut s = String::new();
        let mut i = self.next[1];
        while i != 1 {
            s.push_str(&i.to_string());
            i = self.next[i];
        }
        s
    }

    fn score(&self) -> usize {
        self.next[1] * self.next[self.next[1]]
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn part1(input: &str) -> String {
    let mut game = Game::new(&parse(input));
    game.play(100);
    game.representation()
}

fn part2(input: &str) -> usize {
    let mut cups = parse(input);
    let max = *cups.iter().max().unwrap();
    cups.extend(max + 1..=1_000_000);

    let mut game = Game::new(&cups);
    game.play(10_000_000);
    game.score()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/23.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("389125467"), "67384529");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("389125467"), 149245887792);
    }
}
