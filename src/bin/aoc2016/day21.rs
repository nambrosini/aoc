use itertools::Itertools;

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect_vec()
}

fn part1(input: &str) -> String {
    let input = parse(input);
    solve("abcdefgh", &input)
}

fn part2(input: &str) -> String {
    let input = parse(input);
    let permutations = "abcdefgh"
        .chars()
        .permutations(8)
        .map(|p| p.iter().collect::<String>())
        .collect_vec();
    let goal = "fbgdceah";

    for p in permutations.iter() {
        let res = solve(p, &input);

        if res == goal {
            return p.to_string();
        }
    }

    unreachable!()
}

fn solve(input: &str, instructions: &[Instruction]) -> String {
    let mut chars = input.chars().collect_vec();

    for ins in instructions {
        match ins {
            Instruction::Swap(p1, p2) => chars = swap(&chars, p1, p2),
            Instruction::Rotate(dir, p) => chars = rotate(&chars, dir, *p),
            Instruction::RotateBased(c) => chars = rotate_based(&chars, *c, Direction::Right),
            Instruction::Reverse(p1, p2) => chars = reverse(&chars, *p1, *p2),
            Instruction::Move(p1, p2) => chars = move_letter(&chars, *p1, *p2),
        }
    }

    chars.iter().collect()
}

enum Instruction {
    Swap(Param, Param),
    Rotate(Direction, usize),
    RotateBased(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

enum Param {
    Position(usize),
    Letter(char),
}

enum Direction {
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let split = s.split(' ').collect_vec();

        match split[0] {
            "swap" => {
                if split[1] == "position" {
                    let position1 = split[2].parse::<usize>().unwrap();
                    let position2 = split[5].parse::<usize>().unwrap();
                    Self::Swap(Param::Position(position1), Param::Position(position2))
                } else {
                    let letter1 = split[2].chars().next().unwrap();
                    let letter2 = split[5].chars().next().unwrap();
                    Self::Swap(Param::Letter(letter1), Param::Letter(letter2))
                }
            }
            "rotate" => {
                if split[1] == "based" {
                    Self::RotateBased(split[6].chars().next().unwrap())
                } else {
                    Self::Rotate(split[1].into(), split[2].parse().unwrap())
                }
            }
            "reverse" => Self::Reverse(split[2].parse().unwrap(), split[4].parse().unwrap()),
            "move" => Self::Move(split[2].parse().unwrap(), split[5].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn swap(chars: &[char], a: &Param, b: &Param) -> Vec<char> {
    let mut chars = chars.to_vec();
    match (a, b) {
        (Param::Position(p1), Param::Position(p2)) => {
            chars.swap(*p1, *p2);
        }
        (Param::Letter(l1), Param::Letter(l2)) => {
            let p1 = chars.iter().position(|c| c == l1).unwrap();
            let p2 = chars.iter().position(|c| c == l2).unwrap();
            chars.swap(p1, p2);
        }
        _ => unreachable!(),
    }

    chars
}

fn rotate(chars: &[char], direction: &Direction, steps: usize) -> Vec<char> {
    let steps = steps % chars.len();
    let mut new_chars = vec![];
    match direction {
        Direction::Left => {
            new_chars.extend(&chars[steps..]);
            new_chars.extend(&chars[0..steps]);
        }
        Direction::Right => {
            new_chars.extend(&chars[chars.len() - steps..]);
            new_chars.extend(&chars[..chars.len() - steps]);
        }
    }

    new_chars
}

fn rotate_based(chars: &[char], c: char, dir: Direction) -> Vec<char> {
    let p = chars.iter().position(|&x| x == c).unwrap();
    let count = 1 + p + usize::from(p >= 4);
    rotate(chars, &dir, count)
}

fn reverse(chars: &[char], x: usize, y: usize) -> Vec<char> {
    let reversed = chars[x..=y].to_vec().iter().rev().copied().collect_vec();

    let mut new_vec = Vec::new();
    new_vec.extend(&chars[..x]);
    new_vec.extend(reversed);
    new_vec.extend(&chars[y + 1..]);
    new_vec
}

fn move_letter(chars: &[char], x: usize, y: usize) -> Vec<char> {
    let mut chars = chars.to_vec();
    let removed = chars.remove(x);
    chars.insert(y, removed);
    chars
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/21.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_position() {
        let mut s = "abcde".chars().collect_vec();
        s = swap(&s, &Param::Position(4), &Param::Position(0));
        assert_eq!(s.iter().collect::<String>(), String::from("ebcda"));
    }

    #[test]
    fn test_swap_letter() {
        let mut s = "ebcda".chars().collect_vec();
        s = swap(&s, &Param::Letter('d'), &Param::Letter('b'));
        assert_eq!(s.iter().collect::<String>(), String::from("edcba"));
    }

    #[test]
    fn test_reverse() {
        let mut s = "edcba".chars().collect_vec();
        s = reverse(&s, 0, 4);
        assert_eq!(s.iter().collect::<String>(), String::from("abcde"));
    }

    #[test]
    fn test_rotate() {
        let mut s = "abcde".chars().collect_vec();
        s = rotate(&s, &Direction::Left, 1);
        assert_eq!(s.iter().collect::<String>(), String::from("bcdea"));
    }

    #[test]
    fn test_rotate_right() {
        let mut s = "abcde".chars().collect_vec();
        s = rotate(&s, &Direction::Right, 1);
        assert_eq!(s.iter().collect::<String>(), String::from("eabcd"));
    }

    #[test]
    fn test_move_position() {
        let mut s = "bcdea".chars().collect_vec();
        s = move_letter(&s, 1, 4);
        assert_eq!(s.iter().collect::<String>(), String::from("bdeac"));
    }

    #[test]
    fn test_move_position2() {
        let mut s = "bdeac".chars().collect_vec();
        s = move_letter(&s, 3, 0);
        assert_eq!(s.iter().collect::<String>(), String::from("abdec"));
    }

    #[test]
    fn test_rotate_based() {
        let mut s = "abdec".chars().collect_vec();
        s = rotate_based(&s, 'b', Direction::Right);
        assert_eq!(s.iter().collect::<String>(), String::from("ecabd"));
    }

    #[test]
    fn test_rotate_based2() {
        let mut s = "ecabd".chars().collect_vec();
        s = rotate_based(&s, 'd', Direction::Right);
        assert_eq!(s.iter().collect::<String>(), String::from("decab"));
    }
}
