use adventofcode::util::knot_hash::hex_hash;

fn parse(input: &str) -> Vec<Vec<u8>> {
    let input = input.trim();

    (0..128)
        .map(|row| {
            let hash = hex_hash(&format!("{}-{}", input, row));

            hash.chars()
                .flat_map(|c| {
                    let nibble = c.to_digit(16).unwrap();
                    (0..4).rev().map(move |bit| ((nibble >> bit) & 1) as u8)
                })
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input.iter().flatten().filter(|&&x| x == 1).count()
}

fn part2(input: &str) -> usize {
    let mut input = parse(input);
    count_groups(&mut input)
}

fn count_groups(grid: &mut [Vec<u8>]) -> usize {
    let mut nb_groups: usize = 0;
    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] == 1 {
                recursive_check_neighbors(grid, i, j);
                nb_groups += 1;
            }
        }
    }
    nb_groups
}

fn recursive_check_neighbors(grid: &mut [Vec<u8>], x: usize, y: usize) {
    grid[x][y] = 0;
    if x <= 126 && grid[x + 1][y] == 1 {
        recursive_check_neighbors(grid, x + 1, y);
    }
    if y <= 126 && grid[x][y + 1] == 1 {
        recursive_check_neighbors(grid, x, y + 1);
    }
    if x >= 1 && grid[x - 1][y] == 1 {
        recursive_check_neighbors(grid, x - 1, y);
    }
    if y >= 1 && grid[x][y - 1] == 1 {
        recursive_check_neighbors(grid, x, y - 1);
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2017/14.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("flqrgnkx"), 8108);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("flqrgnkx"), 1242);
    }
}
