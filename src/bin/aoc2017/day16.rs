use itertools::Itertools;

fn parse(input: &str) -> Vec<DanceMove> {
    input.trim().split(',').map(DanceMove::from).collect()
}

fn part1(input: &str) -> String {
    let input = parse(input);
    let mut programs = (b'a'..=b'p').map(|c| c as char).collect_vec();

    programs = dance(&programs, &input);

    programs.iter().join("")
}

fn part2(input: &str) -> String {
    let input = parse(input);
    let programs = (b'a'..=b'p').map(|c| c as char).collect_vec();

    let list = get_cycle(&input, &programs);

    let cycle_size = list.len();
    let index = 1_000_000_000 % cycle_size;

    list[index].iter().join("")
}

fn dance(programs: &[char], moves: &[DanceMove]) -> Vec<char> {
    let mut programs = programs.to_vec();
    let len = programs.len();

    for d in moves {
        match d {
            DanceMove::Spin(v) => {
                programs = programs[len - v..]
                    .iter()
                    .chain(programs[..len - v].iter())
                    .copied()
                    .collect_vec();
            }
            DanceMove::Exchange(a, b) => {
                programs.swap(*a, *b);
            }
            DanceMove::Partner(a, b) => {
                let i_a = programs.iter().position(|x| x == a).unwrap();
                let i_b = programs.iter().position(|x| x == b).unwrap();

                programs.swap(i_a, i_b);
            }
        }
    }

    programs
}

fn get_cycle(input: &[DanceMove], base: &[char]) -> Vec<Vec<char>> {
    let mut list: Vec<Vec<char>> = Vec::new();
    let mut next = base.to_vec();

    list.push(next.clone());

    for _ in 0.. {
        next = dance(&next, input);

        if list.contains(&next) {
            break;
        }

        list.push(next.clone());
    }

    list
}

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl From<&str> for DanceMove {
    fn from(s: &str) -> Self {
        let dance_move = s[..1].chars().next().unwrap();
        let pos = &s[1..].split('/').collect_vec();

        match dance_move {
            's' => DanceMove::Spin(pos[0].parse().unwrap()),
            'x' => DanceMove::Exchange(pos[0].parse().unwrap(), pos[1].parse().unwrap()),
            'p' => DanceMove::Partner(
                pos[0].chars().next().unwrap(),
                pos[1].chars().next().unwrap(),
            ),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/16.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dance() {
        let programs = vec!['a', 'b', 'c', 'd', 'e'];
        let moves = parse("s1,x3/4,pe/b");

        assert_eq!(dance(&programs, &moves), vec!['b', 'a', 'e', 'd', 'c']);
    }
}
