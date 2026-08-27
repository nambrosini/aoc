use std::collections::{HashMap, HashSet};

use pathfinding::prelude::Matrix;

#[derive(Debug)]
struct Tile {
    data: Matrix<bool>,
    index: usize,
    nesw: [u16; 4],
}

impl Tile {
    fn new(data: Matrix<bool>, index: usize) -> Tile {
        let nesw = (0..4)
            .map(|i| {
                let r = data.rotated_ccw(i).iter().next().unwrap().to_vec();
                bools_to_u16(&r)
            })
            .collect::<Vec<_>>();
        Tile {
            data,
            index,
            nesw: [nesw[0], nesw[1], nesw[2], nesw[3]],
        }
    }

    fn rotated_cw(&self) -> Tile {
        Tile {
            data: self.data.rotated_cw(1),
            index: self.index,
            nesw: [self.nesw[3], self.nesw[0], self.nesw[1], self.nesw[2]],
        }
    }

    fn flipped(&self) -> Tile {
        Tile {
            data: self.data.flipped_lr(),
            index: self.index,
            nesw: [
                self.matching(0),
                self.matching(3),
                self.matching(2),
                self.matching(1),
            ],
        }
    }

    fn matching(&self, side: usize) -> u16 {
        let d = self.data.rows;
        let s = self.nesw[side];
        (0..d).fold(0, |a, b| (a << 1) | (((s & (1 << b)) != 0) as u16))
    }

    fn collection(self) -> Vec<Self> {
        let r1 = self.rotated_cw();
        let r2 = r1.rotated_cw();
        let r3 = r2.rotated_cw();
        let mut r = vec![self, r1, r2, r3];
        for i in 0..4 {
            r.push(r[i].flipped());
        }
        r
    }
}

fn bools_to_u16(bools: &[bool]) -> u16 {
    bools.iter().fold(0, |a, &b| (a << 1) | (b as u16))
}

fn parse(input: &str) -> Vec<Tile> {
    let mut tiles = vec![];

    for block in input.trim().split("\n\n") {
        let mut lines = block.lines();
        let header = lines.next().unwrap();
        let index = header[5..header.len() - 1].parse().unwrap();
        let data = Matrix::from_rows(lines.map(|l| l.chars().map(|c| c == '#'))).unwrap();
        tiles.extend(Tile::new(data, index).collection());
    }

    tiles
}

fn corners(tiles: &[Tile]) -> Vec<usize> {
    let mut matching: HashMap<u16, HashSet<usize>> = HashMap::new();
    for t in tiles {
        matching.entry(t.nesw[0]).or_default().insert(t.index);
    }

    let mut counter: HashMap<usize, usize> = HashMap::new();
    matching
        .into_values()
        .filter(|v| v.len() == 1)
        .for_each(|v| *counter.entry(*v.iter().next().unwrap()).or_insert(0) += 1);

    counter
        .into_iter()
        .filter(|&(_, count)| count == 4)
        .map(|(index, _)| index)
        .collect()
}

fn part1(input: &str) -> usize {
    let tiles = parse(input);

    corners(&tiles).into_iter().product()
}

fn part2(input: &str) -> usize {
    let tiles = parse(input);
    let corners = corners(&tiles);
    let topleft = corners[0];

    let topleft_index = tiles
        .iter()
        .enumerate()
        .find_map(|(i, t)| {
            if t.index == topleft {
                let [n, _, _, w] = t.nesw;
                if tiles
                    .iter()
                    .all(|t| t.index == topleft || (t.nesw[0] != n && t.nesw[0] != w))
                {
                    Some(i)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    let mut placed = Matrix::square_from_vec(vec![0; tiles.len() / 8]).unwrap();
    placed[(0, 0)] = topleft_index;
    let mut seen = HashSet::new();
    seen.insert(topleft);

    for col in 1..placed.columns {
        let east_match = tiles[placed[(0, col - 1)]].matching(1);
        let next = tiles
            .iter()
            .enumerate()
            .find(|(_, t)| !seen.contains(&t.index) && t.nesw[3] == east_match)
            .unwrap()
            .0;
        placed[(0, col)] = next;
        seen.insert(tiles[next].index);
    }

    for row in 1..placed.rows {
        for col in 0..placed.columns {
            let south_match = tiles[placed[(row - 1, col)]].matching(2);
            let next = tiles
                .iter()
                .enumerate()
                .find(|(_, t)| !seen.contains(&t.index) && t.nesw[0] == south_match)
                .unwrap()
                .0;
            placed[(row, col)] = next;
            seen.insert(tiles[next].index);
        }
    }

    let tile_side = tiles[0].data.rows;
    let mut assembled = Matrix::new_square(placed.rows * (tile_side - 2), false);
    for row in 0..placed.rows {
        for col in 0..placed.columns {
            assembled.set_slice(
                (row * (tile_side - 2), col * (tile_side - 2)),
                &tiles[placed[(row, col)]]
                    .data
                    .slice(1..tile_side - 1, 1..tile_side - 1)
                    .unwrap(),
            );
        }
    }

    let mut all_assembled = vec![
        assembled.clone(),
        assembled.rotated_cw(1),
        assembled.rotated_cw(2),
        assembled.rotated_cw(3),
    ];
    all_assembled.extend(all_assembled.clone().iter().map(|m| m.flipped_lr()));

    let monster_offsets: Vec<(usize, usize)> = r"
                  #
#    ##    ##    ###
 #  #  #  #  #  #
"
    .lines()
    .skip(1)
    .enumerate()
    .flat_map(|(r, l)| {
        l.chars()
            .enumerate()
            .filter_map(move |(c, ch)| if ch == '#' { Some((r, c)) } else { None })
    })
    .collect();

    let monsters: usize = all_assembled
        .iter()
        .map(|t| {
            (0..t.rows - 2)
                .map(|r| {
                    (0..t.columns - 19)
                        .filter(|c| monster_offsets.iter().all(|&(or, oc)| t[(r + or, c + oc)]))
                        .count()
                })
                .sum::<usize>()
        })
        .sum();

    all_assembled[0].values().filter(|&&b| b).count() - monsters * monster_offsets.len()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/20.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
