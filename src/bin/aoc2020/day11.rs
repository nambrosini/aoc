#[derive(Debug, Clone, Copy, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            _ => unreachable!(),
        }
    }
}

const DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn parse(input: &str) -> Vec<Vec<Seat>> {
    input
        .lines()
        .map(|l| l.chars().map(Seat::from).collect())
        .collect()
}

fn count_adjacent(seats: &[Vec<Seat>], x: usize, y: usize) -> usize {
    DIRS.iter()
        .filter(|(dx, dy)| {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            nx >= 0
                && ny >= 0
                && (nx as usize) < seats.len()
                && (ny as usize) < seats[0].len()
                && seats[nx as usize][ny as usize] == Seat::Occupied
        })
        .count()
}

fn count_visible(seats: &[Vec<Seat>], x: usize, y: usize) -> usize {
    DIRS.iter()
        .filter(|(dx, dy)| {
            let (mut nx, mut ny) = (x as i32 + dx, y as i32 + dy);
            while nx >= 0
                && ny >= 0
                && (nx as usize) < seats.len()
                && (ny as usize) < seats[0].len()
            {
                match seats[nx as usize][ny as usize] {
                    Seat::Occupied => return true,
                    Seat::Empty => return false,
                    Seat::Floor => {
                        nx += dx;
                        ny += dy;
                    }
                }
            }
            false
        })
        .count()
}

fn run(
    seats: &[Vec<Seat>],
    threshold: usize,
    count_fn: impl Fn(&[Vec<Seat>], usize, usize) -> usize,
) -> usize {
    let mut seats = seats.to_vec();

    loop {
        let mut next = seats.clone();
        let mut changed = false;

        for x in 0..seats.len() {
            for y in 0..seats[0].len() {
                let occupied = count_fn(&seats, x, y);
                next[x][y] = match seats[x][y] {
                    Seat::Occupied if occupied >= threshold => Seat::Empty,
                    Seat::Empty if occupied == 0 => Seat::Occupied,
                    s => s,
                };
                if next[x][y] != seats[x][y] {
                    changed = true;
                }
            }
        }

        seats = next;
        if !changed {
            break;
        }
    }

    seats
        .iter()
        .flatten()
        .filter(|&&s| s == Seat::Occupied)
        .count()
}

fn part1(input: &str) -> usize {
    run(&parse(input), 4, count_adjacent)
}

fn part2(input: &str) -> usize {
    run(&parse(input), 5, count_visible)
}

fn main() {
    let input = std::fs::read_to_string("data/input/2020/11.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 26);
    }
}
