fn parse(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    cycle(input, 2017, 2017)
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut curr_pos = 0;
    let mut val_at_pos_one = 0;

    for i in 1..50_000_001 {
        curr_pos = (curr_pos + input) % i + 1;
        if curr_pos == 1 {
            val_at_pos_one = i;
        }
    }

    val_at_pos_one
}

fn cycle(input: usize, n: usize, pos: usize) -> usize {
    let mut v = vec![0];
    let mut current_pos = 0;

    for i in 1..=n {
        let insert_pos = (current_pos + input) % v.len() + 1;

        if insert_pos > v.len() {
            v.push(i);
        } else {
            v.insert(insert_pos, i);
        }

        current_pos = insert_pos;
    }

    v[(v.iter().position(|&x| x == pos).unwrap() + 1) % v.len()]
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("3"), 638);
    }
}
