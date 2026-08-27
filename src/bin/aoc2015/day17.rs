use itertools::Itertools;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect_vec()
}

fn get_combinations(input: &[usize]) -> Vec<Vec<&usize>> {
    let mut v: Vec<Vec<Vec<&usize>>> = vec![];
    for i in 1..input.len() {
        v.push(
            input
                .iter()
                .combinations(i)
                .filter(|x| x.iter().copied().sum::<usize>() == 150)
                .collect_vec(),
        );
    }

    v.iter().flatten().cloned().collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    let v = get_combinations(&input);
    v.len()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let v = get_combinations(&input);
    let min = v.iter().map(|x| x.len()).min().unwrap();
    v.iter().filter(|x| x.len() == min).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2015/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
