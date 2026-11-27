use adventofcode::util::{
    grid::{astar, Find, Grid, Inbound, Parse, Read},
    position::Vec2,
};

fn part1(input: &str) -> usize {
    let grid = Grid::parse(input);

    let path = astar(&grid, grid.find(&'S'), grid.find(&'E'), Some('#')).unwrap();

    let mut save = vec![];
    for (i, p) in path.iter().enumerate() {
        let len_to_now = i + 2;
        for d in Vec2::DIRECTIONS {
            let pd = *p + d;
            if !grid.inbound(&pd) || grid.get(&pd).unwrap() != '#' {
                continue;
            }
            let new_pos = *p + d * 2;
            if !grid.inbound(&new_pos) || grid.get(&new_pos).unwrap() == '#' {
                continue;
            }
            let cheat_start_i = path.iter().position(|x| x == &new_pos).unwrap();
            let cheat_path = &path[cheat_start_i..];

            if cheat_path.len() + len_to_now > path.len() {
                continue;
            }
            save.push(path.len() - cheat_path.len() - len_to_now);
        }
    }

    save.iter().filter(|x| **x >= 100).count()
}

fn part2(input: &str) -> usize {
    let grid = Grid::parse(input);

    let path = astar(&grid, grid.find(&'S'), grid.find(&'E'), Some('#')).unwrap();

    let mut save: Vec<usize> = Vec::new();
    for (i, p) in path.iter().enumerate() {
        for (j, p2) in path.iter().enumerate().skip(i) {
            if p.manhattan(p2) > 20 {
                continue;
            }
            let pat = i + (path.len() - j);
            if path.len() - pat - p.manhattan(p2) as usize + 1 >= 100 {
                save.push(path.len() - pat - p.manhattan(p2) as usize + 1);
            }
        }
    }

    save.iter().filter(|&&x| x >= 100).count()
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/20.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

// Both parts threshold cheats at a >= 100 saving, which the small AoC example
// never reaches, so there is no example-based test for this day.
