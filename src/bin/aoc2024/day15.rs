use std::collections::VecDeque;

use adventofcode::util::{
    direction::Direction,
    grid::{Find, Grid, Parse, Read, Write},
};

type Movements = Vec<Direction>;

const EMPTY: char = '.';
const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';

// Part 2
const OPEN_BOX: char = '[';
const CLOSE_BOX: char = ']';

fn parse(input: &str) -> (Grid<char>, Movements) {
    let mut lines = input.split("\n\n");
    let grid = Grid::parse(lines.next().unwrap());
    let movements = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        })
        .collect();
    (grid, movements)
}

fn part1(input: &str) -> i64 {
    let (mut grid, movements) = parse(input);

    for m in movements {
        move_robot(&mut grid, m);
    }

    grid.find_all(&BOX).iter().map(|b| b.x * 100 + b.y).sum()
}

fn move_robot(grid: &mut Grid<char>, dir: Direction) {
    let robot = grid.find(&ROBOT);
    let new_pos = robot + dir;
    let next = grid.get(&new_pos).unwrap();
    match next {
        EMPTY => {
            grid.set(&robot, EMPTY);
            grid.set(&new_pos, ROBOT);
        }
        WALL => (),
        BOX => {
            let mut current = new_pos;
            while grid.get(&current).unwrap() == BOX {
                current += dir;
            }

            if grid.get(&current).unwrap() == WALL {
                return;
            }
            grid.set(&robot, EMPTY);

            if grid.get(&current).unwrap() == EMPTY {
                while current != new_pos {
                    current -= dir;
                    grid.set(&(current + dir), BOX);
                }
            }
            grid.set(&new_pos, ROBOT);
        }
        _ => (),
    }
}

#[allow(dead_code)]
fn print(grid: &Grid<char>) {
    for row in grid.iter() {
        for e in row.iter() {
            print!("{}", e);
        }
        println!();
    }
}

fn part2(input: &str) -> i64 {
    let (old_grid, movements) = parse(input);

    let mut grid = Grid::new();
    for r in old_grid {
        let mut row = vec![];
        for e in r {
            match e {
                EMPTY => row.extend_from_slice(&[EMPTY, EMPTY]),
                WALL => row.extend_from_slice(&[WALL, WALL]),
                BOX => row.extend_from_slice(&['[', ']']),
                ROBOT => row.extend_from_slice(&[ROBOT, '.']),
                _ => unreachable!(),
            }
        }
        grid.push(row);
    }

    for m in movements {
        move_robot_2(&mut grid, m);
    }

    grid.find_all(&OPEN_BOX)
        .iter()
        .map(|b| b.x * 100 + b.y)
        .sum()
}

fn move_robot_2(grid: &mut Grid<char>, dir: Direction) {
    let robot = grid.find(&ROBOT);
    let new_pos = robot + dir;
    let next = grid.get(&new_pos).unwrap();
    match next {
        EMPTY => {
            grid.set(&robot, EMPTY);
            grid.set(&new_pos, ROBOT);
        }
        WALL => (),
        OPEN_BOX | CLOSE_BOX => match dir {
            Direction::Left | Direction::Right => {
                let mut current = new_pos;
                while grid.get(&current).unwrap() == OPEN_BOX
                    || grid.get(&current).unwrap() == CLOSE_BOX
                {
                    current += dir * 2;
                }

                if grid.get(&current).unwrap() == WALL {
                    return;
                }
                grid.set(&robot, EMPTY);

                if grid.get(&current).unwrap() == EMPTY {
                    while current != new_pos {
                        if dir == Direction::Left {
                            grid.set(&current, OPEN_BOX);
                            grid.set(&(current - dir), CLOSE_BOX);
                        } else {
                            grid.set(&current, CLOSE_BOX);
                            grid.set(&(current - dir), OPEN_BOX);
                        }
                        current -= dir * 2;
                    }
                }
                grid.set(&new_pos, ROBOT);
            }
            Direction::Up | Direction::Down => {
                let mut boxes = VecDeque::new();
                let mut queue = VecDeque::new();
                queue.push_back((new_pos, grid.get(&new_pos).unwrap()));
                if grid.get(&new_pos).unwrap() == OPEN_BOX {
                    queue.push_back((new_pos + Direction::Right, CLOSE_BOX));
                } else {
                    queue.push_back((new_pos + Direction::Left, OPEN_BOX));
                }
                while let Some((pos, b)) = queue.pop_front() {
                    if b == WALL {
                        return;
                    }
                    if b == EMPTY || boxes.contains(&(pos, b)) {
                        continue;
                    }
                    boxes.push_back((pos, b));
                    queue.push_back((pos + dir, grid.get(&(pos + dir)).unwrap()));

                    let right = (
                        pos + Direction::Right,
                        grid.get(&(pos + Direction::Right)).unwrap(),
                    );
                    let left = (
                        pos + Direction::Left,
                        grid.get(&(pos + Direction::Left)).unwrap(),
                    );
                    if b == OPEN_BOX && !queue.contains(&right) {
                        queue.push_back(right);
                    }
                    if b == CLOSE_BOX && !queue.contains(&left) {
                        queue.push_back(left);
                    }
                }

                for (pos, _) in boxes.iter() {
                    grid.set(pos, EMPTY);
                }
                for (pos, b) in boxes {
                    grid.set(&(pos + dir), b);
                }

                grid.set(&robot, EMPTY);
                grid.set(&(robot + dir), ROBOT);
            }
        },
        _ => unreachable!(),
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2024/15.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 9021);
    }
}
