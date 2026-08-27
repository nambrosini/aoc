fn parse(input: &str) -> i32 {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> i32 {
    let input = parse(input);
    let root = (input as f32).sqrt();

    let mut near_root = root as i32;

    if root > root.floor() {
        if root.floor() as i32 % 2 == 0 {
            near_root += 1;
        } else {
            near_root += 2;
        }
    } else if (root - root.floor()).abs() < 0.0001 && root.floor() as i32 % 2 == 0 {
        near_root += 1;
    }

    let near_square = near_root.pow(2);

    let mut position = (near_root / 2, -near_root / 2);

    let mut diff = near_square - input;

    if diff < near_root - 1 {
        position.0 -= diff;
        diff = 0;
    } else {
        position.0 -= near_root - 1;
        diff -= near_root - 1;
    }

    if diff < near_root - 1 {
        position.1 += diff;
        diff = 0;
    } else {
        position.1 += near_root - 1;
        diff -= near_root - 1;
    }

    if diff < near_root - 1 {
        position.0 += diff;
        diff = 0;
    } else {
        position.0 += near_root - 1;
        diff -= near_root - 1;
    }

    if diff < near_root - 1 {
        position.1 -= diff;
    } else {
        position.1 -= near_root - 1;
    }

    manhattan(position)
}

fn part2(input: &str) -> i32 {
    let input = parse(input);
    let mut memory: Vec<i32> = Vec::new();

    loop {
        let value = get_next_stress_value(&memory);
        if value > input {
            return value;
        }

        memory.push(value);
    }
}

fn get_index((x, y): (i32, i32)) -> i32 {
    if x == 0 && y == 0 {
        return 1;
    }
    let root = x.abs().max(y.abs()) * 2;
    (root - 1) * (root - 1)
        + root / 2
        + if x == root / 2 && y != -root / 2 {
            y
        } else if y == root / 2 {
            root - x
        } else if x == -root / 2 {
            root * 2 - y
        } else if y == -root / 2 {
            root * 3 + x
        } else {
            unreachable!()
        }
}

fn manhattan(pos: (i32, i32)) -> i32 {
    pos.0.abs() + pos.1.abs()
}

fn get_next_stress_value(memory: &[i32]) -> i32 {
    let n = memory.len() as i32 + 1;
    if n == 1 {
        return 1;
    }
    let position = get_position(n);

    let mut result = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let index = get_index((position.0 + dx, position.1 + dy));
            if index < n {
                result += memory[index as usize - 1];
            }
        }
    }

    result
}

fn get_position(n: i32) -> (i32, i32) {
    if n == 1 {
        return (0, 0);
    }
    let root = get_closest_even_root(n);
    let position = n - (root - 1) * (root - 1) - 1;
    let position_on_side = position % root - root / 2 + 1;

    match position / root {
        0 => (root / 2, position_on_side),
        1 => (-position_on_side, root / 2),
        2 => (-root / 2, -position_on_side),
        3 => (position_on_side, -root / 2),
        _ => unreachable!(),
    }
}

fn get_closest_even_root(n: i32) -> i32 {
    let sr = ((n - 1) as f32).sqrt() as i32;
    if sr % 2 == 0 { sr } else { sr + 1 }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1"), 0);
        assert_eq!(part1("12"), 3);
        assert_eq!(part1("23"), 2);
        assert_eq!(part1("1024"), 31);
    }
}
