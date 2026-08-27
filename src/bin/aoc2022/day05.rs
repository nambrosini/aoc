type State = Vec<Vec<char>>;

struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split_whitespace().collect();
        Self {
            n: split[1].parse().unwrap(),
            from: split[3].parse().unwrap(),
            to: split[5].parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> (State, Vec<Instruction>) {
    let (drawing, moves) = input.split_once("\n\n").unwrap();
    let rows: Vec<Vec<char>> = drawing.lines().map(|l| l.chars().collect()).collect();

    let mut state: State = vec![vec![]; rows.last().unwrap().len() / 4 + 1];

    for row in &rows {
        let mut col = 1;
        while col < row.len() {
            if row[col].is_ascii_digit() {
                break;
            } else if row[col] != ' ' {
                state[col / 4].insert(0, row[col]);
            }
            col += 4;
        }
    }

    let instructions = moves.lines().map(Instruction::from).collect();

    (state, instructions)
}

fn part1(input: &str) -> String {
    let (mut state, instructions) = parse(input);

    for instruction in &instructions {
        for _ in 0..instruction.n {
            let c = state[instruction.from - 1].pop().unwrap();
            state[instruction.to - 1].push(c);
        }
    }

    state.iter().map(|t| *t.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let (mut state, instructions) = parse(input);

    for instruction in &instructions {
        let len = state[instruction.from - 1].len();
        let mut v: Vec<char> = state[instruction.from - 1]
            .drain(len - instruction.n..)
            .collect();
        state[instruction.to - 1].append(&mut v);
    }

    state.iter().map(|t| *t.last().unwrap()).collect()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2022/05.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), "MCD");
    }
}
