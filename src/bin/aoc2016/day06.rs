use itertools::Itertools;

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect_vec()
}

fn part1(input: &str) -> String {
    let input = parse(input);
    decode(&input, |groups| {
        groups.max_by_key(|&(_, count)| count).unwrap().0
    })
}

fn part2(input: &str) -> String {
    let input = parse(input);
    decode(&input, |groups| {
        groups.min_by_key(|&(_, count)| count).unwrap().0
    })
}

fn decode(
    input: &[String],
    pick: impl Fn(Box<dyn Iterator<Item = (char, usize)>>) -> char,
) -> String {
    let mut s = String::new();

    for i in 0..input[0].len() {
        let groups = input
            .iter()
            .flat_map(|l| l.chars().nth(i))
            .sorted()
            .chunk_by(|&x| x)
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .collect_vec();

        s.push(pick(Box::new(groups.into_iter())));
    }

    s
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/06.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_part1() {
        assert_eq!(&part1(SAMPLE), "easter");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(SAMPLE), "advent");
    }
}
