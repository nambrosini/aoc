use ::std::convert::Into;
use advent_of_code::util::direction::Direction;
use advent_of_code::util::grid::{Grid, Parse};
use advent_of_code::util::position::Position;
use strum::IntoEnumIterator;
advent_of_code::solution!(10);

struct Input {
    grid: Grid<char>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: Grid::parse(input),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse(input).grid;
    let start = get_start(&grid);

    let mut count = usize::MAX;

    for dir in Direction::iter() {
        if let Some(steps) = travel(&grid, &start, &dir, 1) {
            count = count.min(steps);
        }
    }
    Some(count / 2)
}

pub fn part_two(input: &str) -> Option<i64> {
    let grid = parse(input).grid;
    let start = get_start(&grid);

    let mut lop = vec![];

    for dir in Direction::iter() {
        let mut l = vec![];
        if let Ok(()) = save_loop(&grid, &start, &dir, &mut l) {
            lop = l;
            break;
        }
    }
    lop.insert(0, start);

    let mut grid = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !lop.contains(&Position::new(i as i64, j as i64)) {
                grid[i][j] = '.';
            }
        }
    }

    let d1: Direction = Position::new(
        lop[0].x - lop[lop.len() - 1].x,
        lop[0].y - lop[lop.len() - 1].y,
    )
    .into();
    let d2: Direction = Position::new(lop[1].x - lop[0].x, lop[1].y - lop[0].y).into();

    grid[start.x()][start.y()] = match d1 {
        Direction::North => match d2 {
            Direction::North => '|',
            Direction::South => unreachable!(),
            Direction::East => 'L',
            Direction::West => 'J',
        },
        Direction::South => match d2 {
            Direction::North => unreachable!(),
            Direction::South => '|',
            Direction::East => 'F',
            Direction::West => '7',
        },
        Direction::East => match d2 {
            Direction::North => 'J',
            Direction::South => '7',
            Direction::East => '-',
            Direction::West => unreachable!(),
        },
        Direction::West => match d2 {
            Direction::North => 'L',
            Direction::South => 'F',
            Direction::East => unreachable!(),
            Direction::West => '-',
        },
    };

    let mut total = 0;
    for i in 0..grid.len() {
        let mut c = 0;
        let mut last_bend = ' ';
        for j in 0..grid[0].len() {
            let cell = grid[i][j];
            if cell == '.' && c % 2 == 1 && c != 0 {
                grid[i][j] = 'I';
                total += 1;
            } else {
                grid[i][j] = 'O';
            }
            if cell == '|' {
                c += 1;
            }
            if cell == 'F' || cell == 'L' {
                last_bend = cell;
            }
            if last_bend == 'F' && cell == '7' || last_bend == 'L' && cell == 'J' {
                c += 2;
            }
            if last_bend == 'F' && cell == 'J' || last_bend == 'L' && cell == '7' {
                c += 1;
            }
        }
    }

    Some(total)
}

fn travel(
    grid: &Grid<char>,
    position: &Position,
    direction: &Direction,
    step: usize,
) -> Option<usize> {
    let dir: Position = direction.into();
    let new_position = *position + dir;
    if !is_contained(grid, &new_position) {
        return None;
    }
    if grid[new_position.x()][new_position.y()] == 'S' {
        return Some(step);
    }
    let c = grid[new_position.x()][new_position.y()];
    let new_direction = match c {
        '|' | '-' => *direction,
        'L' => match direction {
            Direction::South => Direction::East,
            Direction::West => Direction::North,
            _ => return None,
        },
        'J' => match direction {
            Direction::South => Direction::West,
            Direction::East => Direction::North,
            _ => return None,
        },
        '7' => match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            _ => return None,
        },
        'F' => match direction {
            Direction::North => Direction::East,
            Direction::West => Direction::South,
            _ => return None,
        },
        _ => return None,
    };
    travel(grid, &new_position, &new_direction, step + 1)
}

fn get_start(grid: &Grid<char>) -> Position {
    let mut start = Position::new(0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if e == &'S' {
                start = Position::new(i as i64, j as i64);
            }
        }
    }
    start
}

fn save_loop(
    grid: &Grid<char>,
    position: &Position,
    direction: &Direction,
    l: &mut Vec<Position>,
) -> Result<(), ()> {
    let dir: Position = direction.into();
    let new_position = *position + dir;
    if !is_contained(grid, &new_position) {
        return Err(());
    }
    if grid[new_position.x()][new_position.y()] == 'S' {
        return Ok(());
    }
    let c = grid[new_position.x()][new_position.y()];
    let new_direction = match c {
        '|' | '-' => *direction,
        'L' => match direction {
            Direction::South => Direction::East,
            Direction::West => Direction::North,
            _ => return Err(()),
        },
        'J' => match direction {
            Direction::South => Direction::West,
            Direction::East => Direction::North,
            _ => return Err(()),
        },
        '7' => match direction {
            Direction::North => Direction::West,
            Direction::East => Direction::South,
            _ => return Err(()),
        },
        'F' => match direction {
            Direction::North => Direction::East,
            Direction::West => Direction::South,
            _ => return Err(()),
        },
        _ => return Err(()),
    };
    l.push(new_position);
    save_loop(grid, &new_position, &new_direction, l)
}

fn is_contained(grid: &[Vec<char>], new_position: &Position) -> bool {
    new_position.x >= 0
        && new_position.x() < grid.len()
        && new_position.y >= 0
        && new_position.y() < grid[0].len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_ex_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_ex_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }
}
