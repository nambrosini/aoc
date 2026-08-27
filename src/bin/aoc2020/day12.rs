enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl From<&str> for Action {
    fn from(v: &str) -> Self {
        let val: i32 = v[1..].parse().unwrap();

        match &v[0..1] {
            "N" => Action::N(val),
            "S" => Action::S(val),
            "E" => Action::E(val),
            "W" => Action::W(val),
            "L" => Action::L(val),
            "R" => Action::R(val),
            "F" => Action::F(val),
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<Action> {
    input.lines().map(Action::from).collect()
}

fn rotate((x, y): (i32, i32), degrees: i32) -> (i32, i32) {
    match degrees.rem_euclid(360) {
        0 => (x, y),
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => unimplemented!(),
    }
}

fn part1(input: &str) -> i32 {
    let actions = parse(input);

    let mut pos = (0i32, 0i32);
    let mut heading = (1i32, 0i32);

    for action in &actions {
        match action {
            Action::N(v) => pos.1 += v,
            Action::S(v) => pos.1 -= v,
            Action::E(v) => pos.0 += v,
            Action::W(v) => pos.0 -= v,
            Action::L(v) => heading = rotate(heading, *v),
            Action::R(v) => heading = rotate(heading, -v),
            Action::F(v) => {
                pos.0 += heading.0 * v;
                pos.1 += heading.1 * v;
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn part2(input: &str) -> i32 {
    let actions = parse(input);

    let mut pos = (0i32, 0i32);
    let mut waypoint = (10i32, 1i32);

    for action in &actions {
        match action {
            Action::N(v) => waypoint.1 += v,
            Action::S(v) => waypoint.1 -= v,
            Action::E(v) => waypoint.0 += v,
            Action::W(v) => waypoint.0 -= v,
            Action::L(v) => waypoint = rotate(waypoint, *v),
            Action::R(v) => waypoint = rotate(waypoint, -v),
            Action::F(v) => {
                pos.0 += waypoint.0 * v;
                pos.1 += waypoint.1 * v;
            }
        }
    }

    pos.0.abs() + pos.1.abs()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 286);
    }
}
