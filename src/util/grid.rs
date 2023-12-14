use std::fmt::Display;

pub type Grid<T> = Vec<Vec<T>>;

pub trait Print {
    fn print(&self);
    fn string(&self) -> String;
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
    }

    fn string(&self) -> String {
        let mut res = String::new();
        for row in self {
            for e in row {
                res.push_str(&e.to_string());
            }
            res.push('\n');
        }
        res[..res.len() - 1].to_string()
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
