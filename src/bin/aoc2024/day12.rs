use adventofcode::util::{
    grid::{Grid, Inbound, Parse, ToMap},
    position::Vec2,
};

fn parse(input: &str) -> Vec<Vec<Vec2>> {
    let grid = Grid::parse(input);
    let map = grid.to_map();
    let mut gardens: Vec<Vec<Vec2>> = vec![];
    let mut visited: Vec<Vec2> = vec![];

    for (pos, e) in map.iter() {
        let mut group = vec![];
        if visited.contains(pos) {
            continue;
        }
        recurse(&grid, *pos, e, &mut group);
        gardens.push(group.clone());
        visited.append(&mut group);
    }
    gardens
}

fn part1(input: &str) -> usize {
    let gardens = parse(input);
    let mut sum = 0;

    for garden in gardens {
        let mut perimeter = 0;
        for plot in garden.iter() {
            let mut adjacent = 0;
            for dir in Vec2::DIRECTIONS {
                let new = *plot + dir;
                if garden.contains(&new) {
                    adjacent += 1;
                }
            }
            perimeter += 4 - adjacent;
        }
        sum += garden.len() * perimeter;
    }

    sum
}

fn part2(input: &str) -> usize {
    let gardens = parse(input);
    let mut sum = 0;

    for garden in gardens {
        let corners = count_corners(&garden);
        sum += garden.len() * corners;
    }

    sum
}

fn count_corners(garden: &[Vec2]) -> usize {
    let mut sum = 0;

    for plot in garden {
        sum += (0..4)
            .map(|d| (Vec2::DIRECTIONS[d], Vec2::DIRECTIONS[(d + 1) % 4]))
            .map(|(d1, d2)| {
                (
                    garden.contains(plot),
                    garden.contains(&(*plot + d1)),
                    garden.contains(&(*plot + d2)),
                    garden.contains(&(*plot + d1 + d2)),
                )
            })
            .filter(|(plant, left, right, mid)| {
                (left != plant && right != plant)
                    || (left == plant && right == plant && mid != plant)
            })
            .count();
    }

    sum
}

fn recurse(grid: &Grid<char>, start: Vec2, symbol: &char, visited: &mut Vec<Vec2>) {
    if &grid[start.x()][start.y()] == symbol {
        visited.push(start);
    } else {
        return;
    }

    for dir in Vec2::DIRECTIONS {
        let new = start + dir;
        if !grid.inbound(&new) || visited.contains(&new) {
            continue;
        }
        recurse(grid, new, symbol, visited);
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/12.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1206);
    }
}
