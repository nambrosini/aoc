use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn parse(input: &str) -> Grid {
    input.into()
}

fn part1(input: &str) -> usize {
    let mut grid = parse(input);
    for _ in 0..10 {
        grid.simulate_next_gen()
    }
    grid.calc_result()
}

fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let mut history: HashMap<u64, Vec<usize>> = HashMap::new();
    let minutes = 1_000_000_000;

    for minute in 0..minutes {
        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();

        let entry = history.entry(hash).or_default();
        entry.push(minute);

        if entry.len() == 5 {
            let period = entry[4] - entry[3];
            let remaining = minutes - minute;
            for _ in 0..remaining % period {
                grid.simulate_next_gen();
            }
            return grid.calc_result();
        }

        grid.simulate_next_gen();
    }
    grid.calc_result()
}

#[derive(Clone, Hash)]
struct Grid {
    grid: Vec<Vec<Acre>>,
}

impl Grid {
    fn simulate_next_gen(&mut self) {
        let mut new_grid = self.grid.clone();

        for (i, r) in new_grid.iter_mut().enumerate() {
            for (j, e) in r.iter_mut().enumerate() {
                let next = self.get_next(i, j);
                *e = next;
            }
        }

        self.grid = new_grid;
    }

    fn get_next(&self, x: usize, y: usize) -> Acre {
        let neig = self.get_neighbours_count(x, y);
        match self.grid[x][y] {
            Acre::Open => {
                if neig.0 >= 3 {
                    Acre::Tree
                } else {
                    Acre::Open
                }
            }
            Acre::Tree => {
                if neig.1 >= 3 {
                    Acre::Lumberyard
                } else {
                    Acre::Tree
                }
            }
            Acre::Lumberyard => {
                if neig.0 >= 1 && neig.1 >= 1 {
                    Acre::Lumberyard
                } else {
                    Acre::Open
                }
            }
        }
    }

    // Returns (trees, lumberyards)
    fn get_neighbours_count(&self, x: usize, y: usize) -> (usize, usize) {
        let mut count = (0, 0);
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if !(0..self.grid.len() as i32).contains(&(x as i32 + i))
                    || !(0..self.grid.len() as i32).contains(&(y as i32 + j))
                {
                    continue;
                }

                let xi = (x as i32 + i) as usize;
                let yi = (y as i32 + j) as usize;

                if self.grid[xi][yi] == Acre::Tree {
                    count.0 += 1;
                } else if self.grid[xi][yi] == Acre::Lumberyard {
                    count.1 += 1;
                }
            }
        }

        count
    }

    fn calc_result(&self) -> usize {
        let trees_count = self
            .grid
            .iter()
            .flatten()
            .filter(|x| x == &&Acre::Tree)
            .count();
        let lumb_count = self
            .grid
            .iter()
            .flatten()
            .filter(|x| x == &&Acre::Lumberyard)
            .count();
        trees_count * lumb_count
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<Acre>> = s
            .lines()
            .map(|l| l.chars().map(Acre::from).collect())
            .collect();

        Self { grid }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Acre {
    Open = 0,
    Tree = 1,
    Lumberyard = 2,
}

impl From<char> for Acre {
    fn from(s: char) -> Self {
        match s {
            '.' => Self::Open,
            '|' => Self::Tree,
            '#' => Self::Lumberyard,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/18.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

        assert_eq!(part1(s), 1147);
    }
}
