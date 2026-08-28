use adventofcode::util::{
    grid::{Grid, Write, astar},
    position::Vec2,
};

fn parse(input: &str) -> Vec<Vec2> {
    input
        .trim()
        .lines()
        .map(|l| {
            let s: Vec<i64> = l.split(',').map(|x| x.parse().unwrap()).collect();
            Vec2::new(s[1], s[0])
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let bytes = parse(input);
    let mut grid = Grid::create_empty(71, 71, '.');

    (0..1024).for_each(|i| {
        grid.set(&bytes[i], '#');
    });

    astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#'))
        .unwrap()
        .len()
}

fn part2(input: &str) -> String {
    let bytes = parse(input);
    let mut grid = Grid::create_empty(71, 71, '.');

    (0..1024).for_each(|i| {
        grid.set(&bytes[i], '#');
    });

    let mut last_path: Vec<Vec2> =
        astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#')).unwrap();
    for b in bytes.iter().skip(1024) {
        grid.set(b, '#');
        if !last_path.contains(b) {
            continue;
        }
        if let Some(path) = astar(&grid, Vec2::new(0, 0), Vec2::new(70, 70), Some('#')) {
            last_path = path;
        } else {
            return format!("{},{}", b.y, b.x);
        }
    }

    unreachable!("no byte ever blocks the path")
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/18.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// The example grid is 7x7 while part1/part2 hardcode the real 71x71 grid,
// so the sample input can't exercise this solution as written.
