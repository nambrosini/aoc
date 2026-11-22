fn parse(input: &str) -> Vec<(i64, i64)> {
    input.lines()
         .map(|l| {
             let mut iter = l.split(',');
             let a = iter.next().unwrap().parse().unwrap();
             let b = iter.next().unwrap().parse().unwrap();

             (a, b)
         })
        .collect()
}

fn part1(input: &str) -> i64 {
    let input = parse(input);
    let mut max = 0;
    for (i, x) in input.iter().enumerate() {
        for y in input.iter().skip(i) {
            let res = (x.0 - y.0).abs() * (x.1 - y.1).abs();
            max = max.max(res);
        }
    }
    max
}

fn part2(input: &str) -> i64 {
    todo!()
}

fn main(){
    let input = std::fs::read_to_string("data/input/2025/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
