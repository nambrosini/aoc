use std::collections::HashSet;
use std::fmt::Formatter;

fn parse(input: &str) -> Map {
    let mut tracks: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut carts: Vec<Cart> = vec![];

    for (i, r) in tracks.iter_mut().enumerate() {
        for (j, e) in r.iter_mut().enumerate() {
            let pos = Vec2::new(i as i32, j as i32);
            let cart = Cart::new(
                pos,
                match e {
                    '^' => {
                        *e = '|';
                        Direction::Up
                    }
                    '>' => {
                        *e = '-';
                        Direction::Right
                    }
                    '<' => {
                        *e = '-';
                        Direction::Left
                    }
                    'v' => {
                        *e = '|';
                        Direction::Down
                    }
                    _ => continue,
                },
            );
            carts.push(cart);
        }
    }

    Map::new(carts, tracks)
}

fn part1(input: &str) -> String {
    let mut map = parse(input);

    loop {
        map.simulate();
        if let Some(x) = map.did_crash() {
            return format!("{},{}", x.y, x.x);
        }
    }
}

fn part2(input: &str) -> String {
    let mut map = parse(input);

    loop {
        map.simulate();
        map.remove_crashed();

        if let Some(pos) = map.get_last_cart_pos() {
            return format!("{},{}", pos.y, pos.x);
        }
    }
}

#[derive(Clone)]
struct Map {
    carts: Vec<Cart>,
    tracks: Vec<Vec<char>>,
}

impl Map {
    fn new(carts: Vec<Cart>, tracks: Vec<Vec<char>>) -> Self {
        Self { carts, tracks }
    }

    fn simulate(&mut self) {
        for cart in self.carts.iter_mut() {
            let pos = cart.pos + cart.dir.clone().into();
            let track = self.tracks[pos.x as usize][pos.y as usize];
            if track == '+' {
                cart.dir = cart.dir.turn(&cart.next_turn);
                cart.change_turn();
            } else if track == '/' {
                cart.dir = match cart.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
            } else if track == '\\' {
                cart.dir = match cart.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
            }
            cart.pos = pos;
        }
    }

    fn did_crash(&self) -> Option<Vec2<i32>> {
        for (i, cart) in self.carts.iter().enumerate() {
            if self
                .carts
                .iter()
                .enumerate()
                .any(|(j, c)| c.pos == cart.pos && i != j)
            {
                return Some(cart.pos);
            }
        }

        None
    }

    fn remove_crashed(&mut self) {
        let mut pos_to_remove: HashSet<Vec2<i32>> = HashSet::new();
        for (i, cart) in self.carts.iter().enumerate() {
            if self
                .carts
                .iter()
                .enumerate()
                .any(|(j, c)| c.pos == cart.pos && i != j)
            {
                pos_to_remove.insert(cart.pos);
            }
        }
        self.carts.retain(|c| !pos_to_remove.contains(&c.pos));
    }

    fn get_last_cart_pos(&self) -> Option<Vec2<i32>> {
        if self.carts.len() == 1 {
            return Some(self.carts[0].pos);
        }
        None
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, r) in self.tracks.iter().enumerate() {
            for (j, e) in r.iter().enumerate() {
                let c = if let Some(cart) = self
                    .carts
                    .iter()
                    .find(|c| c.pos.x == i as i32 && c.pos.y == j as i32)
                {
                    match cart.dir {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                } else {
                    *e
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

#[derive(Clone)]
struct Cart {
    pos: Vec2<i32>,
    dir: Direction,
    next_turn: Direction,
}

impl Cart {
    fn new(pos: Vec2<i32>, dir: Direction) -> Self {
        Self {
            pos,
            dir,
            next_turn: Direction::Left,
        }
    }

    fn change_turn(&mut self) {
        self.next_turn = match self.next_turn {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Left,
            _ => unreachable!(),
        };
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: std::ops::AddAssign> std::ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<Direction> for Vec2<i32> {
    fn from(s: Direction) -> Self {
        match s {
            Direction::Up => Vec2::new(-1, 0),
            Direction::Down => Vec2::new(1, 0),
            Direction::Left => Vec2::new(0, -1),
            Direction::Right => Vec2::new(0, 1),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Direction::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Direction::Up => self.clone(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input/2018/13.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let s = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
        assert_eq!("7,3", &part1(s));
    }

    #[test]
    fn test_part2() {
        let s = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";
        assert_eq!("6,4", &part2(s));
    }
}
