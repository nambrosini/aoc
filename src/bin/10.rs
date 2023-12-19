use advent_of_code::util::grid::{Grid, Parse};
use advent_of_code::util::position::{v, Vec2};

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

    for dir in Vec2::DIRECTIONS {
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

    for dir in Vec2::DIRECTIONS {
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
            if !lop.contains(&v(i as i64, j as i64)) {
                grid[i][j] = '.';
            }
        }
    }

    let d1: Vec2 = v(
        lop[0].x - lop[lop.len() - 1].x,
        lop[0].y - lop[lop.len() - 1].y,
    );
    let d2: Vec2 = v(lop[1].x - lop[0].x, lop[1].y - lop[0].y);

    grid[start.x()][start.y()] = match d1 {
        Vec2::NORTH => match d2 {
            Vec2::NORTH => '|',
            Vec2::SOUTH => unreachable!(),
            Vec2::EAST => 'L',
            Vec2::WEST => 'J',
            _ => unreachable!(),
        },
        Vec2::SOUTH => match d2 {
            Vec2::NORTH => unreachable!(),
            Vec2::SOUTH => '|',
            Vec2::EAST => 'F',
            Vec2::WEST => '7',
            _ => unreachable!(),
        },
        Vec2::EAST => match d2 {
            Vec2::NORTH => 'J',
            Vec2::SOUTH => '7',
            Vec2::EAST => '-',
            Vec2::WEST => unreachable!(),
            _ => unreachable!(),
        },
        Vec2::WEST => match d2 {
            Vec2::NORTH => 'L',
            Vec2::SOUTH => 'F',
            Vec2::EAST => unreachable!(),
            Vec2::WEST => '-',
            _ => unreachable!(),
        },
        _ => unreachable!(),
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

fn travel(grid: &Grid<char>, position: &Vec2, direction: &Vec2, step: usize) -> Option<usize> {
    let new_position = *position + *direction;
    if !is_contained(grid, &new_position) {
        return None;
    }
    if grid[new_position.x()][new_position.y()] == 'S' {
        return Some(step);
    }
    let c = grid[new_position.x()][new_position.y()];
    let new_direction = match c {
        '|' | '-' => *direction,
        'L' => match *direction {
            Vec2::SOUTH => Vec2::EAST,
            Vec2::WEST => Vec2::NORTH,
            _ => return None,
        },
        'J' => match *direction {
            Vec2::SOUTH => Vec2::WEST,
            Vec2::EAST => Vec2::NORTH,
            _ => return None,
        },
        '7' => match *direction {
            Vec2::NORTH => Vec2::WEST,
            Vec2::EAST => Vec2::SOUTH,
            _ => return None,
        },
        'F' => match *direction {
            Vec2::NORTH => Vec2::EAST,
            Vec2::WEST => Vec2::SOUTH,
            _ => return None,
        },
        _ => return None,
    };
    travel(grid, &new_position, &new_direction, step + 1)
}

fn get_start(grid: &Grid<char>) -> Vec2 {
    let mut start = Vec2::new(0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if e == &'S' {
                start = Vec2::new(i as i64, j as i64);
            }
        }
    }
    start
}

fn save_loop(
    grid: &Grid<char>,
    position: &Vec2,
    direction: &Vec2,
    l: &mut Vec<Vec2>,
) -> Result<(), ()> {
    let new_position = *position + *direction;
    if !is_contained(grid, &new_position) {
        return Err(());
    }
    if grid[new_position.x()][new_position.y()] == 'S' {
        return Ok(());
    }
    let c = grid[new_position.x()][new_position.y()];
    let new_direction = match c {
        '|' | '-' => *direction,
        'L' => match *direction {
            Vec2::SOUTH => Vec2::EAST,
            Vec2::WEST => Vec2::NORTH,
            _ => return Err(()),
        },
        'J' => match *direction {
            Vec2::SOUTH => Vec2::WEST,
            Vec2::EAST => Vec2::NORTH,
            _ => return Err(()),
        },
        '7' => match *direction {
            Vec2::NORTH => Vec2::WEST,
            Vec2::EAST => Vec2::SOUTH,
            _ => return Err(()),
        },
        'F' => match *direction {
            Vec2::NORTH => Vec2::EAST,
            Vec2::WEST => Vec2::SOUTH,
            _ => return Err(()),
        },
        _ => return Err(()),
    };
    l.push(new_position);
    save_loop(grid, &new_position, &new_direction, l)
}

fn is_contained(grid: &[Vec<char>], new_position: &Vec2) -> bool {
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
