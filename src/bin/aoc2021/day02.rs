enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        let (dir, v) = s.split_once(' ').unwrap();
        let v: i32 = v.parse().unwrap();

        match dir {
            "forward" => Direction::Forward(v),
            "up" => Direction::Up(v),
            "down" => Direction::Down(v),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input.lines().map(Direction::from).collect()
}

fn part1(input: &str) -> i32 {
    let (mut h, mut v) = (0, 0);

    for d in parse(input) {
        match d {
            Direction::Forward(k) => h += k,
            Direction::Up(k) => v -= k,
            Direction::Down(k) => v += k,
        }
    }

    h * v
}

fn part2(input: &str) -> i32 {
    let (mut aim, mut h, mut v) = (0, 0, 0);

    for d in parse(input) {
        match d {
            Direction::Forward(k) => {
                h += k;
                v += aim * k;
            }
            Direction::Up(k) => aim -= k,
            Direction::Down(k) => aim += k,
        }
    }

    h * v
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/02.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 900);
    }
}
