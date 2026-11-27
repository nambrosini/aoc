use adventofcode::util::{
    direction::{Direction, Rotation},
    grid::{Find, Grid, Read, Write},
    intcode::{parse, Intcode, Step},
};

fn part1(input: &str) -> i64 {
    let mem = parse(input);
    let mut pc = Intcode::new(&mem);
    let mut grid = String::new();

    loop {
        match pc.run() {
            Step::Output(v) => grid.push(v as u8 as char),
            Step::Halted => break,
            Step::NeedInput => unreachable!(),
        }
    }

    let map: Vec<Vec<char>> = grid
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let mut sum = 0;

    for x in 1..map.len() - 1 {
        for y in 1..map[x].len() - 1 {
            if map[x][y] == '#'
                && map[x - 1][y] == '#'
                && map[x + 1][y] == '#'
                && map[x][y - 1] == '#'
                && map[x][y + 1] == '#'
            {
                sum += (x * y) as i64;
            }
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mem = parse(input);
    let mut pc = Intcode::new(&mem);
    let mut grid = String::new();

    loop {
        match pc.run() {
            Step::Output(v) => grid.push(v as u8 as char),
            Step::Halted => break,
            Step::NeedInput => unreachable!(),
        }
    }

    let grid: Grid<char> = Grid::parse_from_chars(&grid);

    let path = path_from_camera(&grid);

    0
}

fn path_from_camera(grid: &Grid<char>) -> String {
    println!("{grid}");
    let mut new_grid = grid.clone();
    let mut current = grid.find(&'^');
    let mut dir = Direction::Up;
    let mut path = String::new();
    let mut steps = 0;

    'outer: loop {
        let next_pos = current + dir;
        if let Some(p) = grid.get(&next_pos) {
            if p == '#' {
                steps += 1;
                current = next_pos;
                new_grid.set(&current, dir.char());
                continue;
            }
        }

        for rotation in [Rotation::Right, Rotation::Left] {
            let new_dir = dir.rotate(&rotation);
            let new_pos = current + new_dir;

            if let Some(p) = grid.get(&new_pos) {
                if p == '#' {
                    if steps != 0 {
                        path.push_str(&steps.to_string());
                        path.push(',');
                    }
                    path.push(if rotation == Rotation::Right {
                        'R'
                    } else {
                        'L'
                    });
                    path.push(',');
                    dir = new_dir;
                    steps = 0;
                    continue 'outer;
                }
            }
        }

        path.push_str(&steps.to_string());
        println!("{new_grid}");
        return path;
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2019/17.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::path_from_camera;

    #[test]
    fn test_path_from_camera() {
        let input = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......";
        let grid = adventofcode::util::grid::Grid::parse_from_chars(input);

        let result = path_from_camera(&grid);

        assert_eq!(
            "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
            &result
        );
    }
}
