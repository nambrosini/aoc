use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input.iter().filter(|x| is_triangle(x)).count()
}

fn part2(input: &str) -> usize {
    let input = parse(input);
    let mut sum = 0;

    for i in 0..3 {
        let vec: Vec<usize> = input
            .iter()
            .flatten()
            .skip(i)
            .enumerate()
            .filter(|(i, _)| i % 3 == 0)
            .map(|(_, &e)| e)
            .collect_vec();

        sum += vec.chunks(3).filter(|x| is_triangle(x)).count()
    }

    sum
}

fn is_triangle(triangle: &[usize]) -> bool {
    let mut triangle = triangle.to_vec();

    triangle.sort_unstable();

    triangle[0] + triangle[1] > triangle[2]
}

fn main() {
    let input = std::fs::read_to_string("data/input/2016/03.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
