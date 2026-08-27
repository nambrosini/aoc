use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbours(map: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);
    let mut result = Vec::new();

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && nx < map.len() as i32 && ny < map[0].len() as i32 {
                result.push((nx as usize, ny as usize));
            }
        }
    }

    result
}

fn step(map: &mut [Vec<u32>]) -> usize {
    let mut queue = VecDeque::new();

    for (x, row) in map.iter_mut().enumerate() {
        for (y, e) in row.iter_mut().enumerate() {
            *e += 1;
            if *e == 10 {
                queue.push_back((x, y));
            }
        }
    }

    let mut flashes = 0;
    while let Some((x, y)) = queue.pop_front() {
        flashes += 1;

        for (nx, ny) in neighbours(map, x, y) {
            if map[nx][ny] > 9 {
                continue;
            }
            map[nx][ny] += 1;
            if map[nx][ny] == 10 {
                queue.push_back((nx, ny));
            }
        }
    }

    for row in map.iter_mut() {
        for e in row.iter_mut() {
            if *e > 9 {
                *e = 0;
            }
        }
    }

    flashes
}

fn part1(input: &str) -> usize {
    let mut map = parse(input);

    (0..100).map(|_| step(&mut map)).sum()
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);
    let total = map.len() * map[0].len();

    (1..).find(|_| step(&mut map) == total).unwrap()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 195);
    }
}
