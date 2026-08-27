#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => unreachable!(),
        }
    }
}

type Hand = Vec<(Color, i32)>;
type Game = Vec<Hand>;

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let (_, hands) = l.split_once(": ").unwrap();
            hands
                .split("; ")
                .map(|hand| {
                    hand.split(", ")
                        .map(|cube| {
                            let (count, color) = cube.split_once(' ').unwrap();
                            (color.into(), count.parse().unwrap())
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn max_cubes(game: &Game, color: Color) -> i32 {
    game.iter()
        .flatten()
        .filter(|(c, _)| *c == color)
        .map(|(_, count)| *count)
        .max()
        .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .enumerate()
        .filter(|(_, g)| {
            max_cubes(g, Color::Red) <= 12
                && max_cubes(g, Color::Green) <= 13
                && max_cubes(g, Color::Blue) <= 14
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> i32 {
    parse(input)
        .iter()
        .map(|g| max_cubes(g, Color::Red) * max_cubes(g, Color::Green) * max_cubes(g, Color::Blue))
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2023/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2286);
    }
}
