use std::collections::{HashMap, HashSet};

type Cube = (i32, i32, i32);

fn parse_directions(s: &str) -> Vec<Cube> {
    let mut dirs = vec![];
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let dir = match c {
            'e' => (1, -1, 0),
            'w' => (-1, 1, 0),
            's' => match chars.next().unwrap() {
                'e' => (0, -1, 1),
                'w' => (-1, 0, 1),
                _ => unreachable!(),
            },
            'n' => match chars.next().unwrap() {
                'e' => (1, 0, -1),
                'w' => (0, 1, -1),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        dirs.push(dir);
    }

    dirs
}

fn parse(input: &str) -> Vec<Vec<Cube>> {
    input.lines().map(parse_directions).collect()
}

fn add((x1, y1, z1): Cube, (x2, y2, z2): Cube) -> Cube {
    (x1 + x2, y1 + y2, z1 + z2)
}

const NEIGHBOR_DIRS: [Cube; 6] = [
    (1, -1, 0),
    (0, -1, 1),
    (-1, 0, 1),
    (-1, 1, 0),
    (0, 1, -1),
    (1, 0, -1),
];

fn neighbors(c: Cube) -> impl Iterator<Item = Cube> {
    NEIGHBOR_DIRS.iter().map(move |&d| add(c, d))
}

fn initial_black_tiles(tiles_directions: &[Vec<Cube>]) -> HashSet<Cube> {
    let mut black = HashSet::new();

    for directions in tiles_directions {
        let coord = directions.iter().fold((0, 0, 0), |acc, &d| add(acc, d));

        if !black.remove(&coord) {
            black.insert(coord);
        }
    }

    black
}

fn part1(input: &str) -> usize {
    initial_black_tiles(&parse(input)).len()
}

fn part2(input: &str) -> usize {
    let mut black = initial_black_tiles(&parse(input));

    for _ in 0..100 {
        let mut neighbor_counts: HashMap<Cube, usize> = HashMap::new();
        for &tile in &black {
            for n in neighbors(tile) {
                *neighbor_counts.entry(n).or_insert(0) += 1;
            }
        }

        black = neighbor_counts
            .into_iter()
            .filter(|&(tile, count)| {
                if black.contains(&tile) {
                    count == 1 || count == 2
                } else {
                    count == 2
                }
            })
            .map(|(tile, _)| tile)
            .collect();
    }

    black.len()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/24.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 2208);
    }
}
