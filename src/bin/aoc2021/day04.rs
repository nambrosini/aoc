#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<usize>>,
}

impl Board {
    fn has_won(&self, drawn: &[usize]) -> bool {
        for (i, row) in self.rows.iter().enumerate() {
            let row_won = row.iter().all(|n| drawn.contains(n));
            let col_won = self.rows.iter().all(|r| drawn.contains(&r[i]));

            if row_won || col_won {
                return true;
            }
        }

        false
    }

    fn score(&self, drawn: &[usize]) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|n| !drawn.contains(n))
            .sum::<usize>()
            * drawn.last().unwrap()
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let (draws, boards) = input.split_once("\n\n").unwrap();

    let draws = draws
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let boards = boards
        .split("\n\n")
        .map(|board| Board {
            rows: board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect(),
        })
        .collect();

    (draws, boards)
}

fn part1(input: &str) -> usize {
    let (draws, boards) = parse(input);

    for i in 5..draws.len() {
        for board in &boards {
            if board.has_won(&draws[..=i]) {
                return board.score(&draws[..=i]);
            }
        }
    }

    unreachable!()
}

fn part2(input: &str) -> usize {
    let (draws, mut boards) = parse(input);
    let mut last: Option<(Vec<usize>, Board)> = None;

    for i in 5..draws.len() {
        let called = draws[..=i].to_vec();
        let (won, remaining): (Vec<Board>, Vec<Board>) =
            boards.into_iter().partition(|b| b.has_won(&called));

        if let Some(last_board) = won.into_iter().last() {
            last = Some((called, last_board));
        }

        boards = remaining;
    }

    let (called, board) = last.unwrap();
    board.score(&called)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/04.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1924);
    }
}
