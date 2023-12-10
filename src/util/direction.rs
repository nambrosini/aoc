use crate::util::position::Position;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, EnumIter, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl From<Position> for Direction {
    fn from(pos: Position) -> Self {
        if pos == Position::new(-1, 0) {
            Direction::North
        } else if pos == Position::new(1, 0) {
            Direction::South
        } else if pos == Position::new(0, 1) {
            Direction::East
        } else if pos == Position::new(0, -1) {
            Direction::West
        } else {
            unreachable!()
        }
    }
}
