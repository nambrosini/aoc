fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i128, n: i128) -> i128 {
    let (_, x, _) = egcd(x, n);
    (x % n + n) % n
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let (id, wait) = buses
        .iter()
        .map(|&id| (id, id - (earliest % id)))
        .min_by_key(|&(_, wait)| wait)
        .unwrap();

    id * wait
}

fn part2(input: &str) -> i128 {
    let constraints: Vec<(i128, i128)> = input
        .lines()
        .last()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| {
            let n = x.parse::<i128>().unwrap();
            ((n - (i as i128 % n)) % n, n)
        })
        .collect();

    let n: i128 = constraints.iter().map(|(_, n)| n).product();

    let x: i128 = constraints
        .iter()
        .map(|(a, ni)| {
            let m = n / ni;
            a * mod_inv(m, *ni) * m
        })
        .sum();

    x % n
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = "939
7,13,x,x,59,x,31,19";
        assert_eq!(part1(s), 295);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("939\n17,x,13,19"), 3417);
        assert_eq!(part2("939\n67,7,59,61"), 754018);
    }
}
