use crate::util::position::Vec2;
use std::fmt::Display;

pub type Grid<T> = Vec<Vec<T>>;

pub trait Print {
    fn print(&self);
    fn string(&self) -> String;
}

pub trait Contains<T> {
    fn contains(&self, point: T) -> bool;
}

pub trait Parse {
    fn parse(input: &str) -> Self;
}

impl<T> Print for Grid<T>
where
    T: Display,
{
    fn print(&self) {
        for row in self {
            for e in row {
                print!("{}", e);
            }
            println!()
        }
        println!()
    }

    fn string(&self) -> String {
        let mut res = String::new();
        for row in self {
            for e in row {
                res.push_str(&e.to_string());
            }
            res.push('\n');
        }
        res.to_string()
    }
}

impl<T> Contains<&Vec2> for Grid<T> {
    fn contains(&self, pos: &Vec2) -> bool {
        pos.x >= 0 && pos.x < self.len() as i64 && pos.y >= 0 && pos.y < self[0].len() as i64
    }
}

impl<T: From<char>> Parse for Grid<T> {
    fn parse(input: &str) -> Self {
        input
            .lines()
            .map(|c| c.chars().map(|c| c.into()).collect())
            .collect()
    }
}
