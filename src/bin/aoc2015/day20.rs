fn parse(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    let mut e = 2;

    while e < boundary {
        for h in (e..boundary).step_by(e) {
            if h >= boundary {
                break;
            }

            houses[h] += e * 10;
            if houses[h] >= input {
                boundary = h;
            }
        }

        e += 1;
    }

    houses
        .iter()
        .enumerate()
        .find(|&(_, h)| h >= &input)
        .unwrap()
        .0
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut houses = vec![10; input / 10];
    let mut boundary = houses.len() - 1;

    let mut e = 2;

    while e < boundary {
        for (i, h) in (e..boundary).step_by(e).enumerate() {
            if i >= 50 || h >= boundary {
                break;
            }

            houses[h] += e * 11;
            if houses[h] >= input {
                boundary = h;
            }
        }
        e += 1;
    }

    houses
        .iter()
        .enumerate()
        .find(|&(_, h)| h >= &input)
        .unwrap()
        .0
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/20.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
