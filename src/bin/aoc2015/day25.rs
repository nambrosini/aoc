fn parse(_input: &str) -> (usize, usize) {
    (2978, 3083)
}

fn part1(input: &str) -> i128 {
    let target = parse(input);
    let mut previous: i128 = 20151125;
    let mut index = (1, 1);

    while index != target {
        index = get_next_diagonal(index);

        previous = (previous * 252533) % 33554393;
    }

    previous
}

fn get_next_diagonal(index: (usize, usize)) -> (usize, usize) {
    if index.0 == 1 {
        (index.1 + 1, 1)
    } else {
        (index.0 - 1, index.1 + 1)
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/25.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: 🎄 (free star)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut previous: i128 = 20151125;
        let mut index = (1, 1);
        let target = (6, 6);

        while index != target {
            index = get_next_diagonal(index);
            previous = (previous * 252533) % 33554393;
        }

        assert_eq!(previous, 27995004);
    }
}
