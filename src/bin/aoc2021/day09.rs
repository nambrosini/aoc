fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn neighbours(map: &[Vec<u32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as i32, y as i32);

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= map.len() as i32 || ny >= map[0].len() as i32 {
                None
            } else {
                Some((nx as usize, ny as usize))
            }
        })
        .collect()
}

fn is_low_point(map: &[Vec<u32>], x: usize, y: usize) -> bool {
    neighbours(map, x, y)
        .iter()
        .all(|&(nx, ny)| map[x][y] < map[nx][ny])
}

fn part1(input: &str) -> u32 {
    let map = parse(input);

    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_low_point(&map, i, j) {
                sum += map[i][j] + 1;
            }
        }
    }

    sum
}

fn find_basin(map: &mut [Vec<u32>], x: usize, y: usize) -> usize {
    let mut open = vec![(x, y)];
    let mut visited = vec![];

    while let Some(node) = open.pop() {
        if map[node.0][node.1] == 9 {
            continue;
        }

        visited.push(node);
        map[node.0][node.1] = 9;

        for n in neighbours(map, node.0, node.1) {
            if map[n.0][n.1] != 9 && !visited.contains(&n) && !open.contains(&n) {
                open.push(n);
            }
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut map = parse(input);

    let mut sizes = vec![];
    for j in 0..map[0].len() {
        for i in 0..map.len() {
            if map[i][j] != 9 {
                sizes.push(find_basin(&mut map, i, j));
            }
        }
    }

    sizes.sort_unstable();
    sizes.iter().rev().take(3).product()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2021/09.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1134);
    }
}
