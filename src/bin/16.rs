use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use advent_of_code::util::{
    direction::Direction,
    grid::{Contains, Grid, Parse},
    position::Vec2,
};
use itertools::Itertools;

advent_of_code::solution!(16);

struct Input {
    vals: Grid<char>,
}

fn parse(input: &str) -> Input {
    Input {
        vals: Grid::parse(input),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let vals = parse(input).vals;
    Some(evaluate(
        &vals,
        Beam {
            pos: Vec2::new(0, 0),
            dir: Direction::Right,
        },
    ))
}

pub fn part_two(input: &str) -> Option<usize> {
    let vals = parse(input).vals;

    let mut max: usize = 0;

    for i in 0..vals.len() {
        max = usize::max(
            max,
            evaluate(
                &vals,
                Beam {
                    pos: Vec2::new(i as i64, 0),
                    dir: Direction::Right,
                },
            ),
        );
        max = usize::max(
            max,
            evaluate(
                &vals,
                Beam {
                    pos: Vec2::new(i as i64, vals[0].len() as i64 - 1),
                    dir: Direction::Left,
                },
            ),
        );
    }

    for j in 0..vals[0].len() {
        max = usize::max(
            max,
            evaluate(
                &vals,
                Beam {
                    pos: Vec2::new(0, j as i64),
                    dir: Direction::Down,
                },
            ),
        );
        max = usize::max(
            max,
            evaluate(
                &vals,
                Beam {
                    pos: Vec2::new(vals.len() as i64 - 1, j as i64),
                    dir: Direction::Up,
                },
            ),
        );
    }

    Some(max)
}

fn evaluate(grid: &Grid<char>, start: Beam) -> usize {
    let mut beams: VecDeque<Beam> = VecDeque::from([start]);
    let mut visited: HashSet<Beam> = HashSet::new();
    while let Some(beam) = beams.pop_front() {
        if visited.contains(&beam) {
            continue;
        }
        if !grid.contains(&beam.pos) {
            continue;
        }
        match grid[beam.pos.x()][beam.pos.y()] {
            '.' | '#' => {
                let next_pos = beam.pos + Vec2::from(beam.dir);
                beams.push_back(Beam {
                    pos: next_pos,
                    dir: beam.dir,
                });
            }
            '/' => {
                let dir = match beam.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                let next_pos = beam.pos + Vec2::from(dir);
                beams.push_back(Beam { pos: next_pos, dir });
            }
            '\\' => {
                let dir = match beam.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                let next_pos = beam.pos + Vec2::from(dir);
                beams.push_back(Beam { pos: next_pos, dir });
            }
            '|' => match beam.dir {
                Direction::Up | Direction::Down => {
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(beam.dir),
                        dir: beam.dir,
                    });
                }
                Direction::Left | Direction::Right => {
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(Direction::Up),
                        dir: Direction::Up,
                    });
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(Direction::Down),
                        dir: Direction::Down,
                    });
                }
            },
            '-' => match beam.dir {
                Direction::Left | Direction::Right => {
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(beam.dir),
                        dir: beam.dir,
                    });
                }
                Direction::Up | Direction::Down => {
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(Direction::Right),
                        dir: Direction::Right,
                    });
                    beams.push_back(Beam {
                        pos: beam.pos + Vec2::from(Direction::Left),
                        dir: Direction::Left,
                    });
                }
            },
            _ => unreachable!(),
        }
        visited.insert(beam);
    }
    visited.iter().map(|b| b.pos).unique().count()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Beam {
    pos: Vec2,
    dir: Direction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
