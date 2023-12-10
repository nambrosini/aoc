use std::fmt::Display;

pub type Grid<T> = Vec<Vec<T>>;

pub trait Print {
    fn print(&self);
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
}
